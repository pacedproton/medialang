//! Intermediate representation

pub mod nodes;
pub mod transformer;

use crate::error::Result;
use crate::parser::ast::Program;

/// Transform AST to IR
pub fn transform(ast: &Program) -> Result<nodes::IRProgram> {
    transformer::transform(ast)
}
