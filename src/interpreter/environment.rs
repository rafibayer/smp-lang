use super::InterpreterError;
use std::{collections::HashMap, fmt::Display};

// Value represents a value in an smp program.
#[derive(Debug, EnumDiscriminants, Clone, PartialEq, PartialOrd)]
pub enum Value {
    Num(f64),
    Array(Vec<f64>),
}

impl Value {
    // tries to convert a Value into an f64
    pub fn into_f64(self) -> Result<f64, InterpreterError> {
        Ok(match self {
            Value::Num(val) => val,
            Value::Array(_) => {
                return Err(InterpreterError::TypeError {
                    found_type: ValueDiscriminants::Array,
                    expected_type: ValueDiscriminants::Num,
                })
            }
        })
    }

    // tries to convert a Value into an Vec<f64>
    pub fn into_vec(self) -> Result<Vec<f64>, InterpreterError> {
        Ok(match self {
            Value::Array(val) => val,
            Value::Num(_) => {
                return Err(InterpreterError::TypeError {
                    found_type: ValueDiscriminants::Num,
                    expected_type: ValueDiscriminants::Array,
                })
            }
        })
    }
}

// Creates a Value from an f64
impl From<f64> for Value {
    fn from(val: f64) -> Self {
        Value::Num(val)
    }
}

// Creates a Value from a Vec<f64>
impl From<Vec<f64>> for Value {
    fn from(val: Vec<f64>) -> Self {
        Value::Array(val)
    }
}

// Display for Value, used for program output
impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Num(val) => write!(f, "{}", val),
            Value::Array(val) => write!(f, "{:?}", val),
        }
    }
}

// Environment holds variable bindings
#[derive(Debug)]
pub struct Environment {
    vars: HashMap<String, Value>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            vars: HashMap::new(),
        }
    }

    // Binds a value to a name
    pub fn bind_var(&mut self, name: String, value: Value) {
        self.vars.insert(name, value);
    }

    // Retrieves the value bound to a name
    pub fn get_var(&self, name: &str) -> Result<Value, InterpreterError> {
        match self.vars.get(name) {
            Some(val) => Ok(val.clone()),
            None => Err(InterpreterError::UnboundVar(name.to_string())),
        }
    }
}
