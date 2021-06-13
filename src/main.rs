use std::{fs, env};

use smp::ast;
use smp::{scanner::Scanner};
use smp::interpreter::Interpreter;


fn main(){
    let mut scanner = Scanner::new(
        get_source_file(env::args()).unwrap())
            .unwrap();
    let program = ast::generate_ast(&mut scanner).unwrap();
    let interpreter = Interpreter::new(program);
    let result = interpreter.execute().unwrap();
    if result.is_some() {
        println!("Main: {:?}", result.unwrap());
    } else {
        println!("Main: None");

    }
}

fn get_source_file(args: env::Args) -> Result<String, String> {
    if args.len() < 2 {
        return Err(String::from("Usage: smp filename.smp"));
    }

    let args: Vec<String> = args.collect();

    match fs::read_to_string(args[1].clone()) {
        Ok(contents) => Ok(contents),
        Err(err) => Err(err.to_string()),
    }
}
