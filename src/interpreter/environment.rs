use super::InterpreterError;
use std::collections::HashMap;

#[derive(Debug, EnumDiscriminants, Display, Clone, PartialEq, PartialOrd)]
pub enum Value {
    Num(f64),
    Arr(Vec<f64>),
}

impl Value {
    pub fn into_f64(self) -> Result<f64, InterpreterError> {
        Ok(match self {
            Value::Num(val) => val,
            Value::Arr(_) => {
                return Err(InterpreterError::TypeError {
                    found_type: ValueDiscriminants::Arr,
                    expected_type: ValueDiscriminants::Num,
                })
            }
        })
    }

    pub fn into_vec(self) -> Result<Vec<f64>, InterpreterError> {
        Ok(match self {
            Value::Arr(val) => val,
            Value::Num(_) => {
                return Err(InterpreterError::TypeError {
                    found_type: ValueDiscriminants::Num,
                    expected_type: ValueDiscriminants::Arr,
                })
            }
        })
    }
}

impl From<f64> for Value {
    fn from(val: f64) -> Self {
        Value::Num(val)
    }
}

impl From<Vec<f64>> for Value {
    fn from(val: Vec<f64>) -> Self {
        Value::Arr(val)
    }
}

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

    pub fn bind_var(&mut self, name: String, value: Value) {
        self.vars.insert(name, value);
    }

    pub fn get_var(&self, name: &str) -> Result<Value, InterpreterError> {
        match self.vars.get(name) {
            Some(val) => Ok(val.clone()),
            None => Err(InterpreterError::UnboundVar(name.to_string())),
        }
    }
}
