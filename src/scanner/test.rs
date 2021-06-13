
#[cfg(test)]
mod test {

    use super::super::*;

    #[test]
    fn one_plus_one() {
        let mut s = Scanner::new(String::from("1+1")).unwrap();
        assert_eq!(s.next_token().unwrap(), Token::Num(1.0));
        assert_eq!(s.next_token().unwrap(), Token::Plus);
        assert_eq!(s.next_token().unwrap(), Token::Num(1.0));
        assert_eq!(s.next_token().unwrap(), Token::Eof);
        assert_eq!(s.next_token().unwrap(), Token::Eof);
    }

    #[test]
    fn decimals() {
        let mut s = Scanner::new(String::from("1.5+1")).unwrap();
        assert_eq!(s.next_token().unwrap(), Token::Num(1.5));
        assert_eq!(s.next_token().unwrap(), Token::Plus);
        assert_eq!(s.next_token().unwrap(), Token::Num(1.0));
        assert_eq!(s.next_token().unwrap(), Token::Eof);

    }

    #[test]
    fn arrays() {
        let s = Scanner::new(String::from(r#"
        a := [5];
        "#)).unwrap();
        let expected = vec![
            Token::Name(String::from("a")),
            Token::Assign,
            Token::LBracket,
            Token::Num(5f64),
            Token::RBracket,
            Token::SColon,
        ];

        let mut actual = Vec::new();
        for token in s.into_iter() {
            actual.push(token.unwrap());
        }

        assert_eq!(expected, actual);
    }

    #[test]
    fn scan_program() {
        let s = Scanner::new(String::from(r#"
        def main() {
            if (1 + 1) == 2 {
                return 1; 
            }
            return 0
        }
        "#)).unwrap();
        let expected = vec![
            Token::Def,
            Token::Name(String::from("main")),
            Token::LParen,
            Token::RParen,
            Token::LCurly,
            Token::If,
            Token::LParen,
            Token::Num(1f64),
            Token::Plus,
            Token::Num(1f64),
            Token::RParen,
            Token::Equals,
            Token::Num(2f64),
            Token::LCurly,
            Token::Return,
            Token::Num(1f64),
            Token::SColon,
            Token::RCurly,
            Token::Return,
            Token::Num(0f64),
            Token::RCurly,
        ];

        let mut actual = Vec::new();
        for token in s.into_iter() {
            actual.push(token.unwrap());
        }

        assert_eq!(expected, actual);
        
    }
}