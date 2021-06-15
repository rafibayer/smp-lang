
use std::string::FromUtf8Error;

use super::*;

#[derive(Debug)]
pub enum InterpreterError {
    UnboundVar(String),
    UnboundFunc(String),
    TypeError {
        found_type: ValueDiscriminants,
        expected_type: ValueDiscriminants,
    },
    NoMainDefined,
    ArgMismatch {
        got: usize,
        expected: usize,
    },
    ValuelessExpression(Exp),
    DivideByZero,
    IOError(io::Error),
    InvalidInput(ParseFloatError),
    InvalidChar(FromUtf8Error),
    PrecedenceError(ExpKind),
}

impl From<io::Error> for InterpreterError {
    fn from(e: io::Error) -> Self {
        InterpreterError::IOError(e)
    }
}

impl From<ParseFloatError> for InterpreterError {
    fn from(e: ParseFloatError) -> Self {
        InterpreterError::InvalidInput(e)
    }
}

impl From<std::string::FromUtf8Error> for InterpreterError {
    fn from(e: FromUtf8Error) -> Self {
        InterpreterError::InvalidChar(e)
    }
}