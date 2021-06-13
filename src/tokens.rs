/*
LANGUAGE GRAMMAR
SOURCE: http://canonical.org/~kragen/memory-models/
program ::= def*
def ::= "def" name "(" args ")" block
args ::= "" | name "," args
block ::= "{" statement* "}"
statement ::= "return" exp ";" | name ":=" exp ";" | name"["num"]" := exp ";"|  exp ";" | nest
nest ::= "if" (exp) block | "if" (exp) block "else" block | "while" (exp) block
exp ::= name | num | "["num"]" | exp op exp | name"["num"]" | name "(" exps ")" | "(" exp ")" | unop exp
exps ::= "" | exp "," exps
unop ::= "!" | "-"
op ::= logical | comparison | "+" | "*" | "-" | "/" | "%"
logical ::= "||" | "&&"
comparison ::= "==" | "<" | ">" | "<=" | ">=" | "!="
*/

#[derive(Debug, PartialEq, EnumDiscriminants, Clone)]
pub enum Token {
    // file
    Start, // placeholder before first call to next_token()
    Eof,   // returned once we've passed last token
    // basic
    Def,          // def
    Name(String), // function/variable name
    LParen,       // (
    RParen,       // )
    Comma,        // ,
    LCurly,       // {
    RCurly,       // }
    LBracket,     // [
    RBracket,     // ]
    Return,       // return
    SColon,       // ;
    Assign,       // :=
    // flow
    If,       // if
    Else,     // else
    While,    // while
    Num(f64), // numeric value
    // unary
    Not,   // !
    Minus, // -
    // BitNot, // ~
    // op
    Plus, // +
    Mul,  // *
    Div,  // /
    Mod,  // %
    // logical
    Or,  // ||
    And, // &&
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
