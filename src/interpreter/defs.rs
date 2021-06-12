use std::{collections::HashMap, rc::Rc};

use super::{InterpreterError, symbols::Def};



#[derive(Debug, Clone)]
pub struct Defs {
    funcs: HashMap<String, Rc<Def>>
}

impl Defs {
    pub fn new() -> Defs {
        Defs {
            funcs: HashMap::new()
        }
    }

    pub fn bind_func(&mut self, name: String, value: Rc<Def>) {
        self.funcs.insert(name, value);
    }
    
    pub fn get_func(&self, name: &String) -> Result<&Def, InterpreterError> {
        match self.funcs.get(name) {
            Some(val) => Ok(val),
            None => Err(InterpreterError::UnboundFunc(name.clone())),
        }
    }

}

