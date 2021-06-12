/*
program ::= def*
def ::= "def" name "(" args ")" block
args ::= "" | name "," args
block ::= "{" statement* "}"
statement ::= "return" exp ";" | name ":=" exp ";" | exp ";" | nest
nest ::= "if" exp block | "if" exp block "else" block | "while" exp block
exp ::= name | num | exp op exp | name "(" exps ")" | "(" exp ")" | unop exp
exps ::= "" | exp "," exps
unop ::= "!" | "-"
op ::= logical | comparison | "+" | "*" | "-" | "/" | "%"
logical ::= "||" | "&&"
comparison ::= "==" | "<" | ">" | "<=" | ">=" | "!="
*/


pub struct Program {
    pub defs: Vec<Def>
}

#[derive(Debug, Clone)]
pub struct Def {
    pub name: String,
    pub args: Args,
    pub block: Block,
}

#[derive(Debug, Clone)]
pub struct Args {
    pub names: Vec<String>
}

#[derive(Debug, Clone)]
pub struct Block {
    pub statements: Vec<Statement>
}

// statement ::= "return" exp ";" | name ":=" exp ";" | exp ";" | nest
#[derive(Debug, Clone)]
pub enum StatementKind {
    Return(Exp),
    Assign {name: String, exp: Exp},
    Exp(Exp),
    Nest(Nest),
}

#[derive(Debug, Clone)]
pub struct Statement {
    pub statement: StatementKind
}

#[derive(Debug, Clone)]
pub enum NestKind {
    If {cond: Exp, then: Block},
    IfElse {cond: Exp, then: Block, else_: Block},
    While {cond: Exp, block: Block},
}

#[derive(Debug, Clone)]
pub struct Nest {
    pub nest: NestKind
}

// exp ::= name | num | exp op exp | exp "(" exps ")" | "(" exp ")" | unop exp
#[derive(Debug, Clone)]
pub enum ExpKind {
    Name(String),
    Num(f64),
    Infix(Exp, Op, Exp),
    Call(String, Exps),
    Paren(Exp),
    Unary(Unop, Exp),
}

#[derive(Debug, Clone)]
pub struct Exp {
    pub exp: Box<ExpKind>
}

// exps ::= "" | exp "," exps
#[derive(Debug, Clone)]
pub struct Exps {
    pub exps: Vec<Exp>
}

// unop ::= "!" | "-" | "~"
#[derive(Debug, Clone)]
pub enum UnopKind {
    Not,
    Neg,
    // BitNot,
}

#[derive(Debug, Clone)]
pub struct Unop {
    pub unop: UnopKind
}


// op ::= logical | comparison | "+" | "*" | "-" | "/" | "%"
#[derive(Debug, Clone)]
pub enum OpKind {
    Logical(Logical),
    Comparison(Comparison),
    Plus,
    Mul,
    Minus,
    Div,
    Mod
}

#[derive(Debug, Clone)]
pub struct Op {
    pub op: OpKind
}

// logical ::= "||" | "&&" | "&" | "|" | "^" | "<<" | ">>"
#[derive(Debug, Clone)]
pub enum LogicalKind {
    Or,
    And,
    // BitAnd,
    // BitOr,
    // Xor,
    // LShift,
    // RShift,
}

#[derive(Debug, Clone)]
pub struct Logical {
    pub logical: LogicalKind
}

// comparison ::= "==" | "<" | ">" | "<=" | ">=" | "!="
#[derive(Debug, Clone)]
pub enum ComparisonKind {
    Equals,
    Less,
    More,
    LessEqual,
    MoreEqual,
    NotEqual
}

#[derive(Debug, Clone)]
pub struct Comparison {
    pub comparison: ComparisonKind
}