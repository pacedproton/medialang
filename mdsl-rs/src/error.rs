//! Error handling for the MediaLanguage DSL implementation
//!
//! This module provides comprehensive error types and utilities for all phases
//! of the DSL implementation, from lexical analysis to code generation.

use std::fmt;

/// Main error type for the MediaLanguage DSL
#[derive(Debug, Clone)]
pub enum Error {
    /// Lexical analysis errors
    Lexer(LexerError),
    /// Parsing errors
    Parser(ParserError),
    /// Semantic analysis errors
    Semantic(SemanticError),
    /// Code generation errors
    CodeGen(CodeGenError),
    /// I/O errors
    Io(String),
    /// Database import errors
    Import(ImportError),
    /// Invalid connection string
    InvalidConnectionString(String),
    /// Feature not implemented
    NotImplemented(String),
    /// Database connection error
    DatabaseConnection(String),
    /// Database query error
    DatabaseQuery(String),
}

/// Lexical analysis error types
#[derive(Debug, Clone)]
pub enum LexerError {
    /// Unexpected character encountered
    UnexpectedCharacter {
        /// The unexpected character
        character: char,
        /// Position in source
        position: SourcePosition,
    },
    /// Unterminated string literal
    UnterminatedString {
        /// Position where string started
        position: SourcePosition,
    },
    /// Invalid number format
    InvalidNumber {
        /// The invalid number text
        text: String,
        /// Position in source
        position: SourcePosition,
    },
    /// Invalid escape sequence in string
    InvalidEscape {
        /// The invalid escape sequence
        sequence: String,
        /// Position in source
        position: SourcePosition,
    },
}

/// Parser error types
#[derive(Debug, Clone)]
pub enum ParserError {
    /// Unexpected token encountered
    UnexpectedToken {
        /// The unexpected token
        found: String,
        /// Expected token(s)
        expected: Vec<String>,
        /// Position in source
        position: SourcePosition,
    },
    /// Missing closing delimiter
    MissingClosingDelimiter {
        /// The expected delimiter
        delimiter: String,
        /// Position where it was expected
        position: SourcePosition,
    },
    /// Invalid syntax
    InvalidSyntax {
        /// Description of the syntax error
        message: String,
        /// Position in source
        position: SourcePosition,
    },
    /// Unexpected end of input
    UnexpectedEof {
        /// What was expected
        expected: String,
        /// Position where EOF occurred
        position: SourcePosition,
    },
}

/// Semantic analysis error types
#[derive(Debug, Clone)]
pub enum SemanticError {
    /// Undefined variable reference
    UndefinedVariable {
        /// Variable name
        name: String,
        /// Position in source
        position: SourcePosition,
    },
    /// Duplicate definition
    DuplicateDefinition {
        /// Name being redefined
        name: String,
        /// Position of new definition
        position: SourcePosition,
        /// Position of original definition
        original_position: SourcePosition,
    },
    /// Type mismatch
    TypeMismatch {
        /// Expected type
        expected: String,
        /// Found type
        found: String,
        /// Position in source
        position: SourcePosition,
    },
    /// Invalid field reference
    InvalidField {
        /// Field name
        field: String,
        /// Type name
        type_name: String,
        /// Position in source
        position: SourcePosition,
    },
    /// Circular dependency
    CircularDependency {
        /// Names involved in the cycle
        cycle: Vec<String>,
        /// Position in source
        position: SourcePosition,
    },
    /// Import resolution error
    ImportError {
        /// Import path
        path: String,
        /// Error message
        message: String,
        /// Position in source
        position: SourcePosition,
    },
}

/// Code generation error types
#[derive(Debug, Clone)]
pub enum CodeGenError {
    /// Unsupported feature
    UnsupportedFeature {
        /// Feature name
        feature: String,
        /// Target (SQL/Cypher)
        target: String,
        /// Position in source
        position: SourcePosition,
    },
    /// Invalid target configuration
    InvalidTarget {
        /// Target name
        target: String,
        /// Error message
        message: String,
    },
    /// Generation failure
    GenerationFailure {
        /// Error message
        message: String,
        /// Position in source
        position: SourcePosition,
    },
}

/// Source position information for error reporting
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SourcePosition {
    /// Line number (1-based)
    pub line: usize,
    /// Column number (1-based)
    pub column: usize,
    /// Byte offset in source
    pub offset: usize,
}

impl SourcePosition {
    /// Create a new source position
    pub fn new(line: usize, column: usize, offset: usize) -> Self {
        Self {
            line,
            column,
            offset,
        }
    }

    /// Create a source position at the start of input
    pub fn start() -> Self {
        Self::new(1, 1, 0)
    }
}

impl fmt::Display for SourcePosition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}

/// Result type for DSL operations
pub type Result<T> = std::result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Lexer(e) => write!(f, "Lexer error: {}", e),
            Error::Parser(e) => write!(f, "Parser error: {}", e),
            Error::Semantic(e) => write!(f, "Semantic error: {}", e),
            Error::CodeGen(e) => write!(f, "Code generation error: {}", e),
            Error::Io(e) => write!(f, "I/O error: {}", e),
            Error::Import(e) => write!(f, "Import error: {}", e),
            Error::InvalidConnectionString(s) => write!(f, "Invalid connection string: {}", s),
            Error::NotImplemented(s) => write!(f, "Not implemented: {}", s),
            Error::DatabaseConnection(s) => write!(f, "Database connection error: {}", s),
            Error::DatabaseQuery(s) => write!(f, "Database query error: {}", s),
        }
    }
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LexerError::UnexpectedCharacter {
                character,
                position,
            } => {
                write!(f, "Unexpected character '{}' at {}", character, position)
            }
            LexerError::UnterminatedString { position } => {
                write!(f, "Unterminated string literal at {}", position)
            }
            LexerError::InvalidNumber { text, position } => {
                write!(f, "Invalid number '{}' at {}", text, position)
            }
            LexerError::InvalidEscape { sequence, position } => {
                write!(f, "Invalid escape sequence '{}' at {}", sequence, position)
            }
        }
    }
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParserError::UnexpectedToken {
                found,
                expected,
                position,
            } => {
                write!(
                    f,
                    "Unexpected token '{}' at {}, expected {}",
                    found,
                    position,
                    expected.join(" or ")
                )
            }
            ParserError::MissingClosingDelimiter {
                delimiter,
                position,
            } => {
                write!(f, "Missing closing '{}' at {}", delimiter, position)
            }
            ParserError::InvalidSyntax { message, position } => {
                write!(f, "Invalid syntax at {}: {}", position, message)
            }
            ParserError::UnexpectedEof { expected, position } => {
                write!(
                    f,
                    "Unexpected end of input at {}, expected {}",
                    position, expected
                )
            }
        }
    }
}

impl fmt::Display for SemanticError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SemanticError::UndefinedVariable { name, position } => {
                write!(f, "Undefined variable '{}' at {}", name, position)
            }
            SemanticError::DuplicateDefinition {
                name,
                position,
                original_position,
            } => {
                write!(
                    f,
                    "Duplicate definition of '{}' at {}, originally defined at {}",
                    name, position, original_position
                )
            }
            SemanticError::TypeMismatch {
                expected,
                found,
                position,
            } => {
                write!(
                    f,
                    "Type mismatch at {}: expected {}, found {}",
                    position, expected, found
                )
            }
            SemanticError::InvalidField {
                field,
                type_name,
                position,
            } => {
                write!(
                    f,
                    "Invalid field '{}' for type '{}' at {}",
                    field, type_name, position
                )
            }
            SemanticError::CircularDependency { cycle, position } => {
                write!(
                    f,
                    "Circular dependency at {}: {}",
                    position,
                    cycle.join(" -> ")
                )
            }
            SemanticError::ImportError {
                path,
                message,
                position,
            } => {
                write!(
                    f,
                    "Import error for '{}' at {}: {}",
                    path, position, message
                )
            }
        }
    }
}

impl fmt::Display for CodeGenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CodeGenError::UnsupportedFeature {
                feature,
                target,
                position,
            } => {
                write!(
                    f,
                    "Unsupported feature '{}' for target '{}' at {}",
                    feature, target, position
                )
            }
            CodeGenError::InvalidTarget { target, message } => {
                write!(f, "Invalid target '{}': {}", target, message)
            }
            CodeGenError::GenerationFailure { message, position } => {
                write!(f, "Generation failure at {}: {}", position, message)
            }
        }
    }
}

/// Import error types for database import functionality
#[derive(Debug, Clone)]
pub enum ImportError {
    /// Database connection failed
    ConnectionFailed {
        /// Error message
        message: String,
    },
    /// Database query failed
    QueryFailed {
        /// SQL query that failed
        query: String,
        /// Error message
        message: String,
    },
    /// Schema analysis failed
    SchemaAnalysisFailed {
        /// Error message
        message: String,
    },
    /// Data mapping failed
    MappingFailed {
        /// Table name
        table: String,
        /// Error message
        message: String,
    },
    /// MDSL generation failed
    GenerationFailed {
        /// Error message
        message: String,
    },
    /// Unsupported database type
    UnsupportedDatabaseType {
        /// Database type
        database_type: String,
    },
}

impl fmt::Display for ImportError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ImportError::ConnectionFailed { message } => {
                write!(f, "Database connection failed: {}", message)
            }
            ImportError::QueryFailed { query, message } => {
                write!(f, "Query failed: {}\nQuery: {}", message, query)
            }
            ImportError::SchemaAnalysisFailed { message } => {
                write!(f, "Schema analysis failed: {}", message)
            }
            ImportError::MappingFailed { table, message } => {
                write!(f, "Data mapping failed for table '{}': {}", table, message)
            }
            ImportError::GenerationFailed { message } => {
                write!(f, "MDSL generation failed: {}", message)
            }
            ImportError::UnsupportedDatabaseType { database_type } => {
                write!(f, "Unsupported database type: {}", database_type)
            }
        }
    }
}

impl std::error::Error for Error {}
impl std::error::Error for LexerError {}
impl std::error::Error for ParserError {}
impl std::error::Error for SemanticError {}
impl std::error::Error for CodeGenError {}
impl std::error::Error for ImportError {}

// Convenience conversion functions
impl From<LexerError> for Error {
    fn from(err: LexerError) -> Self {
        Error::Lexer(err)
    }
}

impl From<ParserError> for Error {
    fn from(err: ParserError) -> Self {
        Error::Parser(err)
    }
}

impl From<SemanticError> for Error {
    fn from(err: SemanticError) -> Self {
        Error::Semantic(err)
    }
}

impl From<CodeGenError> for Error {
    fn from(err: CodeGenError) -> Self {
        Error::CodeGen(err)
    }
}

impl From<ImportError> for Error {
    fn from(err: ImportError) -> Self {
        Error::Import(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Io(err.to_string())
    }
}

// Add From implementations for better error handling
impl From<std::fmt::Error> for Error {
    fn from(err: std::fmt::Error) -> Self {
        Error::Io(err.to_string())
    }
}
