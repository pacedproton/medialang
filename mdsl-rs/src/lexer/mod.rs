//! Lexical analysis for the MediaLanguage DSL
//!
//! This module provides tokenization of MediaLanguage DSL source code.
//! It handles keywords, identifiers, literals, operators, and comments.

pub mod scanner;
pub mod token;

// Re-export core types
pub use scanner::Lexer;
pub use token::{Keyword, Literal, Token, TokenKind};

// Also export Scanner as an alias for compatibility
pub use scanner::Lexer as Scanner;
