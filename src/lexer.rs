//! # MDSL Lexer - Hand-crafted Tokenizer
//!
//! Purpose: Transform MDSL source text into a stream of tokens for parsing.
//! This lexer is hand-written for educational purposes and maximum control.
//!
//! ## Design Philosophy
//! - No external parser dependencies - pure Rust implementation
//! - Clear separation of concerns between lexical and syntactic analysis
//! - Comprehensive error reporting with position information
//! - Support for all MDSL language constructs found in the examples

use std::fmt;

/// Represents a position in the source file for error reporting
#[derive(Debug, Clone, PartialEq)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}

/// All possible token types in the MDSL language
/// Based on analysis of kronen_zeitung_freeze3.mdsl and express_freeze3.mdsl
#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    // Keywords (all uppercase as per specification)
    Import,          // IMPORT
    Let,             // LET
    Template,        // TEMPLATE
    Family,          // FAMILY
    Outlet,          // OUTLET
    Extends,         // EXTENDS
    BasedOn,         // BASED_ON
    OutletRef,       // OUTLET_REF
    From,            // FROM
    To,              // TO
    Until,           // UNTIL
    Current,         // CURRENT
    Year,            // YEAR
    Data,            // DATA
    For,             // FOR
    Override,        // OVERRIDE
    ForPeriod,       // FOR_PERIOD
    InheritsFrom,    // INHERITS_FROM
    DiachronicLink,  // DIACHRONIC_LINK
    SynchronousLink, // SYNCHRONOUS_LINK
    Unit,            // UNIT
    Vocabulary,      // VOCABULARY
    Category,        // CATEGORY
    Boolean,         // BOOLEAN
    Number,          // NUMBER (keyword for type)
    Text,            // TEXT (keyword for type)
    Id,              // ID (keyword for type)
    PrimaryKey,      // PRIMARY_KEY

    // Identifiers and literals
    Identifier(String),    // Variable names, field names
    StringLiteral(String), // "quoted strings"
    NumberLiteral(f64),    // 200001, 45000, 25.0
    BooleanLiteral(bool),  // true, false

    // Variables (starting with $)
    Variable(String), // $austria_region, $founding_note

    // Annotations
    Comment(String), // @comment "text"
    MapsTo(String),  // @maps_to "table_name"

    // Operators and delimiters
    Assign,    // =
    Semicolon, // ;
    Comma,     // ,
    Dot,       // .
    Colon,     // :

    // Brackets and braces
    LeftBrace,    // {
    RightBrace,   // }
    LeftBracket,  // [
    RightBracket, // ]
    LeftParen,    // (
    RightParen,   // )

    // End of file
    Eof,
}

/// A token with its kind and source position
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub position: Position,
}

impl Token {
    pub fn new(kind: TokenKind, position: Position) -> Self {
        Self { kind, position }
    }
}

/// Hand-crafted lexer for MDSL
///
/// Processes input character by character, maintaining position for error reporting.
/// Follows the principle of longest match for keywords vs identifiers.
pub struct Lexer<'a> {
    input: &'a str,
    chars: std::str::Chars<'a>,
    current_char: Option<char>,
    position: Position,
    peeked_char: Option<char>,
}

impl<'a> Lexer<'a> {
    /// Create a new lexer for the given input
    pub fn new(input: &'a str) -> Self {
        let mut chars = input.chars();
        let current_char = chars.next();

        Self {
            input,
            chars,
            current_char,
            position: Position { line: 1, column: 1 },
            peeked_char: None,
        }
    }

    /// Advance to the next character, updating position tracking
    fn advance(&mut self) {
        if let Some(ch) = self.current_char {
            if ch == '\n' {
                self.position.line += 1;
                self.position.column = 1;
            } else {
                self.position.column += 1;
            }
        }

        self.current_char = if let Some(peeked) = self.peeked_char.take() {
            Some(peeked)
        } else {
            self.chars.next()
        };
    }

    /// Peek at the next character without consuming it
    fn peek(&mut self) -> Option<char> {
        if self.peeked_char.is_none() {
            self.peeked_char = self.chars.next();
        }
        self.peeked_char
    }

    /// Skip whitespace and comments
    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.current_char {
            if ch.is_whitespace() {
                self.advance();
            } else if ch == '/' && self.peek() == Some('/') {
                // Skip line comment
                self.skip_line_comment();
            } else {
                break;
            }
        }
    }

    /// Skip a line comment (// to end of line)
    fn skip_line_comment(&mut self) {
        while let Some(ch) = self.current_char {
            self.advance();
            if ch == '\n' {
                break;
            }
        }
    }

    /// Read a string literal, handling escape sequences
    fn read_string(&mut self) -> Result<String, String> {
        let mut value = String::new();
        self.advance(); // Skip opening quote

        while let Some(ch) = self.current_char {
            match ch {
                '"' => {
                    self.advance(); // Skip closing quote
                    return Ok(value);
                }
                '\\' => {
                    self.advance();
                    match self.current_char {
                        Some('n') => value.push('\n'),
                        Some('t') => value.push('\t'),
                        Some('r') => value.push('\r'),
                        Some('\\') => value.push('\\'),
                        Some('"') => value.push('"'),
                        Some(c) => value.push(c),
                        None => {
                            return Err(format!(
                                "Unterminated escape sequence at {}",
                                self.position
                            ))
                        }
                    }
                    self.advance();
                }
                '\n' => {
                    self.advance();
                    value.push(ch);
                }
                _ => {
                    value.push(ch);
                    self.advance();
                }
            }
        }

        Err(format!("Unterminated string literal at {}", self.position))
    }

    /// Read a number (integer or float)
    fn read_number(&mut self) -> Result<f64, String> {
        let mut value = String::new();

        // Read integer part
        while let Some(ch) = self.current_char {
            if ch.is_ascii_digit() {
                value.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        // Check for decimal point
        if self.current_char == Some('.') && self.peek().map_or(false, |c| c.is_ascii_digit()) {
            value.push('.');
            self.advance();

            // Read fractional part
            while let Some(ch) = self.current_char {
                if ch.is_ascii_digit() {
                    value.push(ch);
                    self.advance();
                } else {
                    break;
                }
            }
        }

        value
            .parse()
            .map_err(|e| format!("Invalid number '{}' at {}: {}", value, self.position, e))
    }

    /// Read an identifier or keyword
    fn read_identifier(&mut self) -> String {
        let mut value = String::new();

        while let Some(ch) = self.current_char {
            if ch.is_alphanumeric() || ch == '_' {
                value.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        value
    }

    /// Read a variable (starting with $)
    fn read_variable(&mut self) -> String {
        self.advance(); // Skip $
        self.read_identifier()
    }

    /// Read an annotation (@comment, @maps_to, etc.)
    fn read_annotation(&mut self) -> Result<TokenKind, String> {
        self.advance(); // Skip @
        let name = self.read_identifier();

        self.skip_whitespace();

        match name.as_str() {
            "comment" => {
                if self.current_char == Some('"') {
                    let content = self.read_string()?;
                    Ok(TokenKind::Comment(content))
                } else {
                    Err(format!(
                        "Expected string after @comment at {}",
                        self.position
                    ))
                }
            }
            "maps_to" => {
                if self.current_char == Some('"') {
                    let content = self.read_string()?;
                    Ok(TokenKind::MapsTo(content))
                } else {
                    Err(format!(
                        "Expected string after @maps_to at {}",
                        self.position
                    ))
                }
            }
            _ => Err(format!("Unknown annotation @{} at {}", name, self.position)),
        }
    }

    /// Convert identifier to keyword token or return as identifier
    fn identifier_to_token(&self, word: String) -> TokenKind {
        match word.as_str() {
            // Primary keywords
            "IMPORT" => TokenKind::Import,
            "LET" => TokenKind::Let,
            "TEMPLATE" => TokenKind::Template,
            "FAMILY" => TokenKind::Family,
            "OUTLET" => TokenKind::Outlet,
            "EXTENDS" => TokenKind::Extends,
            "BASED_ON" => TokenKind::BasedOn,
            "OUTLET_REF" => TokenKind::OutletRef,
            "FROM" => TokenKind::From,
            "TO" => TokenKind::To,
            "UNTIL" => TokenKind::Until,
            "CURRENT" => TokenKind::Current,
            "YEAR" => TokenKind::Year,
            "DATA" => TokenKind::Data,
            "FOR" => TokenKind::For,
            "OVERRIDE" => TokenKind::Override,
            "FOR_PERIOD" => TokenKind::ForPeriod,
            "INHERITS_FROM" => TokenKind::InheritsFrom,
            "DIACHRONIC_LINK" => TokenKind::DiachronicLink,
            "SYNCHRONOUS_LINK" => TokenKind::SynchronousLink,

            // Type keywords
            "UNIT" => TokenKind::Unit,
            "VOCABULARY" => TokenKind::Vocabulary,
            "CATEGORY" => TokenKind::Category,
            "BOOLEAN" => TokenKind::Boolean,
            "NUMBER" => TokenKind::Number,
            "TEXT" => TokenKind::Text,
            "ID" => TokenKind::Id,
            "PRIMARY_KEY" => TokenKind::PrimaryKey,

            // Boolean literals
            "true" => TokenKind::BooleanLiteral(true),
            "false" => TokenKind::BooleanLiteral(false),

            // Otherwise it's an identifier
            _ => TokenKind::Identifier(word),
        }
    }

    /// Get the next token from the input
    pub fn next_token(&mut self) -> Result<Token, String> {
        self.skip_whitespace();

        let start_position = self.position.clone();

        match self.current_char {
            None => Ok(Token::new(TokenKind::Eof, start_position)),

            Some(ch) => match ch {
                // Single-character tokens
                '=' => {
                    self.advance();
                    Ok(Token::new(TokenKind::Assign, start_position))
                }
                ';' => {
                    self.advance();
                    Ok(Token::new(TokenKind::Semicolon, start_position))
                }
                ',' => {
                    self.advance();
                    Ok(Token::new(TokenKind::Comma, start_position))
                }
                '.' => {
                    self.advance();
                    Ok(Token::new(TokenKind::Dot, start_position))
                }
                ':' => {
                    self.advance();
                    Ok(Token::new(TokenKind::Colon, start_position))
                }
                '{' => {
                    self.advance();
                    Ok(Token::new(TokenKind::LeftBrace, start_position))
                }
                '}' => {
                    self.advance();
                    Ok(Token::new(TokenKind::RightBrace, start_position))
                }
                '[' => {
                    self.advance();
                    Ok(Token::new(TokenKind::LeftBracket, start_position))
                }
                ']' => {
                    self.advance();
                    Ok(Token::new(TokenKind::RightBracket, start_position))
                }
                '(' => {
                    self.advance();
                    Ok(Token::new(TokenKind::LeftParen, start_position))
                }
                ')' => {
                    self.advance();
                    Ok(Token::new(TokenKind::RightParen, start_position))
                }

                // String literals
                '"' => {
                    let content = self.read_string()?;
                    Ok(Token::new(
                        TokenKind::StringLiteral(content),
                        start_position,
                    ))
                }

                // Variables (starting with $)
                '$' => {
                    let name = self.read_variable();
                    Ok(Token::new(TokenKind::Variable(name), start_position))
                }

                // Annotations (starting with @)
                '@' => {
                    let annotation = self.read_annotation()?;
                    Ok(Token::new(annotation, start_position))
                }

                // Numbers
                c if c.is_ascii_digit() => {
                    let value = self.read_number()?;
                    Ok(Token::new(TokenKind::NumberLiteral(value), start_position))
                }

                // Identifiers and keywords
                c if c.is_alphabetic() || c == '_' => {
                    let word = self.read_identifier();
                    let token_kind = self.identifier_to_token(word);
                    Ok(Token::new(token_kind, start_position))
                }

                // Unexpected character
                _ => {
                    self.advance();
                    Err(format!(
                        "Unexpected character '{}' at {}",
                        ch, start_position
                    ))
                }
            },
        }
    }
}

/// Iterator implementation for convenient token streaming
impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token, String>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next_token() {
            Ok(token) if token.kind == TokenKind::Eof => None,
            result => Some(result),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_tokens() {
        let input = "IMPORT LET = ; { } [ ]";
        let mut lexer = Lexer::new(input);

        assert_eq!(lexer.next_token().unwrap().kind, TokenKind::Import);
        assert_eq!(lexer.next_token().unwrap().kind, TokenKind::Let);
        assert_eq!(lexer.next_token().unwrap().kind, TokenKind::Assign);
        assert_eq!(lexer.next_token().unwrap().kind, TokenKind::Semicolon);
        assert_eq!(lexer.next_token().unwrap().kind, TokenKind::LeftBrace);
        assert_eq!(lexer.next_token().unwrap().kind, TokenKind::RightBrace);
        assert_eq!(lexer.next_token().unwrap().kind, TokenKind::LeftBracket);
        assert_eq!(lexer.next_token().unwrap().kind, TokenKind::RightBracket);
        assert_eq!(lexer.next_token().unwrap().kind, TokenKind::Eof);
    }

    #[test]
    fn test_string_literal() {
        let input = r#""Hello World""#;
        let mut lexer = Lexer::new(input);

        match lexer.next_token().unwrap().kind {
            TokenKind::StringLiteral(s) => assert_eq!(s, "Hello World"),
            _ => panic!("Expected string literal"),
        }
    }

    #[test]
    fn test_number_literal() {
        let input = "123 45.67";
        let mut lexer = Lexer::new(input);

        match lexer.next_token().unwrap().kind {
            TokenKind::NumberLiteral(n) => assert_eq!(n, 123.0),
            _ => panic!("Expected number literal"),
        }

        match lexer.next_token().unwrap().kind {
            TokenKind::NumberLiteral(n) => assert_eq!(n, 45.67),
            _ => panic!("Expected number literal"),
        }
    }

    #[test]
    fn test_variable() {
        let input = "$austria_region";
        let mut lexer = Lexer::new(input);

        match lexer.next_token().unwrap().kind {
            TokenKind::Variable(name) => assert_eq!(name, "austria_region"),
            _ => panic!("Expected variable"),
        }
    }

    #[test]
    fn test_annotation() {
        let input = r#"@comment "This is a comment""#;
        let mut lexer = Lexer::new(input);

        match lexer.next_token().unwrap().kind {
            TokenKind::Comment(content) => assert_eq!(content, "This is a comment"),
            _ => panic!("Expected comment annotation"),
        }
    }

    #[test]
    fn test_keywords() {
        let input = "OUTLET EXTENDS TEMPLATE FAMILY CURRENT";
        let mut lexer = Lexer::new(input);

        assert_eq!(lexer.next_token().unwrap().kind, TokenKind::Outlet);
        assert_eq!(lexer.next_token().unwrap().kind, TokenKind::Extends);
        assert_eq!(lexer.next_token().unwrap().kind, TokenKind::Template);
        assert_eq!(lexer.next_token().unwrap().kind, TokenKind::Family);
        assert_eq!(lexer.next_token().unwrap().kind, TokenKind::Current);
    }
}
