
pub mod scanner;
pub mod tokens;


#[cfg(test)]
mod test {

    use super::*;
    use scanner::{Scanner, ScannerError};
    use tokens::Token;

    #[test]
    fn one_plus_one() {
        let mut s = Scanner::new(String::from("1+1"));
        assert_eq!(s.next_token().unwrap(), Token::Num(1.0));
        assert_eq!(s.next_token().unwrap(), Token::Plus);
        assert_eq!(s.next_token().unwrap(), Token::Num(1.0));
        assert_eq!(s.next_token().unwrap(), Token::Eof);
        assert_eq!(s.next_token().unwrap(), Token::Eof);
    }

    #[test]
    fn decimals() {
        let mut s = Scanner::new(String::from("1.5+1"));
        assert_eq!(s.next_token().unwrap(), Token::Num(1.5));
        assert_eq!(s.next_token().unwrap(), Token::Plus);
        assert_eq!(s.next_token().unwrap(), Token::Num(1.0));
        assert_eq!(s.next_token().unwrap(), Token::Eof);

    }
}