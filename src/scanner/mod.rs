use crate::tokens::Token;


#[cfg(test)]
mod test;

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
    src_line: usize,
}

impl Scanner {

    pub fn new(input: String) -> Result<Scanner, ScannerError> {
        let mut scanner = 
        Scanner {
            input,
            cur: 0,
            next: Token::Start,
            src_line: 1,
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
    
    // gets and consumes the next Token
    pub fn next_token(&mut self) -> Result<Token, ScannerError> {
        let result = self.next.clone();
        self.next = self.get_next()?;
        // println!("src: {}", self.src_line);
        Ok(result)
    }

    // returns the next recognized token in the input
    fn get_next(&mut self) -> Result<Token, ScannerError> {
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
            '[' => {
                self.advance();
                Ok(Token::LBracket)
            }
            ']' => {
                self.advance();
                Ok(Token::RBracket)
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
                if '&' == self.get_char() {
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
                // consume <
                self.advance();
                if '=' == self.get_char() {
                    // consume =
                    self.advance();
                    return Ok(Token::LessEqual);
                }
                Ok(Token::Less)
            }
            '>' => {
                // consume >
                self.advance();
                if '=' == self.get_char() {
                    // consume =
                    self.advance();
                    return Ok(Token::MoreEqual);
                }
                Ok(Token::More)
            }

            // numbers
            '0'..='9' => {
                self.parse_num()
            }
            

            // keywords, function names, variable names
            'a'..='z' | 'A'..='Z' => self.parse_word(),
            // eof
            EOF_CHAR => Ok(Token::Eof),
            // comments
            '#' => {
                self.skip_line();
                self.get_next()
            }
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
            if self.get_char() == '\n' {
                self.src_line += 1;
            }
            self.advance();
        }
    }

    // advances until next line, or EOF
    fn skip_line(&mut self) {
        while self.get_char() != '\n' && self.get_char() != EOF_CHAR {
            self.advance();
        }

        if self.get_char() == '\n' {
            self.advance();
            self.src_line += 1;
        }
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

    // tries to convert a str to a keyword,
    // returning None if the str is not a valid keyword
    fn get_keyword(word: &str) -> Option<Token> {
        match word {
            "def" => Some(Token::Def),
            "return" => Some(Token::Return),
            "if" => Some(Token::If),
            "else" => Some(Token::Else),
            "while" => Some(Token::While),
            "sqrt" => Some(Token::Sqrt),
            "len" => Some(Token::Len),
            "round" => Some(Token::Round),
            "input" => Some(Token::Input),
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