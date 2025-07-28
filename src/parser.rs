// Purpose: Parse tokens into an AST, using recursive descent for simplicity and tutorial value.

use crate::ast::*;
use crate::lexer::{Lexer, Token, TokenKind};
use std::collections::HashMap;

#[derive(Debug)]
pub enum ParseError {
    UnexpectedToken(TokenKind, Position, String), // Found, Position, Expected
    LexerError(String),
}

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Result<Self, ParseError> {
        let mut lexer = Lexer::new(input);
        let first_token = lexer.next_token().map_err(ParseError::LexerError)?;
        Ok(Parser { lexer, current_token: first_token })
    }

    fn advance(&mut self) -> Result<(), ParseError> {
        self.current_token = self.lexer.next_token().map_err(ParseError::LexerError)?;
        Ok(())
    }

    fn expect(&mut self, kind: TokenKind) -> Result<(), ParseError> {
        if self.current_token.kind == kind {
            self.advance()?;
            Ok(())
        } else {
            Err(ParseError::UnexpectedToken(
                self.current_token.kind.clone(),
                self.current_token.position.clone(),
                format!("{:?}", kind),
            ))
        }
    }

    /// Parse the entire DSL program.
    pub fn parse_program(&mut self) -> Result<Program, ParseError> {
        let mut definitions = Vec::new();
        while self.current_token.kind != TokenKind::Eof {
            if self.current_token.kind == TokenKind::Define {
                definitions.push(self.parse_definition()?);
            } else {
                self.advance()?;
            }
        }
        Ok(Program { definitions })
    }

    /// Parse a single definition (currently only media outlet).
    fn parse_definition(&mut self) -> Result<Definition, ParseError> {
        self.expect(TokenKind::Define)?;
        self.expect(TokenKind::Media)?;
        self.expect(TokenKind::Outlet)?;
        let outlet = self.parse_media_outlet()?;
        Ok(Definition::MediaOutlet(outlet))
    }

    /// Parse a media outlet with name, optional ID, and properties.
    fn parse_media_outlet(&mut self) -> Result<MediaOutlet, ParseError> {
        let position = self.current_token.position.clone();
        let name = match &self.current_token.kind {
            TokenKind::String(s) => {
                let name = s.clone();
                self.advance()?;
                name
            }
            _ => return Err(ParseError::UnexpectedToken(
                self.current_token.kind.clone(),
                position.clone(),
                "string literal (outlet name)".to_string(),
            )),
        };

        self.expect(TokenKind::LBrace)?;

        let mut id = None;
        let mut properties = HashMap::new();

        while self.current_token.kind != TokenKind::RBrace {
            match &self.current_token.kind {
                TokenKind::Id => {
                    self.advance()?;
                    self.expect(TokenKind::Arrow)?;
                    if let TokenKind::Number(n) = self.current_token.kind {
                        id = Some(n);
                        self.advance()?;
                    } else {
                        return Err(ParseError::UnexpectedToken(
                            self.current_token.kind.clone(),
                            self.current_token.position.clone(),
                            "number (id)".to_string(),
                        ));
                    }
                }
                TokenKind::Identifier(key) => {
                    let key = key.clone();
                    self.advance()?;
                    self.expect(TokenKind::Arrow)?;
                    let value = match &self.current_token.kind {
                        TokenKind::String(s) => {
                            let value = PropertyValue::String(s.clone());
                            self.advance()?;
                            value
                        }
                        TokenKind::Number(n) => {
                            let value = PropertyValue::Number(*n);
                            self.advance()?;
                            value
                        }
                        _ => return Err(ParseError::UnexpectedToken(
                            self.current_token.kind.clone(),
                            self.current_token.position.clone(),
                            "string or number (property value)".to_string(),
                        )),
                    };
                    properties.insert(key, value);
                }
                _ => self.advance()?, // Skip unrecognized tokens for now
            }
        }

        self.expect(TokenKind::RBrace)?;
        Ok(MediaOutlet { name, id, properties, position })
    }
}