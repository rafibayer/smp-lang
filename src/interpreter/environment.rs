use std::{collections::HashMap, rc::Rc};
use crate::tokens::Token;
use super::{InterpreterError, symbols::Def};


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

    pub fn get_var(&self, name: &String) -> Result<f64, InterpreterError> {
        match self.vars.get(name) {
            Some(val) => Ok(*val),
            None => Err(InterpreterError::UnboundVar(name.clone())),
        }
    }

   
}