//! # MediaLanguage DSL - Rust Implementation
//!
//! This library provides a complete implementation of the MediaLanguage DSL for modeling
//! media outlets, companies, and their relationships. It's designed as a tutorial project
//! to demonstrate DSL implementation in Rust.
//!
//! ## Architecture
//!
//! The library follows a multi-pass architecture:
//! 1. **Lexer**: Tokenizes source code into structured tokens
//! 2. **Parser**: Builds an Abstract Syntax Tree (AST) from tokens
//! 3. **Semantic Analysis**: Validates semantics and resolves symbols
//! 4. **Intermediate Representation**: Normalizes the AST for code generation
//! 5. **Code Generation**: Generates SQL and Cypher from the IR
//!
//! ## Usage
//!
//! ```rust
//! use mdsl_rs::{lexer::Lexer, parser::Parser, codegen::SqlGenerator, ir};
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let source = r#"
//! UNIT MediaOutlet {
//!     id: ID PRIMARY KEY,
//!     name: TEXT(120),
//!     sector: NUMBER
//! }
//! "#;
//!
//! let mut lexer = Lexer::new(source);
//! let tokens = lexer.tokenize()?;
//! let mut parser = Parser::new(tokens);
//! let ast = parser.parse()?;
//! let ir = ir::transform(&ast)?;
//! let sql_generator = SqlGenerator::new();
//! let sql = sql_generator.generate(&ir)?;
//! # Ok(())
//! # }
//! ```

#![deny(missing_docs)]
#![deny(unsafe_code)]
#![warn(clippy::all)]
#![warn(rust_2018_idioms)]

// Re-export core types for convenience
pub use error::{Error, Result};

/// Error handling types and utilities
pub mod error;

/// Lexical analysis - converts source code into tokens
pub mod lexer;

/// Syntax analysis - converts tokens into an Abstract Syntax Tree
pub mod parser;

/// Semantic analysis - validates and enriches the AST
pub mod semantic;

/// Intermediate representation - normalized form for code generation
pub mod ir;

/// Code generation - generates SQL and Cypher from IR
pub mod codegen;

/// SQL to MDSL import functionality
#[cfg(feature = "import")]
pub mod import;

/// Interactive REPL for testing and development
#[cfg(feature = "repl")]
// pub mod repl;

/// Utility functions and types
pub mod utils;

/// Neo4j validation and testing
#[cfg(feature = "neo4j")]
pub mod neo4j;

/// Library version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Semantic analysis - validates and enriches the AST
pub use semantic::{validate_program, ValidationResult, ValidationSeverity};

/// Parse a MediaLanguage DSL source string into an AST
///
/// This is a convenience function that combines lexing and parsing.
///
/// # Arguments
///
/// * `source` - The MediaLanguage DSL source code as a string
///
/// # Returns
///
/// Returns a `Result` containing the parsed AST or an error if parsing fails.
///
/// # Examples
///
/// ```rust
/// use mdsl_rs::parse;
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let source = r#"
/// UNIT MediaOutlet {
///     id: ID PRIMARY KEY,
///     name: TEXT(120)
/// }
/// "#;
///
/// let ast = parse(source)?;
/// # Ok(())
/// # }
/// ```
pub fn parse(source: &str) -> Result<parser::ast::Program> {
    let mut lexer = lexer::Lexer::new(source);
    let tokens = lexer.tokenize()?;
    let mut parser = parser::Parser::new(tokens);
    parser.parse()
}

/// Compile MediaLanguage DSL source to SQL
///
/// This function provides a complete pipeline from source code to SQL generation.
///
/// # Arguments
///
/// * `source` - The MediaLanguage DSL source code
///
/// # Returns
///
/// Returns a `Result` containing the generated SQL or an error.
#[cfg(feature = "sql-codegen")]
pub fn compile_to_sql(source: &str) -> Result<String> {
    let ast = parse(source)?;
    let ir = ir::transform(&ast)?;
    let sql_generator = codegen::SqlGenerator::new();
    sql_generator.generate(&ir)
}

/// Compile MediaLanguage DSL source to Cypher
///
/// This function provides a complete pipeline from source code to Cypher generation.
///
/// # Arguments
///
/// * `source` - The MediaLanguage DSL source code
///
/// # Returns
///
/// Returns a `Result` containing the generated Cypher or an error.
#[cfg(feature = "cypher-codegen")]
pub fn compile_to_cypher(source: &str) -> Result<String> {
    let ast = parse(source)?;
    let ir = ir::transform(&ast)?;
    let cypher_generator = codegen::CypherGenerator::new();
    cypher_generator.generate(&ir)
}
