
use simple_lang::{scanner::Scanner, tokens::Token};


fn main() {

    // repl loop
    loop {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line)
            .expect("Failed to read line");

        let mut scan = Scanner::new(line);

        // print tokens until eof
        loop {
            let token = scan.next_token().unwrap();
            match token {
                Token::Eof => break,
                other => print!("{:?} ", other),
            }
        }
        println!();
    }
    

    
}