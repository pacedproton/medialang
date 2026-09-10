//! Semantic analysis for the MediaLanguage DSL

pub mod symbol_table;
pub mod type_checker;
pub mod validator;

// Re-export key types for convenience
pub use validator::{
    validate_program, ValidationIssue, ValidationReporter, ValidationResult, ValidationSeverity,
    ValidationSummary, Validator,
};
