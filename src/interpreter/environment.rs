use super::InterpreterError;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Environment {
    vars: HashMap<String, f64>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            vars: HashMap::new(),
        }
    }

    pub fn bind_var(&mut self, name: String, value: f64) {
        self.vars.insert(name, value);
    }

    pub fn get_var(&self, name: &str) -> Result<f64, InterpreterError> {
        match self.vars.get(name) {
            Some(val) => Ok(*val),
            None => Err(InterpreterError::UnboundVar(name.to_string())),
        }
    }
}
