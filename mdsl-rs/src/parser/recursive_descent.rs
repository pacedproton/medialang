//! Recursive descent parser for the MediaLanguage DSL
//!
//! This module implements a recursive descent parser that converts tokens into an Abstract Syntax Tree (AST).
//! The parser handles all MediaLanguage DSL constructs including imports, variables, units, vocabularies,
//! families, outlets, templates, data declarations, and relationships.

use super::ast::*;
use super::error::ParseError;
use crate::error::{Result, SourcePosition};
use crate::lexer::{Keyword, Token, TokenKind};

/// Recursive descent parser for MediaLanguage DSL
pub struct Parser {
    /// Tokens to parse
    tokens: Vec<Token>,
    /// Current position in token stream
    current: usize,
}

impl Parser {
    /// Create a new parser with the given tokens
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    /// Parse the tokens into an AST
    pub fn parse(&mut self) -> Result<Program> {
        let position = self.current_position();
        let mut statements = Vec::new();

        while !self.is_at_end() {
            // Skip newlines, comments, and semicolons at the top level
            if self.match_token(&TokenKind::Newline)
                || self.match_comment()
                || self.match_token(&TokenKind::Semicolon)
            {
                continue;
            }

            match self.parse_statement() {
                Ok(stmt) => statements.push(stmt),
                Err(e) => {
                    // Error recovery: skip to next statement
                    self.synchronize();
                    return Err(e);
                }
            }
        }

        Ok(Program::new(statements, position))
    }

    /// Parse a top-level statement
    fn parse_statement(&mut self) -> Result<Statement> {
        match &self.current_token().kind {
            TokenKind::Keyword(Keyword::Import) => self.parse_import().map(Statement::Import),
            TokenKind::Keyword(Keyword::Let) => self.parse_variable().map(Statement::Variable),
            TokenKind::Keyword(Keyword::Unit) => self.parse_unit().map(Statement::Unit),
            TokenKind::Keyword(Keyword::Vocabulary) => {
                self.parse_vocabulary().map(Statement::Vocabulary)
            }
            TokenKind::Keyword(Keyword::Family) => {
                self.parse_family().map(Statement::Family)
            }
            TokenKind::Keyword(Keyword::Template) => self.parse_template().map(Statement::Template),
            TokenKind::Keyword(Keyword::Data) => self.parse_data().map(Statement::Data),
            TokenKind::Keyword(Keyword::Event) => self.parse_event().map(Statement::Event),
            TokenKind::Keyword(Keyword::Catalog) => self.parse_catalog().map(Statement::Catalog),
            TokenKind::Keyword(Keyword::DiachronicLink) => self
                .parse_diachronic_link()
                .map(|link| Statement::Relationship(RelationshipDeclaration::Diachronic(link))),
            TokenKind::Keyword(Keyword::SynchronousLink) => self
                .parse_synchronous_link()
                .map(|link| Statement::Relationship(RelationshipDeclaration::Synchronous(link))),
            TokenKind::Comment(_) | TokenKind::MultiLineComment(_) => {
                self.parse_comment().map(Statement::Comment)
            }
            TokenKind::Identifier(_) => {
                // Check if this is a standalone vocabulary body (identifier followed by '{')
                if self
                    .peek_token()
                    .map_or(false, |t| t.kind == TokenKind::LeftBrace)
                {
                    self.parse_standalone_vocabulary()
                        .map(Statement::Vocabulary)
                } else {
                    Err(self.error(format!("Unexpected token: {}", self.current_token().text)))
                }
            }
            _ => Err(self.error(format!("Unexpected token: {}", self.current_token().text))),
        }
    }

    /// Parse an import statement: IMPORT "file.mdsl";
    fn parse_import(&mut self) -> Result<ImportStatement> {
        let position = self.current_position();
        self.consume_keyword(Keyword::Import, "Expected 'import'")?;

        let path = match &self.current_token().kind {
            TokenKind::String(s) => {
                let path = s.clone();
                self.advance();
                path
            }
            _ => return Err(self.error("Expected string literal after 'import'".to_string())),
        };

        self.consume_optional_semicolon();
        Ok(ImportStatement { path, position })
    }

    /// Parse a variable declaration: LET name = value;
    fn parse_variable(&mut self) -> Result<VariableDeclaration> {
        let position = self.current_position();
        self.consume_keyword(Keyword::Let, "Expected 'let'")?;

        let name = self.consume_identifier("Expected variable name")?;
        self.consume_token(TokenKind::Assign, "Expected '=' after variable name")?;
        let value = self.parse_expression()?;

        self.consume_optional_semicolon();
        Ok(VariableDeclaration {
            name,
            value,
            position,
        })
    }

    /// Parse a unit declaration: UNIT Name { ... }
    fn parse_unit(&mut self) -> Result<UnitDeclaration> {
        let position = self.current_position();
        self.consume_keyword(Keyword::Unit, "Expected 'unit'")?;

        let name = self.consume_identifier("Expected unit name")?;
        self.consume_token(TokenKind::LeftBrace, "Expected '{' after unit name")?;

        let mut fields = Vec::new();
        while !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
            // Skip newlines, comments, and trailing commas
            if self.match_token(&TokenKind::Newline) || self.match_comment() {
                continue;
            }

            // Skip trailing commas before closing brace
            if self.check(&TokenKind::Comma)
                && self
                    .peek_token()
                    .map_or(false, |t| t.kind == TokenKind::RightBrace)
            {
                self.advance(); // consume the comma
                continue;
            }

            // Skip trailing commas followed by newlines/comments
            if self.check(&TokenKind::Comma) {
                let next_token = self.peek_token();
                if let Some(token) = next_token {
                    if matches!(token.kind, TokenKind::Newline | TokenKind::RightBrace)
                        || self.is_comment_token(&token.kind)
                    {
                        self.advance(); // consume the comma
                        continue;
                    }
                }
            }

            fields.push(self.parse_field_declaration()?);

            // Handle optional comma after field
            if self.match_token(&TokenKind::Comma) {
                // Check if next token is closing brace or newline/comment
                let next_token = self.peek_token();
                if let Some(token) = next_token {
                    if token.kind == TokenKind::RightBrace || self.is_comment_token(&token.kind) {
                        // This is a trailing comma, skip it
                        continue;
                    }
                }
            }
        }

        self.consume_token(TokenKind::RightBrace, "Expected '}' after unit fields")?;
        Ok(UnitDeclaration {
            name,
            fields,
            position,
        })
    }

    /// Parse a field declaration: name: TYPE [PRIMARY KEY]
    fn parse_field_declaration(&mut self) -> Result<FieldDeclaration> {
        let position = self.current_position();
        let name = self.consume_identifier("Expected field name")?;
        self.consume_token(TokenKind::Colon, "Expected ':' after field name")?;

        let field_type = self.parse_field_type()?;
        let is_primary_key = if self.match_keywords(&[Keyword::Primary, Keyword::Key]) {
            // Consume the PRIMARY KEY tokens
            self.advance(); // consume PRIMARY
            self.advance(); // consume KEY
            true
        } else {
            false
        };

        Ok(FieldDeclaration {
            name,
            field_type,
            is_primary_key,
            position,
        })
    }

    /// Parse a field type: ID, TEXT(n), NUMBER, BOOLEAN, CATEGORY(...)
    fn parse_field_type(&mut self) -> Result<FieldType> {
        match &self.current_token().kind {
            TokenKind::Keyword(Keyword::Id) => {
                self.advance();
                Ok(FieldType::Id)
            }
            TokenKind::Keyword(Keyword::Text) => {
                self.advance();
                if self.match_token(&TokenKind::LeftParen) {
                    let length = self.consume_number("Expected length after 'TEXT('")?;
                    self.consume_token(TokenKind::RightParen, "Expected ')' after TEXT length")?;
                    Ok(FieldType::Text(Some(length as u32)))
                } else {
                    Ok(FieldType::Text(None))
                }
            }
            TokenKind::Keyword(Keyword::Number) => {
                self.advance();
                Ok(FieldType::Number)
            }
            TokenKind::Keyword(Keyword::Boolean) => {
                self.advance();
                Ok(FieldType::Boolean)
            }
            TokenKind::Keyword(Keyword::Category) => {
                self.advance();
                self.consume_token(TokenKind::LeftParen, "Expected '(' after 'CATEGORY'")?;
                let mut values = Vec::new();

                while !self.check(&TokenKind::RightParen) && !self.is_at_end() {
                    // Skip newlines and comments
                    if self.match_token(&TokenKind::Newline) || self.match_comment() {
                        continue;
                    }

                    let value = self.consume_string("Expected category value")?;
                    values.push(value);
                    if !self.match_token(&TokenKind::Comma) {
                        break;
                    }
                }

                // Skip newlines before closing parenthesis
                while self.match_token(&TokenKind::Newline) || self.match_comment() {
                    // Continue skipping
                }
                self.consume_token(TokenKind::RightParen, "Expected ')' after category values")?;
                Ok(FieldType::Category(values))
            }
            _ => Err(self.error("Expected field type".to_string())),
        }
    }

    /// Parse a vocabulary declaration
    fn parse_vocabulary(&mut self) -> Result<VocabularyDeclaration> {
        let position = self.current_position();
        self.consume_keyword(Keyword::Vocabulary, "Expected 'vocabulary'")?;

        let name = self.consume_identifier("Expected vocabulary name")?;
        self.consume_token(TokenKind::LeftBrace, "Expected '{' after vocabulary name")?;

        let mut bodies = Vec::new();
        while !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
            // Skip newlines and comments
            if self.match_token(&TokenKind::Newline) || self.match_comment() {
                continue;
            }

            bodies.push(self.parse_vocabulary_body()?);

            // Skip optional comma after vocabulary body
            if self.match_token(&TokenKind::Comma) {
                // Skip newlines and comments after comma
                while self.match_token(&TokenKind::Newline) || self.match_comment() {
                    // Continue skipping
                }
            }
        }

        // Skip newlines before closing brace
        while self.match_token(&TokenKind::Newline) || self.match_comment() {
            // Continue skipping
        }
        self.consume_token(
            TokenKind::RightBrace,
            "Expected '}' after vocabulary bodies",
        )?;
        Ok(VocabularyDeclaration {
            name,
            bodies,
            position,
        })
    }

    /// Parse standalone vocabulary body (without VOCABULARY keyword)
    fn parse_standalone_vocabulary(&mut self) -> Result<VocabularyDeclaration> {
        let position = self.current_position();
        let name = self.consume_identifier("Expected vocabulary name")?;

        // We'll create the actual body after parsing the entries

        // Parse the vocabulary body structure
        self.consume_token(TokenKind::LeftBrace, "Expected '{' after vocabulary name")?;

        let mut entries = Vec::new();
        while !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
            if self.match_token(&TokenKind::Newline) || self.match_comment() {
                continue;
            }
            entries.push(self.parse_vocabulary_entry()?);
            self.consume_optional_comma();
        }

        self.consume_token(
            TokenKind::RightBrace,
            "Expected '}' after vocabulary entries",
        )?;

        let body = VocabularyBody {
            name: name.clone(),
            entries,
            position,
        };

        Ok(VocabularyDeclaration {
            name,
            bodies: vec![body],
            position,
        })
    }

    /// Parse vocabulary body
    fn parse_vocabulary_body(&mut self) -> Result<VocabularyBody> {
        let position = self.current_position();

        // Skip newlines and comments before vocabulary body name
        while self.match_token(&TokenKind::Newline) || self.match_comment() {
            // Continue skipping
        }

        let name = self.consume_identifier("Expected vocabulary body name")?;

        self.consume_token(
            TokenKind::LeftBrace,
            "Expected '{' after vocabulary body name",
        )?;

        let mut entries = Vec::new();
        while !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
            if self.match_token(&TokenKind::Newline) || self.match_comment() {
                continue;
            }
            entries.push(self.parse_vocabulary_entry()?);
            self.consume_optional_comma();
        }

        // Skip newlines before closing brace
        while self.match_token(&TokenKind::Newline) || self.match_comment() {
            // Continue skipping
        }
        self.consume_token(
            TokenKind::RightBrace,
            "Expected '}' after vocabulary entries",
        )?;
        Ok(VocabularyBody {
            name,
            entries,
            position,
        })
    }

    /// Parse a vocabulary entry: key: "value"
    fn parse_vocabulary_entry(&mut self) -> Result<VocabularyEntry> {
        let position = self.current_position();
        let key = match &self.current_token().kind {
            TokenKind::Number(n) => {
                let n = *n;
                self.advance();
                VocabularyKey::Number(n)
            }
            TokenKind::String(s) => {
                let s = s.clone();
                self.advance();
                VocabularyKey::String(s)
            }
            _ => return Err(self.error("Expected number or string key".to_string())),
        };

        self.consume_token(TokenKind::Colon, "Expected ':' after vocabulary key")?;
        let value = self.consume_string("Expected string value")?;

        Ok(VocabularyEntry {
            key,
            value,
            position,
        })
    }

    /// Parse a family declaration
    fn parse_family(&mut self) -> Result<FamilyDeclaration> {
        let position = self.current_position();
        // Accept only 'family' keyword
        if !self.match_keyword(Keyword::Family) {
            return Err(self.error("Expected 'family'".to_string()));
        }

        let name = self.consume_string("Expected family name")?;
        self.consume_token(TokenKind::LeftBrace, "Expected '{' after family name")?;

        let mut members = Vec::new();
        while !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
            if self.match_token(&TokenKind::Newline)
                || self.match_comment()
                || self.match_token(&TokenKind::Semicolon)
            {
                continue;
            }

            match &self.current_token().kind {
                TokenKind::Keyword(Keyword::Outlet) => {
                    members.push(FamilyMember::Outlet(self.parse_outlet_declaration()?));
                }
                TokenKind::Keyword(Keyword::OutletRef) => {
                    members.push(FamilyMember::OutletReference(
                        self.parse_outlet_reference()?,
                    ));
                }
                TokenKind::Keyword(Keyword::Data) => {
                    members.push(FamilyMember::Data(self.parse_data()?));
                }
                TokenKind::Keyword(Keyword::DiachronicLink) => {
                    members.push(FamilyMember::Relationship(
                        RelationshipDeclaration::Diachronic(self.parse_diachronic_link()?),
                    ));
                }
                TokenKind::Keyword(Keyword::SynchronousLink)
                | TokenKind::Keyword(Keyword::SynchronousLinks) => {
                    members.push(FamilyMember::Relationship(
                        RelationshipDeclaration::Synchronous(self.parse_synchronous_link()?),
                    ));
                }
                TokenKind::Comment(_) | TokenKind::MultiLineComment(_) => {
                    members.push(FamilyMember::Comment(self.parse_comment()?));
                }
                TokenKind::Annotation(_) => {
                    // Parse annotation and add as comment
                    let annotation = self.parse_annotation()?;
                    members.push(FamilyMember::Comment(CommentStatement {
                        text: format!(
                            "@{}: {}",
                            annotation.name,
                            annotation.value.unwrap_or_default()
                        ),
                        is_multiline: false,
                        position: annotation.position,
                    }));
                }
                _ => return Err(self.error("Expected family member".to_string())),
            }
        }

        self.consume_token(TokenKind::RightBrace, "Expected '}' after family members")?;
        Ok(FamilyDeclaration {
            name,
            members,
            position,
        })
    }

    /// Parse template declaration
    fn parse_template(&mut self) -> Result<TemplateDeclaration> {
        let position = self.current_position();
        self.consume_keyword(Keyword::Template, "Expected 'template'")?;

        // Skip the template type (e.g., "OUTLET")
        if let TokenKind::Keyword(_) = &self.current_token().kind {
            self.advance();
        }

        let name = self.consume_string("Expected template name")?;
        self.consume_token(TokenKind::LeftBrace, "Expected '{' after template name")?;

        let blocks = self.parse_outlet_blocks()?;

        self.consume_token(TokenKind::RightBrace, "Expected '}' after template blocks")?;
        Ok(TemplateDeclaration {
            name,
            blocks,
            position,
        })
    }

    /// Parse outlet declaration
    fn parse_outlet_declaration(&mut self) -> Result<OutletDeclaration> {
        let position = self.current_position();
        self.consume_keyword(Keyword::Outlet, "Expected 'outlet'")?;

        let name = self.consume_string("Expected outlet name")?;

        // Parse inheritance clause (EXTENDS TEMPLATE "name" or BASED_ON id)
        let inheritance = if self.match_keyword(Keyword::Extends) {
            self.consume_keyword(Keyword::Template, "Expected 'template' after 'extends'")?;
            let template_name = self.consume_string("Expected template name")?;
            Some(InheritanceClause::ExtendsTemplate(template_name))
        } else if self.match_keyword(Keyword::BasedOn) {
            let id = self.consume_number("Expected ID after 'based_on'")?;
            Some(InheritanceClause::BasedOn(id))
        } else {
            None
        };

        self.consume_token(
            TokenKind::LeftBrace,
            "Expected '{' after outlet declaration",
        )?;

        let blocks = self.parse_outlet_blocks()?;

        self.consume_token(TokenKind::RightBrace, "Expected '}' after outlet blocks")?;
        Ok(OutletDeclaration {
            name,
            inheritance,
            blocks,
            position,
        })
    }

    /// Parse outlet blocks (identity, lifecycle, characteristics, metadata)
    fn parse_outlet_blocks(&mut self) -> Result<Vec<OutletBlock>> {
        let mut blocks = Vec::new();

        while !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
            if self.match_token(&TokenKind::Newline)
                || self.match_comment()
                || self.match_token(&TokenKind::Semicolon)
            {
                continue;
            }

            match &self.current_token().kind {
                TokenKind::Keyword(Keyword::Identity) => {
                    blocks.push(OutletBlock::Identity(self.parse_identity_block()?));
                }
                TokenKind::Keyword(Keyword::Lifecycle) => {
                    blocks.push(OutletBlock::Lifecycle(self.parse_lifecycle_block()?));
                }
                TokenKind::Keyword(Keyword::Characteristics) => {
                    blocks.push(OutletBlock::Characteristics(
                        self.parse_characteristics_block()?,
                    ));
                }
                TokenKind::Keyword(Keyword::Metadata) => {
                    blocks.push(OutletBlock::Metadata(self.parse_metadata_block()?));
                }
                TokenKind::Comment(_) | TokenKind::MultiLineComment(_) => {
                    blocks.push(OutletBlock::Comment(self.parse_comment()?));
                }
                TokenKind::Identifier(_) | TokenKind::Keyword(Keyword::Id) => {
                    // Handle field assignments like "id = 200001;"
                    let field = self.parse_identity_field()?;
                    blocks.push(OutletBlock::Identity(IdentityBlock {
                        fields: vec![field],
                        position: self.current_position(),
                    }));
                }
                _ => return Err(self.error("Expected outlet block".to_string())),
            }
        }

        Ok(blocks)
    }

    /// Parse identity block
    fn parse_identity_block(&mut self) -> Result<IdentityBlock> {
        let position = self.current_position();
        self.consume_keyword(Keyword::Identity, "Expected 'identity'")?;
        self.consume_token(TokenKind::LeftBrace, "Expected '{' after 'identity'")?;

        let mut fields = Vec::new();
        while !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
            if self.match_token(&TokenKind::Newline) || self.match_comment() {
                continue;
            }
            fields.push(self.parse_identity_field()?);
            // Consume optional semicolon or comma
            self.consume_optional_semicolon();
            self.consume_optional_comma();
        }

        self.consume_token(TokenKind::RightBrace, "Expected '}' after identity fields")?;
        Ok(IdentityBlock { fields, position })
    }

    /// Parse identity field
    fn parse_identity_field(&mut self) -> Result<IdentityField> {
        let position = self.current_position();
        let name = self.consume_identifier("Expected field name")?;
        self.consume_token(TokenKind::Assign, "Expected '=' after field name")?;

        // Check if this is an array assignment
        if self.check(&TokenKind::LeftBracket) {
            self.advance(); // consume '['
            let mut values = Vec::new();

            while !self.check(&TokenKind::RightBracket) && !self.is_at_end() {
                // Skip newlines and comments
                if self.match_token(&TokenKind::Newline) || self.match_comment() {
                    continue;
                }

                values.push(self.parse_object_literal()?);

                // Skip newlines and comments after object literal
                while self.match_token(&TokenKind::Newline) || self.match_comment() {
                    // continue
                }

                if !self.match_token(&TokenKind::Comma) {
                    break;
                }
            }

            self.consume_token(TokenKind::RightBracket, "Expected ']' after array values")?;
            Ok(IdentityField::ArrayAssignment {
                name,
                values,
                position,
            })
        } else {
            let value = self.parse_expression()?;
            Ok(IdentityField::Assignment {
                name,
                value,
                position,
            })
        }
    }

    // Helper methods for parsing expressions, tokens, etc.

    /// Parse an expression
    fn parse_expression(&mut self) -> Result<Expression> {
        match &self.current_token().kind {
            TokenKind::Dollar => {
                self.advance();
                let name = self.consume_identifier("Expected variable name after '$'")?;
                Ok(Expression::Variable(name))
            }
            TokenKind::String(s) => {
                let s = s.clone();
                self.advance();
                Ok(Expression::String(s))
            }
            TokenKind::Number(n) => {
                let n = *n;
                self.advance();
                Ok(Expression::Number(n))
            }
            TokenKind::Boolean(b) => {
                let b = *b;
                self.advance();
                Ok(Expression::Boolean(b))
            }
            TokenKind::Keyword(Keyword::True) => {
                self.advance();
                Ok(Expression::Boolean(true))
            }
            TokenKind::Keyword(Keyword::False) => {
                self.advance();
                Ok(Expression::Boolean(false))
            }
            TokenKind::LeftBrace => Ok(Expression::Object(self.parse_object_literal()?)),
            _ => Err(self.error("Expected expression".to_string())),
        }
    }

    /// Parse object literal: { key = value, ... }
    fn parse_object_literal(&mut self) -> Result<ObjectLiteral> {
        let position = self.current_position();
        self.consume_token(TokenKind::LeftBrace, "Expected '{'")?;

        let mut fields = Vec::new();
        while !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
            if self.match_token(&TokenKind::Newline) || self.match_comment() {
                continue;
            }

            let name = self.consume_identifier("Expected field name")?;
            self.consume_token(TokenKind::Assign, "Expected '=' after field name")?;

            // Check for period assignment
            if matches!(self.current_token().kind, TokenKind::String(_))
                || matches!(self.current_token().kind, TokenKind::Number(_))
            {
                // This might be a date range, check if there's a TO
                let from_expr = match &self.current_token().kind {
                    TokenKind::String(s) => {
                        let s = s.clone();
                        self.advance();
                        DateExpression::Literal(s)
                    }
                    TokenKind::Number(_) => {
                        // Handle numeric dates
                        let date = self.consume_string("Expected date")?;
                        DateExpression::Literal(date)
                    }
                    _ => return Err(self.error("Expected date expression".to_string())),
                };

                if self.match_keyword(Keyword::To) {
                    let to_expr = if self.match_keyword(Keyword::Current) {
                        Some(DateExpression::Current)
                    } else {
                        let date = self.consume_string("Expected date after 'to'")?;
                        Some(DateExpression::Literal(date))
                    };

                    let date_range = DateRange {
                        from: from_expr,
                        to: to_expr,
                        position,
                    };
                    fields.push(ObjectField::Period {
                        value: date_range,
                        position,
                    });
                } else {
                    // Regular assignment
                    let value = Expression::String(match from_expr {
                        DateExpression::Literal(s) => s,
                        DateExpression::Current => "CURRENT".to_string(),
                    });
                    fields.push(ObjectField::Assignment {
                        name,
                        value,
                        position,
                    });
                }
            } else {
                let value = self.parse_expression()?;
                fields.push(ObjectField::Assignment {
                    name,
                    value,
                    position,
                });
            }

            if !self.match_token(&TokenKind::Semicolon) && !self.match_token(&TokenKind::Comma) {
                break;
            }
        }

        self.consume_token(TokenKind::RightBrace, "Expected '}' after object fields")?;
        Ok(ObjectLiteral { fields, position })
    }

    // Lifecycle block parsing implementation
    fn parse_lifecycle_block(&mut self) -> Result<LifecycleBlock> {
        let position = self.current_position();
        self.consume_keyword(Keyword::Lifecycle, "Expected 'lifecycle'")?;
        self.consume_token(TokenKind::LeftBrace, "Expected '{'")?;

        let mut entries = Vec::new();
        while !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
            if self.match_token(&TokenKind::Newline) || self.match_comment() {
                continue;
            }

            // Parse lifecycle entry: status "name" from "date" [to "date"] { attributes }
            let entry = self.parse_lifecycle_entry()?;
            entries.push(entry);
            
            // Consume optional semicolon or comma
            self.consume_optional_semicolon();
            self.consume_optional_comma();
        }

        self.consume_token(TokenKind::RightBrace, "Expected '}' after lifecycle entries")?;
        Ok(LifecycleBlock { entries, position })
    }

    fn parse_lifecycle_entry(&mut self) -> Result<LifecycleEntry> {
        let position = self.current_position();
        
        // Parse "status" keyword
        self.consume_keyword(Keyword::Status, "Expected 'status'")?;
        
        // Parse status name (string)
        let status = self.consume_string("Expected status name")?;
        
        // Parse "from" keyword
        self.consume_keyword(Keyword::From, "Expected 'from' after status name")?;
        
        // Parse from date
        let from = self.parse_date_expression()?;
        
        // Parse optional "to" clause
        let to = if self.match_keyword(Keyword::To) {
            Some(self.parse_date_expression()?)
        } else {
            None
        };
        
        // Parse attributes block
        self.consume_token(TokenKind::LeftBrace, "Expected '{' after lifecycle entry header")?;
        
        let mut attributes = Vec::new();
        while !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
            if self.match_token(&TokenKind::Newline) {
                continue;
            }
            
            // Parse lifecycle attribute (this will handle comments)
            let attr = self.parse_lifecycle_attribute()?;
            attributes.push(attr);
            
            // Consume optional semicolon or comma
            self.consume_optional_semicolon();
            self.consume_optional_comma();
        }
        
        self.consume_token(TokenKind::RightBrace, "Expected '}' after lifecycle attributes")?;
        
        Ok(LifecycleEntry {
            status,
            from,
            to,
            attributes,
            position,
        })
    }

    fn parse_lifecycle_attribute(&mut self) -> Result<LifecycleAttribute> {
        let position = self.current_position();
        
        // Handle comment attributes
        if self.is_comment_token(&self.current_token().kind) {
            let comment = self.parse_comment()?;
            return Ok(LifecycleAttribute::Comment(comment));
        }
        
        // Handle annotation attributes
        if matches!(self.current_token().kind, TokenKind::Annotation(_)) {
            let annotation = self.parse_annotation()?;
            let comment = CommentStatement {
                text: format!("@{}: {}", annotation.name, annotation.value.unwrap_or_default()),
                is_multiline: false,
                position: annotation.position,
            };
            return Ok(LifecycleAttribute::Comment(comment));
        }
        
        // Parse assignment: name = value;
        let name = self.consume_identifier("Expected attribute name")?;
        self.consume_token(TokenKind::Assign, "Expected '=' after attribute name")?;
        let value = self.parse_expression()?;
        
        Ok(LifecycleAttribute::Assignment {
            name,
            value,
            position,
        })
    }

    fn parse_date_expression(&mut self) -> Result<DateExpression> {
        if self.match_keyword(Keyword::Current) {
            Ok(DateExpression::Current)
        } else {
            let date_string = self.consume_string("Expected date string or CURRENT")?;
            Ok(DateExpression::Literal(date_string))
        }
    }

    fn parse_characteristics_block(&mut self) -> Result<CharacteristicsBlock> {
        let position = self.current_position();
        self.consume_keyword(Keyword::Characteristics, "Expected 'characteristics'")?;
        self.consume_token(TokenKind::LeftBrace, "Expected '{'")?;

        let mut fields = Vec::new();
        while !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
            if self.match_token(&TokenKind::Newline) || self.match_comment() {
                continue;
            }

            let field_position = self.current_position();
            let name = self.consume_identifier("Expected field name")?;
            self.consume_token(TokenKind::Assign, "Expected '=' after field name")?;

            // Simple approach: skip to semicolon for complex expressions
            if matches!(self.current_token().kind, TokenKind::String(_)) {
                let string_value = self.consume_string("Expected string value")?;

                // If there's a { after the string, skip the entire complex expression
                if self.check(&TokenKind::LeftBrace) {
                    // Skip the complex nested object
                    let mut depth = 1;
                    self.advance(); // consume '{'
                    while depth > 0 && !self.is_at_end() {
                        match &self.current_token().kind {
                            TokenKind::LeftBrace => depth += 1,
                            TokenKind::RightBrace => depth -= 1,
                            _ => {}
                        }
                        self.advance();
                    }
                }

                // Add as simple assignment with the string value
                fields.push(CharacteristicField::Assignment {
                    name,
                    value: Expression::String(string_value),
                    position: field_position,
                });
            } else if self.check(&TokenKind::LeftBrace) {
                // Skip nested object literals for now
                let mut depth = 1;
                self.advance(); // consume '{'
                while depth > 0 && !self.is_at_end() {
                    match &self.current_token().kind {
                        TokenKind::LeftBrace => depth += 1,
                        TokenKind::RightBrace => depth -= 1,
                        _ => {}
                    }
                    self.advance();
                }

                // Add a placeholder assignment
                fields.push(CharacteristicField::Assignment {
                    name,
                    value: Expression::String("complex_object".to_string()),
                    position: field_position,
                });
            } else {
                // Try to parse as simple expression
                match self.parse_expression() {
                    Ok(value) => {
                        fields.push(CharacteristicField::Assignment {
                            name,
                            value,
                            position: field_position,
                        });
                    }
                    Err(_) => {
                        // Skip complex expressions for now
                        while !self.is_at_end() && !self.check(&TokenKind::Semicolon) {
                            self.advance();
                        }
                        if self.check(&TokenKind::Semicolon) {
                            self.advance();
                        }
                    }
                }
            }

            // Consume optional semicolon or comma
            self.consume_optional_semicolon();
            self.consume_optional_comma();
        }

        self.consume_token(
            TokenKind::RightBrace,
            "Expected '}' after characteristics fields",
        )?;
        Ok(CharacteristicsBlock { fields, position })
    }

    fn parse_metadata_block(&mut self) -> Result<MetadataBlock> {
        let position = self.current_position();
        self.consume_keyword(Keyword::Metadata, "Expected 'metadata'")?;
        self.consume_token(TokenKind::LeftBrace, "Expected '{'")?;

        let mut fields = Vec::new();
        while !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
            if self.match_token(&TokenKind::Newline) || self.match_comment() {
                continue;
            }

            let field_position = self.current_position();
            let name = self.consume_identifier("Expected field name")?;
            self.consume_token(TokenKind::Assign, "Expected '=' after field name")?;
            let value = self.parse_expression()?;

            fields.push(MetadataField::Assignment {
                name,
                value,
                position: field_position,
            });

            self.consume_optional_semicolon();
        }

        self.consume_token(TokenKind::RightBrace, "Expected '}' after metadata fields")?;
        Ok(MetadataBlock { fields, position })
    }

    fn parse_outlet_reference(&mut self) -> Result<OutletReference> {
        // TODO: Implement outlet reference parsing
        let position = self.current_position();
        self.consume_keyword(Keyword::OutletRef, "Expected 'outlet_ref'")?;
        let id = self.consume_number("Expected outlet ID")?;

        // Handle array syntax: ["Express"] or simple string
        let name = if self.check(&TokenKind::LeftBracket) {
            self.advance(); // consume '['
            let name = self.consume_string("Expected outlet name in array")?;
            self.consume_token(TokenKind::RightBracket, "Expected ']' after outlet name")?;
            name
        } else {
            self.consume_string("Expected outlet name")?
        };

        // Parse the block if present
        if self.check(&TokenKind::LeftBrace) {
            self.consume_token(TokenKind::LeftBrace, "Expected '{'")?;

            // Skip to end of block for now
            let mut depth = 1;
            while depth > 0 && !self.is_at_end() {
                match &self.current_token().kind {
                    TokenKind::LeftBrace => depth += 1,
                    TokenKind::RightBrace => depth -= 1,
                    _ => {}
                }
                self.advance();
            }
        }

        Ok(OutletReference { id, name, position })
    }

    fn parse_data(&mut self) -> Result<DataDeclaration> {
        // TODO: Implement data declaration parsing
        let position = self.current_position();
        self.consume_keyword(Keyword::Data, "Expected 'data'")?;
        self.consume_keyword(Keyword::For, "Expected 'for'")?;
        let target_id = self.consume_number("Expected target ID")?;

        self.consume_token(TokenKind::LeftBrace, "Expected '{'")?;

        // Skip to end of block for now
        let mut depth = 1;
        while depth > 0 && !self.is_at_end() {
            match &self.current_token().kind {
                TokenKind::LeftBrace => depth += 1,
                TokenKind::RightBrace => depth -= 1,
                _ => {}
            }
            self.advance();
        }

        Ok(DataDeclaration {
            target_id,
            blocks: Vec::new(),
            position,
        })
    }

    /// Parse event declaration: EVENT name { ... }
    fn parse_event(&mut self) -> Result<EventDeclaration> {
        let position = self.current_position();
        self.consume_keyword(Keyword::Event, "Expected 'event'")?;
        let name = self.consume_identifier("Expected event name")?;
        self.consume_token(TokenKind::LeftBrace, "Expected '{'")?;

        let mut fields = Vec::new();

        while !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
            if matches!(self.current_token().kind, TokenKind::Newline) {
                self.advance();
                continue;
            }

            match &self.current_token().kind {
                TokenKind::Keyword(Keyword::Type) => {
                    fields.push(self.parse_event_type_field()?);
                }
                TokenKind::Keyword(Keyword::Date) => {
                    fields.push(self.parse_event_date_field()?);
                }
                TokenKind::Keyword(Keyword::Status) => {
                    fields.push(self.parse_event_status_field()?);
                }
                TokenKind::Keyword(Keyword::Entities) => {
                    fields.push(self.parse_event_entities_field()?);
                }
                TokenKind::Keyword(Keyword::Impact) => {
                    fields.push(self.parse_event_impact_field()?);
                }
                TokenKind::Keyword(Keyword::Metadata) => {
                    fields.push(self.parse_event_metadata_field()?);
                }
                TokenKind::Annotation(_) => {
                    let annotation = self.parse_annotation()?;
                    fields.push(EventField::Annotation {
                        name: annotation.name,
                        value: annotation.value,
                        position: annotation.position,
                    });
                }
                TokenKind::Comment(_) | TokenKind::MultiLineComment(_) => {
                    let comment = self.parse_comment()?;
                    fields.push(EventField::Comment {
                        text: comment.text,
                        position: comment.position,
                    });
                }
                _ => {
                    return Err(self.error(format!(
                        "Unexpected token in event: {:?}",
                        self.current_token().kind
                    )));
                }
            }
        }

        self.consume_token(TokenKind::RightBrace, "Expected '}'")?;

        Ok(EventDeclaration {
            name,
            fields,
            position,
        })
    }

    /// Parse event type field: type = "acquisition";
    fn parse_event_type_field(&mut self) -> Result<EventField> {
        let position = self.current_position();
        self.consume_keyword(Keyword::Type, "Expected 'type'")?;
        self.consume_token(TokenKind::Assign, "Expected '='")?;
        let value = self.consume_string("Expected event type")?;
        self.consume_token(TokenKind::Semicolon, "Expected ';'")?;

        Ok(EventField::Type { value, position })
    }

    /// Parse event date field: date = "2010-03-15";
    fn parse_event_date_field(&mut self) -> Result<EventField> {
        let position = self.current_position();
        self.consume_keyword(Keyword::Date, "Expected 'date'")?;
        self.consume_token(TokenKind::Assign, "Expected '='")?;
        let value = if self.match_keyword(Keyword::Current) {
            DateExpression::Current
        } else {
            DateExpression::Literal(self.consume_string("Expected date or CURRENT")?)
        };
        self.consume_token(TokenKind::Semicolon, "Expected ';'")?;

        Ok(EventField::Date { value, position })
    }

    /// Parse event status field: status = "completed";
    fn parse_event_status_field(&mut self) -> Result<EventField> {
        let position = self.current_position();
        self.consume_keyword(Keyword::Status, "Expected 'status'")?;
        self.consume_token(TokenKind::Assign, "Expected '='")?;
        let value = self.consume_string("Expected event status")?;
        self.consume_token(TokenKind::Semicolon, "Expected ';'")?;

        Ok(EventField::Status { value, position })
    }

    /// Parse event entities field: entities = { ... };
    fn parse_event_entities_field(&mut self) -> Result<EventField> {
        let position = self.current_position();
        self.consume_keyword(Keyword::Entities, "Expected 'entities'")?;
        self.consume_token(TokenKind::Assign, "Expected '='")?;
        self.consume_token(TokenKind::LeftBrace, "Expected '{'")?;

        let mut entities = Vec::new();
        while !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
            if matches!(self.current_token().kind, TokenKind::Newline) {
                self.advance();
                continue;
            }
            entities.push(self.parse_event_entity()?);
        }

        self.consume_token(TokenKind::RightBrace, "Expected '}'")?;
        self.consume_token(TokenKind::Semicolon, "Expected ';'")?;

        Ok(EventField::Entities { entities, position })
    }

    /// Parse event entity: entity_name = { ... };
    fn parse_event_entity(&mut self) -> Result<EventEntity> {
        let position = self.current_position();
        let name = self.consume_identifier("Expected entity name")?;
        self.consume_token(TokenKind::Assign, "Expected '='")?;
        self.consume_token(TokenKind::LeftBrace, "Expected '{'")?;

        let mut roles = Vec::new();
        while !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
            if matches!(self.current_token().kind, TokenKind::Newline) {
                self.advance();
                continue;
            }
            roles.push(self.parse_entity_role()?);
        }

        self.consume_token(TokenKind::RightBrace, "Expected '}'")?;
        self.consume_token(TokenKind::Semicolon, "Expected ';'")?;

        Ok(EventEntity {
            name,
            roles,
            position,
        })
    }

    /// Parse entity role: id = 123; role = "buyer"; etc.
    fn parse_entity_role(&mut self) -> Result<EntityRole> {
        let position = self.current_position();

        match &self.current_token().kind {
            TokenKind::Identifier(name) if name == "id" => {
                self.advance();
                self.consume_token(TokenKind::Assign, "Expected '='")?;
                let value = self.consume_number("Expected entity ID")?;
                self.consume_token(TokenKind::Semicolon, "Expected ';'")?;
                Ok(EntityRole::Id { value, position })
            }
            TokenKind::Keyword(Keyword::Id) => {
                self.advance();
                self.consume_token(TokenKind::Assign, "Expected '='")?;
                let value = self.consume_number("Expected entity ID")?;
                self.consume_token(TokenKind::Semicolon, "Expected ';'")?;
                Ok(EntityRole::Id { value, position })
            }
            TokenKind::Keyword(Keyword::Role) => {
                self.advance();
                self.consume_token(TokenKind::Assign, "Expected '='")?;
                let value = self.consume_string("Expected role name")?;
                self.consume_token(TokenKind::Semicolon, "Expected ';'")?;
                Ok(EntityRole::Role { value, position })
            }
            TokenKind::Keyword(Keyword::StakeBefore) => {
                self.advance();
                self.consume_token(TokenKind::Assign, "Expected '='")?;
                let value = self.consume_number("Expected stake percentage")?;
                self.consume_token(TokenKind::Semicolon, "Expected ';'")?;
                Ok(EntityRole::StakeBefore { value, position })
            }
            TokenKind::Keyword(Keyword::StakeAfter) => {
                self.advance();
                self.consume_token(TokenKind::Assign, "Expected '='")?;
                let value = self.consume_number("Expected stake percentage")?;
                self.consume_token(TokenKind::Semicolon, "Expected ';'")?;
                Ok(EntityRole::StakeAfter { value, position })
            }
            _ => Err(self.error("Expected entity role field".to_string())),
        }
    }

    /// Parse event impact field: impact = { ... };
    fn parse_event_impact_field(&mut self) -> Result<EventField> {
        let position = self.current_position();
        self.consume_keyword(Keyword::Impact, "Expected 'impact'")?;
        self.consume_token(TokenKind::Assign, "Expected '='")?;
        self.consume_token(TokenKind::LeftBrace, "Expected '{'")?;

        let mut impact = Vec::new();
        while !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
            if matches!(self.current_token().kind, TokenKind::Newline) {
                self.advance();
                continue;
            }
            impact.push(self.parse_impact_field()?);
        }

        self.consume_token(TokenKind::RightBrace, "Expected '}'")?;
        self.consume_token(TokenKind::Semicolon, "Expected ';'")?;

        Ok(EventField::Impact { impact, position })
    }

    /// Parse impact field: field_name = value;
    fn parse_impact_field(&mut self) -> Result<ImpactField> {
        let position = self.current_position();
        let name = self.consume_identifier("Expected impact field name")?;
        self.consume_token(TokenKind::Assign, "Expected '='")?;
        let value = self.parse_expression()?;
        self.consume_token(TokenKind::Semicolon, "Expected ';'")?;

        Ok(ImpactField {
            name,
            value,
            position,
        })
    }

    /// Parse event metadata field: metadata = { ... };
    fn parse_event_metadata_field(&mut self) -> Result<EventField> {
        let position = self.current_position();
        self.consume_keyword(Keyword::Metadata, "Expected 'metadata'")?;
        self.consume_token(TokenKind::Assign, "Expected '='")?;
        self.consume_token(TokenKind::LeftBrace, "Expected '{'")?;

        let mut metadata = Vec::new();
        while !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
            if matches!(self.current_token().kind, TokenKind::Newline) {
                self.advance();
                continue;
            }
            metadata.push(self.parse_metadata_field_inner()?);
        }

        self.consume_token(TokenKind::RightBrace, "Expected '}'")?;
        self.consume_token(TokenKind::Semicolon, "Expected ';'")?;

        Ok(EventField::Metadata { metadata, position })
    }

    /// Parse metadata field: field_name = value;
    fn parse_metadata_field_inner(&mut self) -> Result<MetadataField> {
        let position = self.current_position();
        let name = self.consume_identifier("Expected metadata field name")?;
        self.consume_token(TokenKind::Assign, "Expected '='")?;
        let value = self.parse_expression()?;
        self.consume_token(TokenKind::Semicolon, "Expected ';'")?;

        Ok(MetadataField::Assignment {
            name,
            value,
            position,
        })
    }

    /// Parse catalog declaration: CATALOG name { ... }
    fn parse_catalog(&mut self) -> Result<CatalogDeclaration> {
        let position = self.current_position();
        self.consume_keyword(Keyword::Catalog, "Expected 'catalog'")?;
        let name = self.consume_identifier("Expected catalog name")?;
        self.consume_token(TokenKind::LeftBrace, "Expected '{' after catalog name")?;

        let mut sources = Vec::new();
        while !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
            if self.match_token(&TokenKind::Newline) || self.match_comment() {
                continue;
            }

            if self.match_keyword(Keyword::Source) {
                sources.push(self.parse_source_declaration()?);
            } else {
                return Err(self.error("Expected 'source' declaration".to_string()));
            }
        }

        self.consume_token(TokenKind::RightBrace, "Expected '}' after catalog sources")?;
        Ok(CatalogDeclaration {
            name,
            sources,
            position,
        })
    }

    /// Parse source declaration: SOURCE "name" { ... }
    fn parse_source_declaration(&mut self) -> Result<SourceDeclaration> {
        let position = self.current_position();
        let name = self.consume_string("Expected source name")?;
        self.consume_token(TokenKind::LeftBrace, "Expected '{' after source name")?;

        let mut fields = Vec::new();
        while !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
            if self.match_token(&TokenKind::Newline) || self.match_comment() {
                continue;
            }

            fields.push(self.parse_source_field()?);
            self.consume_optional_semicolon();
        }

        self.consume_token(TokenKind::RightBrace, "Expected '}' after source fields")?;
        Ok(SourceDeclaration {
            name,
            fields,
            position,
        })
    }

    /// Parse source field
    fn parse_source_field(&mut self) -> Result<SourceField> {
        let position = self.current_position();

        // Check for annotations
        if let TokenKind::Annotation(_name) = &self.current_token().kind {
            let annotation = self.parse_annotation()?;
            return Ok(SourceField::Annotation(annotation));
        }

        // Check for comments
        if matches!(
            self.current_token().kind,
            TokenKind::Comment(_) | TokenKind::MultiLineComment(_)
        ) {
            let comment = self.parse_comment()?;
            return Ok(SourceField::Comment(comment));
        }

        // Parse field name
        let name = self.consume_identifier("Expected field name")?;

        // Check if this is a block without assignment (field { ... })
        if self.check(&TokenKind::LeftBrace) {
            self.advance(); // consume '{'
            let mut nested_fields = Vec::new();

            while !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
                if self.match_token(&TokenKind::Newline) || self.match_comment() {
                    continue;
                }

                nested_fields.push(self.parse_nested_source_field()?);
                self.consume_optional_semicolon();
                self.skip_whitespace_and_comments();
            }

            self.consume_token(TokenKind::RightBrace, "Expected '}' after nested fields")?;
            Ok(SourceField::NestedAssignment {
                name,
                fields: nested_fields,
                position,
            })
        } else {
            // Simple assignment - expect '=' after field name
            self.consume_token(TokenKind::Assign, "Expected '=' after field name")?;
            let value = self.parse_expression()?;
            Ok(SourceField::Assignment {
                name,
                value,
                position,
            })
        }
    }

    /// Parse nested source field
    fn parse_nested_source_field(&mut self) -> Result<NestedSourceField> {
        let position = self.current_position();

        // Check for comments
        if matches!(
            self.current_token().kind,
            TokenKind::Comment(_) | TokenKind::MultiLineComment(_)
        ) {
            let comment = self.parse_comment()?;
            return Ok(NestedSourceField::Comment(comment));
        }

        // Parse field assignment
        let name = self.consume_identifier("Expected field name")?;
        self.consume_token(TokenKind::Assign, "Expected '=' after field name")?;
        let value = self.parse_expression()?;

        Ok(NestedSourceField::Assignment {
            name,
            value,
            position,
        })
    }

    fn parse_diachronic_link(&mut self) -> Result<DiachronicLink> {
        let position = self.current_position();
        self.consume_keyword(Keyword::DiachronicLink, "Expected 'diachronic_link'")?;
        
        // Link name can be either an identifier or string
        let name = if self.check_string() {
            self.consume_string("Expected link name")?
        } else {
            self.consume_identifier("Expected link name")?
        };

        self.consume_token(TokenKind::LeftBrace, "Expected '{'")?;

        let mut fields = Vec::new();
        
        while !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
            // Skip newlines and whitespace first
            if matches!(self.current_token().kind, TokenKind::Newline) {
                self.advance();
                continue;
            }
            
            // Skip comments
            if self.is_comment_token(&self.current_token().kind) {
                let comment = self.parse_comment()?;
                fields.push(DiachronicField::Comment(comment));
                continue;
            }
            
            // Parse field assignments - handle both identifiers and keywords
            let field_name = match &self.current_token().kind {
                TokenKind::Identifier(name) => name.clone(),
                TokenKind::Keyword(kw) => kw.to_string(),
                _ => {
                    return Err(self.error(format!("Expected field name in diachronic link, found {:?}", self.current_token().kind)));
                }
            };
            
            let field_pos = self.current_position();
            self.advance(); // consume field name
            
            self.consume_token(TokenKind::Assign, "Expected '=' after field name")?;
            
            match field_name.as_str() {
                "predecessor" => {
                    let value = self.consume_number("Expected predecessor ID")?;
                    fields.push(DiachronicField::Predecessor { value, position: field_pos });
                }
                "successor" => {
                    let value = self.consume_number("Expected successor ID")?;
                    fields.push(DiachronicField::Successor { value, position: field_pos });
                }
                "relationship_type" => {
                    let value = self.consume_string("Expected relationship type")?;
                    fields.push(DiachronicField::RelationshipType { value, position: field_pos });
                }
                "event_date" => {
                    let date_string = self.consume_string("Expected event date")?;
                    // Create a simple date range from the string
                    let date_range = DateRange {
                        from: DateExpression::Literal(date_string),
                        to: None,
                        position: field_pos,
                    };
                    fields.push(DiachronicField::EventDate { value: date_range, position: field_pos });
                }
                "triggered_by_event" => {
                    let value = self.consume_identifier("Expected event identifier")?;
                    fields.push(DiachronicField::TriggeredByEvent { value, position: field_pos });
                }
                _ => {
                    return Err(self.error(format!("Unknown diachronic field: {}", field_name)));
                }
            }
                
            // Consume optional semicolon or comma
            self.consume_optional_semicolon();
            self.consume_optional_comma();
            self.skip_whitespace_and_comments();
        }
        
        self.consume_token(TokenKind::RightBrace, "Expected '}'")?;
        self.consume_optional_semicolon();

        Ok(DiachronicLink {
            name,
            fields,
            position,
        })
    }

    fn parse_synchronous_link(&mut self) -> Result<SynchronousLink> {
        let position = self.current_position();
        // Accept both singular and plural forms
        if !self.match_keyword(Keyword::SynchronousLink)
            && !self.match_keyword(Keyword::SynchronousLinks)
        {
            return Err(
                self.error("Expected 'synchronous_link' or 'synchronous_links'".to_string())
            );
        }
        // Link name can be either an identifier or string
        let name = if self.check_string() {
            self.consume_string("Expected link name")?
        } else {
            self.consume_identifier("Expected link name")?
        };

        self.consume_token(TokenKind::LeftBrace, "Expected '{'")?;

        let mut fields = Vec::new();
        
        while !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
            // Skip newlines and whitespace first
            if matches!(self.current_token().kind, TokenKind::Newline) {
                self.advance();
                continue;
            }
            
            // Skip comments
            if self.is_comment_token(&self.current_token().kind) {
                let comment = self.parse_comment()?;
                fields.push(SynchronousField::Comment(comment));
                continue;
            }
            
            // Parse field assignments - handle both identifiers and keywords  
            let field_name = match &self.current_token().kind {
                TokenKind::Identifier(name) => name.clone(),
                TokenKind::Keyword(kw) => kw.to_string(),
                _ => {
                    return Err(self.error(format!("Expected field name in synchronous link, found {:?}", self.current_token().kind)));
                }
            };
            
            let field_pos = self.current_position();
            self.advance(); // consume field name
            
            self.consume_token(TokenKind::Assign, "Expected '=' after field name")?;
            
            match field_name.as_str() {
                "outlet_1" => {
                    let outlet_spec = self.parse_outlet_spec()?;
                    fields.push(SynchronousField::Outlet1 { spec: outlet_spec, position: field_pos });
                }
                "outlet_2" => {
                    let outlet_spec = self.parse_outlet_spec()?;
                    fields.push(SynchronousField::Outlet2 { spec: outlet_spec, position: field_pos });
                }
                "relationship_type" => {
                    let value = self.consume_string("Expected relationship type")?;
                    fields.push(SynchronousField::RelationshipType { value, position: field_pos });
                }
                "period_start" | "period_end" => {
                    let date = if self.match_keyword(Keyword::Current) {
                        DateExpression::Current
                    } else {
                        DateExpression::Literal(self.consume_string("Expected date or CURRENT")?)
                    };
                    let date_range = DateRange {
                        from: date,
                        to: None,
                        position: field_pos,
                    };
                    fields.push(SynchronousField::Period { value: date_range, position: field_pos });
                }
                "created_by_event" => {
                    let value = self.consume_identifier("Expected event identifier")?;
                    fields.push(SynchronousField::CreatedByEvent { value, position: field_pos });
                }
                _ => {
                    return Err(self.error(format!("Unknown synchronous field: {}", field_name)));
                }
            }
            
            // Consume optional semicolon or comma
            self.consume_optional_semicolon();
            self.consume_optional_comma();
            self.skip_whitespace_and_comments();
        }
        
        self.consume_token(TokenKind::RightBrace, "Expected '}'")?;
        self.consume_optional_semicolon();

        Ok(SynchronousLink {
            name,
            fields,
            position,
        })
    }

    fn parse_outlet_spec(&mut self) -> Result<OutletSpec> {
        self.consume_token(TokenKind::LeftBrace, "Expected '{' for outlet spec")?;
        
        let mut id = None;
        let mut role = None;
        
        while !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
            // Skip newlines and whitespace first
            if matches!(self.current_token().kind, TokenKind::Newline) {
                self.advance();
                continue;
            }
            
            // Skip comments
            if self.is_comment_token(&self.current_token().kind) {
                self.advance(); // Just skip comments in outlet specs
                continue;
            }
            
            // Parse field assignments - handle both identifiers and keywords
            let field_name = match &self.current_token().kind {
                TokenKind::Identifier(name) => name.clone(),
                TokenKind::Keyword(kw) => kw.to_string(),
                _ => {
                    return Err(self.error(format!("Expected field name in outlet spec, found {:?}", self.current_token().kind)));
                }
            };
            
            self.advance(); // consume field name
            
            self.consume_token(TokenKind::Assign, "Expected '=' after field name")?;
            
            match field_name.as_str() {
                "id" | "ID" => {
                    id = Some(self.consume_number("Expected outlet ID")?);
                }
                "role" => {
                    role = Some(self.consume_string("Expected role")?);
                }
                _ => {
                    return Err(self.error(format!("Unknown outlet field: {}", field_name)));
                }
            }
            
            self.consume_optional_semicolon();
            self.skip_whitespace_and_comments();
        }
        
        self.consume_token(TokenKind::RightBrace, "Expected '}' after outlet spec")?;
        
        Ok(OutletSpec {
            id: id.unwrap_or(0.0),
            role,
            position: self.current_position(),
        })
    }

    fn parse_comment(&mut self) -> Result<CommentStatement> {
        let position = self.current_position();
        match &self.current_token().kind {
            TokenKind::Comment(text) => {
                let text = text.clone();
                self.advance();
                Ok(CommentStatement {
                    text,
                    is_multiline: false,
                    position,
                })
            }
            TokenKind::MultiLineComment(text) => {
                let text = text.clone();
                self.advance();
                Ok(CommentStatement {
                    text,
                    is_multiline: true,
                    position,
                })
            }
            _ => Err(self.error("Expected comment".to_string())),
        }
    }

    fn parse_annotation(&mut self) -> Result<AnnotationStatement> {
        let position = self.current_position();
        match &self.current_token().kind {
            TokenKind::Annotation(name) => {
                let name = name.clone();
                self.advance();

                // Check if there's an equals sign followed by a value
                let value = if self.match_token(&TokenKind::Assign) {
                    if matches!(self.current_token().kind, TokenKind::String(_)) {
                        Some(self.consume_string("Expected annotation value")?)
                    } else {
                        return Err(self.error("Expected string value after '='".to_string()));
                    }
                } else if matches!(self.current_token().kind, TokenKind::String(_)) {
                    // Direct string value without equals sign
                    Some(self.consume_string("Expected annotation value")?)
                } else {
                    None
                };

                Ok(AnnotationStatement {
                    name,
                    value,
                    position,
                })
            }
            _ => Err(self.error("Expected annotation".to_string())),
        }
    }

    // Utility methods

    /// Get the current token
    fn current_token(&self) -> &Token {
        static EOF_TOKEN: Token = Token {
            kind: TokenKind::Eof,
            text: String::new(),
            position: SourcePosition {
                line: 1,
                column: 1,
                offset: 0,
            },
        };
        self.tokens.get(self.current).unwrap_or(&EOF_TOKEN)
    }

    /// Get the current position
    fn current_position(&self) -> SourcePosition {
        self.current_token().position
    }

    /// Check if we're at the end of the token stream
    fn is_at_end(&self) -> bool {
        matches!(self.current_token().kind, TokenKind::Eof) || self.current >= self.tokens.len()
    }

    /// Advance to the next token
    fn advance(&mut self) {
        if !self.is_at_end() {
            self.current += 1;
        }
    }

    /// Check if current token matches the given kind
    fn check(&self, kind: &TokenKind) -> bool {
        &self.current_token().kind == kind
    }

    /// Check if current token is a string
    fn check_string(&self) -> bool {
        matches!(self.current_token().kind, TokenKind::String(_))
    }

    /// Match a token and advance if it matches
    fn match_token(&mut self, kind: &TokenKind) -> bool {
        if self.check(kind) {
            self.advance();
            true
        } else {
            false
        }
    }

    /// Match a keyword and advance if it matches
    fn match_keyword(&mut self, keyword: Keyword) -> bool {
        if let TokenKind::Keyword(kw) = &self.current_token().kind {
            if *kw == keyword {
                self.advance();
                return true;
            }
        }
        false
    }

    /// Match multiple keywords in sequence
    fn match_keywords(&mut self, keywords: &[Keyword]) -> bool {
        let start_pos = self.current;
        for keyword in keywords {
            if !self.match_keyword(keyword.clone()) {
                self.current = start_pos;
                return false;
            }
        }
        true
    }

    /// Check if current token is a comment
    fn match_comment(&mut self) -> bool {
        match &self.current_token().kind {
            TokenKind::Comment(_) | TokenKind::MultiLineComment(_) => {
                self.advance();
                true
            }
            _ => false,
        }
    }

    /// Consume a token of the given kind or return an error
    fn consume_token(&mut self, kind: TokenKind, message: &str) -> Result<()> {
        if self.check(&kind) {
            self.advance();
            Ok(())
        } else {
            Err(self.error(message.to_string()))
        }
    }

    /// Consume a keyword or return an error
    fn consume_keyword(&mut self, keyword: Keyword, message: &str) -> Result<()> {
        if self.match_keyword(keyword) {
            Ok(())
        } else {
            Err(self.error(message.to_string()))
        }
    }

    /// Consume an identifier or return an error
    fn consume_identifier(&mut self, message: &str) -> Result<String> {
        match &self.current_token().kind {
            TokenKind::Identifier(name) => {
                let name = name.clone();
                self.advance();
                Ok(name)
            }
            TokenKind::Keyword(keyword) => {
                // Allow certain keywords to be used as field names
                let name = match keyword {
                    Keyword::Id => "id",
                    Keyword::Period => "PERIOD",
                    Keyword::Status => "status",
                    Keyword::From => "from",
                    Keyword::To => "to",
                    Keyword::Current => "current",
                    Keyword::Year => "year",
                    Keyword::For => "for",
                    Keyword::Data => "data",
                    Keyword::Text => "text",
                    Keyword::Number => "number",
                    Keyword::Boolean => "boolean",
                    Keyword::Category => "category",
                    Keyword::Primary => "primary",
                    Keyword::Key => "key",
                    Keyword::Unit => "unit",
                    Keyword::Role => "role",
                    Keyword::Details => "details",
                    Keyword::RelationshipType => "RELATIONSHIP_TYPE",
                    Keyword::Type => "type",
                    Keyword::Date => "date",
                    Keyword::Entities => "entities",
                    Keyword::Impact => "impact",
                    _ => return Err(self.error(message.to_string())),
                };
                self.advance();
                Ok(name.to_string())
            }
            _ => Err(self.error(message.to_string())),
        }
    }

    /// Consume a string or return an error
    fn consume_string(&mut self, message: &str) -> Result<String> {
        match &self.current_token().kind {
            TokenKind::String(s) => {
                let s = s.clone();
                self.advance();
                Ok(s)
            }
            _ => Err(self.error(message.to_string())),
        }
    }

    /// Consume a number or return an error
    fn consume_number(&mut self, message: &str) -> Result<f64> {
        match &self.current_token().kind {
            TokenKind::Number(n) => {
                let n = *n;
                self.advance();
                Ok(n)
            }
            _ => Err(self.error(message.to_string())),
        }
    }

    /// Consume optional semicolon
    fn consume_optional_semicolon(&mut self) {
        self.match_token(&TokenKind::Semicolon);
    }

    /// Consume optional comma
    #[allow(dead_code)]
    fn consume_optional_comma(&mut self) {
        self.match_token(&TokenKind::Comma);
    }

    /// Peek at the next token without advancing
    fn peek_token(&self) -> Option<&Token> {
        if self.current + 1 < self.tokens.len() {
            Some(&self.tokens[self.current + 1])
        } else {
            None
        }
    }

    /// Check if a token is a comment
    fn is_comment_token(&self, kind: &TokenKind) -> bool {
        matches!(kind, TokenKind::Comment(_) | TokenKind::MultiLineComment(_))
    }

    fn skip_whitespace_and_comments(&mut self) {
        while !self.is_at_end() {
            match &self.current_token().kind {
                TokenKind::Comment(_) | TokenKind::MultiLineComment(_) | TokenKind::Newline => {
                    self.advance();
                }
                _ => break,
            }
        }
    }

    /// Create a parse error
    fn error(&self, message: String) -> crate::error::Error {
        crate::error::Error::Parser(ParseError::UnexpectedToken {
            expected: vec![message],
            found: self.current_token().text.clone(),
            position: self.current_position(),
        })
    }

    /// Synchronize parser after an error
    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if matches!(self.current_token().kind, TokenKind::Semicolon) {
                self.advance();
                return;
            }

            match &self.current_token().kind {
                TokenKind::Keyword(Keyword::Import)
                | TokenKind::Keyword(Keyword::Let)
                | TokenKind::Keyword(Keyword::Unit)
                | TokenKind::Keyword(Keyword::Vocabulary)
                | TokenKind::Keyword(Keyword::Family)
                | TokenKind::Keyword(Keyword::Template)
                | TokenKind::Keyword(Keyword::Data) => return,
                _ => {}
            }

            self.advance();
        }
    }


}
