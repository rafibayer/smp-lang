
use std::error::Error;
use std::fs;

use simple_lang::ast;
use simple_lang::{scanner::Scanner};
use simple_lang::interpreter::Interpreter;


fn main(){
    let file = fs::read_to_string("program.smp").unwrap();
    let mut scanner = Scanner::new(file).unwrap();
    let program = ast::generate_ast(&mut scanner).unwrap();
    let interpreter = Interpreter::new(program);

    println!("Main: {:?}", interpreter.execute().unwrap());
}