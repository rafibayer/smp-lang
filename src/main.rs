
use simple_lang::{scanner::Scanner, tokens::Token};


fn main() {

    // repl loop
    loop {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line)
            .expect("Failed to read line");

        let scan = Scanner::new(line).unwrap();

        for token in scan.into_iter() {
            print!("{:?} ", token.unwrap());
        }
        println!();
    }
    

    
}