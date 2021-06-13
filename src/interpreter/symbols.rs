/*
LANGUAGE GRAMMAR
SOURCE: http://canonical.org/~kragen/memory-models/
program ::= def*
def ::= "def" name "(" args ")" block
args ::= "" | name "," args
block ::= "{" statement* "}"
statement ::= "return" exp ";" | name ":=" exp ";" | name "[" exp "]" ":=" exp ";"|  exp ";" | nest
nest ::= "if" (exp) block | "if" (exp) block "else" block | "while" (exp) block
exp ::= name | num | "[" exp "]" | exp op exp | name "[" exp "]" | name "(" exps ")" | "(" exp ")" | unop exp
exps ::= "" | exp "," exps
unop ::= "!" | "-"
op ::= logical | comparison | "+" | "*" | "-" | "/" | "%"
logical ::= "||" | "&&"
comparison ::= "==" | "<" | ">" | "<=" | ">=" | "!="
*/


// program ::= def*
#[derive(Debug, Clone)]
pub struct Program {
    pub defs: Vec<Def>
}

// def ::= "def" name "(" args ")" block
#[derive(Debug, Clone)]
pub struct Def {
    pub name: String,
    pub args: Args,
    pub block: Block,
}

// args ::= "" | name "," args
#[derive(Debug, Clone)]
pub struct Args {
    pub names: Vec<String>
}

// block ::= "{" statement* "}"
#[derive(Debug, Clone)]
pub struct Block {
    pub statements: Vec<Statement>
}

// statement ::= "return" exp ";" | name ":=" exp ";" | name "[" num "]" := exp ";"|  exp ";" | nest
#[derive(Debug, Clone)]
pub enum StatementKind {
    Return(Exp),
    Assign {name: String, exp: Exp},
    ArrayAssign {name: String, index_exp: Exp, value: Exp},
    Exp(Exp),
    Nest(Nest),
}

#[derive(Debug, Clone)]
pub struct Statement {
    pub statement: StatementKind
}

// nest ::= "if" (exp) block | "if" (exp) block "else" block | "while" (exp) block
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

// exp ::= name | num | "[" exp "]" | exp op exp | name "[" exp "]" | name "(" exps ")" | "(" exp ")" | unop exp
#[derive(Debug, Clone)]
pub enum ExpKind {
    Name(String),
    Num(f64),
    ArrayInit{size: Exp},
    Infix(Exp, Op, Exp),
    ArrayAccess{name: String, index: Exp},
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

// unop ::= "!" | "-" 
#[derive(Debug, Clone)]
pub enum UnopKind {
    Not,
    Neg,
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

// logical ::= "||" | "&&"
#[derive(Debug, Clone)]
pub enum LogicalKind {
    Or,
    And,
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