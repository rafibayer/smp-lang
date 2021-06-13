use std::{fs, env};

use simple_lang::ast;
use simple_lang::{scanner::Scanner};
use simple_lang::interpreter::Interpreter;


fn main(){
    let mut scanner = Scanner::new(
        get_source_file(env::args()).unwrap())
            .unwrap();
    let program = ast::generate_ast(&mut scanner).unwrap();
    let interpreter = Interpreter::new(program);
    println!("Main: {:?}", interpreter.execute().unwrap());
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
