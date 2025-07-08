//! Lexical scanner for the MediaLanguage DSL
//!
//! This module provides the core lexical analysis functionality, converting
//! source code into a stream of tokens.

use super::token::{Keyword, Token, TokenKind};
use crate::error::{LexerError, Result, SourcePosition};
use std::iter::Peekable;
use std::str::Chars;

/// Lexical scanner for MediaLanguage DSL
pub struct Lexer<'a> {
    /// Character iterator
    chars: Peekable<Chars<'a>>,
    /// Current position in source
    position: SourcePosition,
    /// Current character
    current_char: Option<char>,
    /// Tokens collected so far
    tokens: Vec<Token>,
}

impl<'a> Lexer<'a> {
    /// Create a new lexer for the given source code
    pub fn new(source: &'a str) -> Self {
        let mut lexer = Self {
            chars: source.chars().peekable(),
            position: SourcePosition::start(),
            current_char: None,
            tokens: Vec::new(),
        };
        lexer.advance(); // Initialize current_char
        lexer
    }

    /// Tokenize the entire source code
    pub fn tokenize(&mut self) -> Result<Vec<Token>> {
        self.tokens.clear();

        while let Some(ch) = self.current_char {
            match ch {
                // Whitespace (skip)
                ' ' | '\t' | '\r' => {
                    self.advance();
                }

                // Newlines (preserve for statement separation)
                '\n' => {
                    self.add_token(TokenKind::Newline);
                    self.advance();
                }

                // Single-character tokens
                '=' => {
                    self.add_token(TokenKind::Assign);
                    self.advance();
                }
                ';' => {
                    self.add_token(TokenKind::Semicolon);
                    self.advance();
                }
                ':' => {
                    self.add_token(TokenKind::Colon);
                    self.advance();
                }
                ',' => {
                    self.add_token(TokenKind::Comma);
                    self.advance();
                }
                '.' => {
                    self.add_token(TokenKind::Dot);
                    self.advance();
                }
                '$' => {
                    self.add_token(TokenKind::Dollar);
                    self.advance();
                }
                '{' => {
                    self.add_token(TokenKind::LeftBrace);
                    self.advance();
                }
                '}' => {
                    self.add_token(TokenKind::RightBrace);
                    self.advance();
                }
                '(' => {
                    self.add_token(TokenKind::LeftParen);
                    self.advance();
                }
                ')' => {
                    self.add_token(TokenKind::RightParen);
                    self.advance();
                }
                '[' => {
                    self.add_token(TokenKind::LeftBracket);
                    self.advance();
                }
                ']' => {
                    self.add_token(TokenKind::RightBracket);
                    self.advance();
                }
                '<' => {
                    self.add_token(TokenKind::LeftAngle);
                    self.advance();
                }
                '>' => {
                    self.add_token(TokenKind::RightAngle);
                    self.advance();
                }

                // Comments
                '/' => {
                    if self.peek() == Some('/') {
                        self.scan_line_comment()?;
                    } else if self.peek() == Some('*') {
                        self.scan_block_comment()?;
                    } else {
                        return Err(LexerError::UnexpectedCharacter {
                            character: ch,
                            position: self.position,
                        }
                        .into());
                    }
                }

                // Annotations
                '@' => {
                    self.scan_annotation()?;
                }

                // Hash comments (alternative syntax)
                '#' => {
                    self.scan_hash_comment()?;
                }

                // String literals
                '"' => {
                    self.scan_string()?;
                }

                // Numbers
                '0'..='9' => {
                    self.scan_number()?;
                }

                // Identifiers and keywords
                'a'..='z' | 'A'..='Z' | '_' => {
                    self.scan_identifier()?;
                }

                // Unexpected character
                _ => {
                    return Err(LexerError::UnexpectedCharacter {
                        character: ch,
                        position: self.position,
                    }
                    .into());
                }
            }
        }

        // Add EOF token
        self.add_token(TokenKind::Eof);
        Ok(self.tokens.clone())
    }

    /// Compatibility method for scan_tokens
    pub fn scan_tokens(&mut self) -> Result<Vec<Token>> {
        self.tokenize()
    }

    /// Advance to the next character
    fn advance(&mut self) {
        if let Some(ch) = self.current_char {
            self.position.offset += ch.len_utf8();
            if ch == '\n' {
                self.position.line += 1;
                self.position.column = 1;
            } else {
                self.position.column += 1;
            }
        }
        self.current_char = self.chars.next();
    }

    /// Peek at the next character without advancing
    fn peek(&mut self) -> Option<char> {
        self.chars.peek().copied()
    }

    /// Add a token to the token stream
    fn add_token(&mut self, kind: TokenKind) {
        let text = match &kind {
            TokenKind::Newline => "\n".to_string(),
            TokenKind::Eof => "".to_string(),
            _ => self
                .current_char
                .map_or_else(String::new, |c| c.to_string()),
        };

        self.tokens.push(Token::new(kind, text, self.position));
    }

    /// Add a token with specific text
    /// Add a token with custom text (unused but kept for future use)
    #[allow(dead_code)]
    fn add_token_with_text(&mut self, kind: TokenKind, text: String) {
        self.tokens.push(Token::new(kind, text, self.position));
    }

    /// Scan a line comment (// comment)
    fn scan_line_comment(&mut self) -> Result<()> {
        let start_pos = self.position;

        // Skip the '//'
        self.advance(); // first '/'
        self.advance(); // second '/'

        let mut comment_text = String::new();
        while let Some(ch) = self.current_char {
            if ch == '\n' {
                break;
            }
            comment_text.push(ch);
            self.advance();
        }

        self.tokens.push(Token::new(
            TokenKind::Comment(comment_text.trim().to_string()),
            format!("//{}", comment_text),
            start_pos,
        ));

        Ok(())
    }

    /// Scan a block comment (/* comment */)
    fn scan_block_comment(&mut self) -> Result<()> {
        let start_pos = self.position;

        // Skip the '/*'
        self.advance(); // '/'
        self.advance(); // '*'

        let mut comment_text = String::new();
        let mut found_end = false;

        while let Some(ch) = self.current_char {
            if ch == '*' && self.peek() == Some('/') {
                self.advance(); // '*'
                self.advance(); // '/'
                found_end = true;
                break;
            }
            comment_text.push(ch);
            self.advance();
        }

        if !found_end {
            return Err(LexerError::UnterminatedString {
                position: start_pos,
            }
            .into());
        }

        self.tokens.push(Token::new(
            TokenKind::MultiLineComment(comment_text.clone()),
            format!("/*{}*/", comment_text),
            start_pos,
        ));

        Ok(())
    }

    /// Scan a hash comment (# comment)
    fn scan_hash_comment(&mut self) -> Result<()> {
        let start_pos = self.position;

        // Skip the '#'
        self.advance();

        let mut comment_text = String::new();
        while let Some(ch) = self.current_char {
            if ch == '\n' {
                break;
            }
            comment_text.push(ch);
            self.advance();
        }

        self.tokens.push(Token::new(
            TokenKind::Comment(comment_text.trim().to_string()),
            format!("#{}", comment_text),
            start_pos,
        ));

        Ok(())
    }

    /// Scan an annotation (@annotation)
    fn scan_annotation(&mut self) -> Result<()> {
        let start_pos = self.position;

        // Skip the '@'
        self.advance();

        let mut annotation_text = String::new();
        while let Some(ch) = self.current_char {
            if ch.is_alphanumeric() || ch == '_' {
                annotation_text.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        self.tokens.push(Token::new(
            TokenKind::Annotation(annotation_text.clone()),
            format!("@{}", annotation_text),
            start_pos,
        ));

        Ok(())
    }

    /// Scan a string literal
    fn scan_string(&mut self) -> Result<()> {
        let start_pos = self.position;

        // Skip opening quote
        self.advance();

        let mut string_value = String::new();
        let mut found_closing = false;

        while let Some(ch) = self.current_char {
            match ch {
                '"' => {
                    found_closing = true;
                    self.advance(); // Skip closing quote
                    break;
                }
                '\\' => {
                    // Handle escape sequences
                    self.advance();
                    if let Some(escaped) = self.current_char {
                        match escaped {
                            'n' => string_value.push('\n'),
                            't' => string_value.push('\t'),
                            'r' => string_value.push('\r'),
                            '\\' => string_value.push('\\'),
                            '"' => string_value.push('"'),
                            _ => {
                                return Err(LexerError::InvalidEscape {
                                    sequence: format!("\\{}", escaped),
                                    position: self.position,
                                }
                                .into());
                            }
                        }
                        self.advance();
                    }
                }
                '\n' => {
                    return Err(LexerError::UnterminatedString {
                        position: start_pos,
                    }
                    .into());
                }
                _ => {
                    string_value.push(ch);
                    self.advance();
                }
            }
        }

        if !found_closing {
            return Err(LexerError::UnterminatedString {
                position: start_pos,
            }
            .into());
        }

        self.tokens.push(Token::new(
            TokenKind::String(string_value.clone()),
            format!("\"{}\"", string_value),
            start_pos,
        ));

        Ok(())
    }

    /// Scan a number literal
    fn scan_number(&mut self) -> Result<()> {
        let start_pos = self.position;
        let mut number_text = String::new();

        // Scan integer part
        while let Some(ch) = self.current_char {
            if ch.is_ascii_digit() {
                number_text.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        // Check for decimal point
        if self.current_char == Some('.') && self.peek().map_or(false, |c| c.is_ascii_digit()) {
            number_text.push('.');
            self.advance();

            // Scan fractional part
            while let Some(ch) = self.current_char {
                if ch.is_ascii_digit() {
                    number_text.push(ch);
                    self.advance();
                } else {
                    break;
                }
            }
        }

        // Parse the number
        let number_value: f64 = number_text.parse().map_err(|_| LexerError::InvalidNumber {
            text: number_text.clone(),
            position: start_pos,
        })?;

        self.tokens.push(Token::new(
            TokenKind::Number(number_value),
            number_text,
            start_pos,
        ));

        Ok(())
    }

    /// Scan an identifier or keyword
    fn scan_identifier(&mut self) -> Result<()> {
        let start_pos = self.position;
        let mut identifier_text = String::new();

        // Scan the identifier
        while let Some(ch) = self.current_char {
            if ch.is_alphanumeric() || ch == '_' || ch == '.' {
                identifier_text.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        // Check if it's a keyword
        let token_kind = if let Some(keyword) = Keyword::from_str(&identifier_text) {
            // Handle boolean literals specially
            match keyword {
                Keyword::True => TokenKind::Boolean(true),
                Keyword::False => TokenKind::Boolean(false),
                _ => TokenKind::Keyword(keyword),
            }
        } else {
            TokenKind::Identifier(identifier_text.clone())
        };

        self.tokens
            .push(Token::new(token_kind, identifier_text, start_pos));

        Ok(())
    }
}
