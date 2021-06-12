use crate::tokens::Token;


#[derive(Debug)]
pub enum ScannerError {
    UnexpectedToken(String),
    UnexpectedEof,
    InvalidNumeric(String),
}

const EOF_CHAR: char = '\0';

#[derive(Debug)]
pub struct Scanner {
    input: String,
    cur: usize,
    next: Token,
}

impl Scanner {

    pub fn new(input: String) -> Result<Scanner, ScannerError> {
        let mut scanner = 
        Scanner {
            input,
            cur: 0,
            next: Token::Start,
        };

        scanner.next_token()?;
        Ok(scanner)
    }

    pub fn is_eof(&self) -> bool {
        self.cur >= self.input.len()
    }

    pub fn peek_next(&self) -> Token {
        self.next.clone()
    }
    
    pub fn next_token(&mut self) -> Result<Token, ScannerError> {
        let result = self.next.clone();
        self.next = self.get_next()?;
        Ok(result)
    }

    // returns the next recognized token in the input
    pub fn get_next(&mut self) -> Result<Token, ScannerError> {
        self.skip_whitespace();

        if self.is_eof() {
            return Ok(Token::Eof);
        }

        match self.get_char() {
            // single-char tokens
            '(' => {
                self.advance();
                Ok(Token::LParen)
            }
            ')' => {
                self.advance();
                Ok(Token::RParen)
            }
            ',' => {
                self.advance();
                Ok(Token::Comma)
            }
            '{' => {
                self.advance();
                Ok(Token::LCurly)
            }
            '}' => {
                self.advance();
                Ok(Token::RCurly)
            }
            ';' => {
                self.advance();
                Ok(Token::SColon)
            }
            '-' => {
                self.advance();
                Ok(Token::Minus)
            }
            '+' => {
                self.advance();
                Ok(Token::Plus)
            }
            '*' => {
                self.advance();
                Ok(Token::Mul)
            }
            '/' => {
                self.advance();
                Ok(Token::Div)
            }
            '%' => {
                self.advance();
                Ok(Token::Mod)
            },
            // multi-char tokens
            ':' => {
                // consume : and =
                self.advance();
                if self.get_char() == '=' {
                    self.advance();
                    return Ok(Token::Assign);
                }
                Err(ScannerError::UnexpectedToken(self.get_char().to_string()))
            }
            // multi-char operators
            '|'  => {
                // consume |
                self.advance();
                if '|' == self.get_char() {
                   // consume |
                   self.advance();
                   return Ok(Token::Or);
                }
                Err(ScannerError::UnexpectedToken(self.get_char().to_string()))
            }
            '&' => {
                // consume &
                self.advance();
                if '|' == self.get_char() {
                   // consume &
                   self.advance();
                   return Ok(Token::And);
                }
                Err(ScannerError::UnexpectedToken(self.get_char().to_string()))
            }
            '!' => {
                // consume !
                self.advance();
                if '=' == self.get_char() {
                    // consume =
                    self.advance();
                    return Ok(Token::NotEqual);
                }
                Ok(Token::Not)
            },
            '=' => {
                // consume =
                self.advance();
                if '=' == self.get_char() {
                   // consume =
                   self.advance();
                   return Ok(Token::Equals);
                }
                Err(ScannerError::UnexpectedToken(self.get_char().to_string()))
            },
            '<' => {
               self.parse_angle(Token::Less, Token::LShift, Token::LessEqual)
            }
            '>' => {
               self.parse_angle(Token::More, Token::RShift, Token::MoreEqual)
            }

            // numbers
            '0'..='9' => {
                self.parse_num()
            }
            

            // keywords, function names, variable names
            'a'..='z' | 'A'..='Z' => self.parse_word(),
            // eof
            EOF_CHAR => Ok(Token::Eof),
            // unknown character
            unknown => Err(ScannerError::UnexpectedToken(unknown.to_string())),
        }
    }

    // gets the character in input at cur.
    // returns the EOF_CHAR if we have overun input
    fn get_char(&self) -> char {
        if self.input.len() > self.cur {
            return self.input.as_bytes()[self.cur] as char;
        }
        EOF_CHAR
    }

    // advances scanner to next byte
    fn advance(&mut self) {
        self.cur += 1;
    }

    // skips over all whitespace in input
    fn skip_whitespace(&mut self) {

        while self.get_char().is_whitespace() {
            self.advance();
        }
    }

    // helper to parse angle bracket tokens (< or >)
    fn parse_angle(&mut self, single: Token, double: Token, equal: Token) -> Result<Token, ScannerError> {
        assert!(self.get_char() == '<' || self.get_char() == '>');
        let first = self.get_char();
        // consume first
        self.advance();
        let second = self.get_char();
        if second == first {
            // consume second
            self.advance();
            return Ok(double);
        } else if '=' == second {
            // consume =
            self.advance();
            return Ok(equal);
        }
        Ok(single)
    }

    // parses a word, returns either a keyword or a name
    fn parse_word(&mut self) -> Result<Token, ScannerError> {
        assert!(self.get_char().is_alphabetic());
    
        // keep consuming tokens until we reach a non-alphanumeric
        let mut word = String::new();
        while self.get_char().is_alphanumeric() {
            word.push(self.get_char());
            // consume next
            self.advance();
        }

        // check if word is a keyword, if so return it
        if let Some(keyword) = Scanner::get_keyword(word.as_str()) {
            return Ok(keyword);
        }
        // else, a name
        Ok(Token::Name(word))
    }

    // parses a floating point number
    fn parse_num(&mut self) -> Result<Token, ScannerError> {
        assert!(self.get_char().is_ascii_digit());
        let mut num = String::new();

        // consume until we hit non digit or non .
        while self.get_char().is_ascii_digit() || self.get_char() == '.' {
            num.push(self.get_char());
            self.advance();
        }

        // let parse worry about extra .'s or other problems
        match num.parse() {
            Ok(val) => Ok(Token::Num(val)),
            Err(_) => Err(ScannerError::InvalidNumeric(num)),
        }

    }

    // tries to convert a word to a keyword,
    // returning None if word is not a valid keyword
    // todo: better name, also should this go in tokens.rs?
    fn get_keyword(word: &str) -> Option<Token> {
        match word {
            "def" => Some(Token::Def),
            "return" => Some(Token::Return),
            "if" => Some(Token::If),
            "else" => Some(Token::Else),
            "while" => Some(Token::While),
            _ => None
        }
    }
}

impl Iterator for Scanner {
    type Item = Result<Token, ScannerError>;

    fn next(&mut self) -> Option<Self::Item> {
        let res = self.next_token();
        if res.is_ok() {
            if let Token::Eof = res.as_ref().unwrap() {
                return None;
            }
        }

        Some(res)
    }
}


#[cfg(test)]
mod test {

    use super::*;

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