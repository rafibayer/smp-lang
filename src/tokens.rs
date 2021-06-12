/* 
LANGUAGE GRAMMAR
SOURCE: http://canonical.org/~kragen/memory-models/
program ::= def*
def ::= "def" name "(" args ")" block
args ::= "" | name "," args
block ::= "{" statement* "}"
statement ::= "return" exp ";" | name ":=" exp ";" | exp ";" | nest
nest ::= "if" exp block | "if" exp block "else" block | "while" exp block
exp ::= name | num | exp op exp | exp "(" exps ")" | "(" exp ")" | unop exp
exps ::= "" | exp "," exps
unop ::= "!" | "-" | "~"
op ::= logical | comparison | "+" | "*" | "-" | "/" | "%"
logical ::= "||" | "&&" | "&" | "|" | "^" | "<<" | ">>"
comparison ::= "==" | "<" | ">" | "<=" | ">=" | "!="
*/

#[derive(Debug, PartialEq)]
pub enum Token {
    // file
    Eof,
    // basic
    Def,          // def
    Name(String), // function/variable name
    LParen,       // (
    RParen,       // )
    Comma,        // ,
    LCurly,       // {
    RCurly,       // }
    Return,       // return
    SColon,       // ;
    Assign,       // :=
    /* flow */
    If,       // if
    Else,     // else
    While,    // while
    Num(f64), // numeric value
    // unary
    Not,    // !
    Minus,  // -
    // BitNot, // ~
    // op
    Plus, // +
    Mul,  // *
    Div,  // /
    Mod,  // %
    // logical
    Or,     // ||
    And,    // &&
    // BitOr,  // |
    // BitAnd, // &
    // Xor, // ^
    LShift, // <<
    RShift, // >>
    // Comparison
    Equals,    // ==
    Less,      // <
    More,      // >
    LessEqual, // <=
    MoreEqual, // >=
    NotEqual,  // !=
}


