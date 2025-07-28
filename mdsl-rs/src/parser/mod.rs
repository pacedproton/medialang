//! Parser for the MediaLanguage DSL
//!
//! This module provides syntax analysis, converting tokens into an Abstract Syntax Tree (AST).

pub mod ast;
pub mod error;
pub mod recursive_descent;

// Re-export core types
pub use ast::*;
pub use error::ParseError;
pub use recursive_descent::Parser;

// Tests are in the tests/ directory
