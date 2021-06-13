

#[cfg(test)]
mod test {

    use super::super::*;


    #[test]
    fn test_simple_ast() {
        let mut scan = Scanner::new(
            String::from(r#"

            def add(a, b) {
                return a;
            }
           
            def main() {
               return add(1, 2);
            }
            "#)).unwrap();
        
        let prog = generate_ast(&mut scan);

        println!("{:#?}", prog.unwrap());
        
    }
}