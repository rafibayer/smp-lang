
use crate::interpreter::symbols::*;
use crate::interpreter::defs::Defs;
use crate::scanner::{Scanner, ScannerError};


pub enum ASTError {
    ScannerError(ScannerError),
    SyntaxError,
}

pub fn generate_ast(scanner: Scanner) -> Result<Program, ASTError> {
    todo!()
}