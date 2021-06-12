use crate::interpreter::symbols::*;
use crate::scanner::{Scanner, ScannerError};
use crate::tokens::{Token, TokenDiscriminants};

mod operator;

const DISCRIMINANT_ERROR: &str = "Enum variant did not match discriminant";

pub enum ASTError {
    ScannerError(ScannerError),
    UnexpectedToken(Token),
    InvalidOperator(Token),
}

impl From<ScannerError> for ASTError {
    fn from(err: ScannerError) -> Self {
        ASTError::ScannerError(err)
    }
}

pub fn generate_ast(scanner: &mut Scanner) -> Result<Program, ASTError> {
    let mut defs = Vec::new();
    while !scanner.is_eof() {
        let def = generate_def(scanner)?;
        defs.push(def);
    }

    Ok(Program { defs })
}

fn generate_def(scanner: &mut Scanner) -> Result<Def, ASTError> {
    // consume def
    consume_token(scanner, TokenDiscriminants::Def)?;

    // consume name
    let name = match consume_token(scanner, TokenDiscriminants::Name)? {
        Token::Name(value) => value,
        _ => panic!("{}", DISCRIMINANT_ERROR),
    };

    // consume (
    consume_token(scanner, TokenDiscriminants::LParen)?;

    // consume args
    let args = generate_args(scanner)?;

    // consume )
    consume_token(scanner, TokenDiscriminants::RParen)?;

    let block = generate_block(scanner)?;

    Ok(Def { name, args, block })
}

fn generate_args(scanner: &mut Scanner) -> Result<Args, ASTError> {
    let mut names = Vec::new();
    while !variant_equal(&scanner.peek_next(), TokenDiscriminants::LParen) {
        // consume name
        let arg = match consume_token(scanner, TokenDiscriminants::Name)? {
            Token::Name(value) => value,
            _ => panic!("{}", DISCRIMINANT_ERROR),
        };
        names.push(arg);

        // consume ,
        consume_token(scanner, TokenDiscriminants::Comma)?;
    }

    Ok(Args { names })
}

fn generate_block(scanner: &mut Scanner) -> Result<Block, ASTError> {
    // consume {
    consume_token(scanner, TokenDiscriminants::LCurly)?;

    let mut statements = Vec::new();
    while !variant_equal(&scanner.peek_next(), TokenDiscriminants::RCurly) {
        statements.push(generate_statement(scanner)?);
    }

    // consume }
    consume_token(scanner, TokenDiscriminants::RCurly)?;

    Ok(Block { statements })
}

// statement ::= "return" exp ";" | name ":=" exp ";" | exp ";" | nest
fn generate_statement(scanner: &mut Scanner) -> Result<Statement, ASTError> {
    let statement = match scanner.peek_next() {
        Token::Return => {
            // consume return
            consume_token(scanner, TokenDiscriminants::Return)?;
            // consume exp
            let exp = generate_exp(scanner)?;
            // consume ;
            consume_token(scanner, TokenDiscriminants::SColon)?;
            StatementKind::Return(exp)
        }
        Token::If | Token::While => StatementKind::Nest(generate_nest(scanner)?),
        // here there is some ambiguity, if name is next, this statement
        // could either be: a. an assigment, or b. an exp.
        // we consume the token to look ahead to check for the assigment operator.
        Token::Name(name) => {
            // consume name
            consume_token(scanner, TokenDiscriminants::Name)?;
            match scanner.peek_next() {
                // statment is an assigment
                Token::Assign => {
                    // consume :=
                    consume_token(scanner, TokenDiscriminants::Assign)?;
                    let exp = generate_exp(scanner)?;
                    // consume ;
                    consume_token(scanner, TokenDiscriminants::SColon)?;
                    StatementKind::Assign { name, exp }
                }
                // otherwise, just an exp starting with a name (e.i usage or fn call)
                _ => {
                    let exp = generate_exp_name(scanner, name)?;
                    // consume ;
                    consume_token(scanner, TokenDiscriminants::SColon)?;
                    StatementKind::Exp(exp)
                }
            }
        }
        // try to parse an exp
        _ => {
            let exp = generate_exp(scanner)?;
            consume_token(scanner, TokenDiscriminants::SColon)?;
            StatementKind::Exp(exp)
        }
    };

    Ok(Statement { statement })
}



// exp ::= name | num | exp op exp | exp "(" exps ")" | "(" exp ")" | unop exp
fn generate_exp(scanner: &mut Scanner) -> Result<Exp, ASTError> {
    let first = scanner.get_next()?;
    let exp = match first {
        // let all name-first expressions get handled by special case
        Token::Name(name) => {
            return generate_exp_name(scanner, name);
        }
        // num and infix cases
        Token::Num(value) => {
            match scanner.peek_next() {
                // just a number followed by ;
                Token::SColon => ExpKind::Num(value),
                // Infix operators
                Token::Plus => {
                    generate_infix(ExpKind::Num(value), operator::lookup_infix(scanner.peek_next())?, generate_exp(scanner)?)?
                }
                Token::Mul => {
                    generate_infix(ExpKind::Num(value), OpKind::Mul, generate_exp(scanner)?)?
                }
                Token::Div => {
                    generate_infix(ExpKind::Num(value), OpKind::Div, generate_exp(scanner)?)?
                }
                Token::Mod => {
                    generate_infix(ExpKind::Num(value), OpKind::Mod, generate_exp(scanner)?)?
                }
                // logical operators
                Token::Or => generate_infix(
                    ExpKind::Num(value),
                    OpKind::Logical(Logical {
                        logical: LogicalKind::Or,
                    }),
                    generate_exp(scanner)?,
                )?,
                Token::And => generate_infix(
                    ExpKind::Num(value),
                    OpKind::Logical(Logical {
                        logical: LogicalKind::And,
                    }),
                    generate_exp(scanner)?,
                )?,
                // comparison operators
                Token::Equals => generate_infix(
                    ExpKind::Num(value),
                    OpKind::Comparison(Comparison {
                        comparison: ComparisonKind::Equals,
                    }),
                    generate_exp(scanner)?,
                )?,
                Token::Less => generate_infix(
                    ExpKind::Num(value),
                    OpKind::Comparison(Comparison {
                        comparison: ComparisonKind::Less,
                    }),
                    generate_exp(scanner)?,
                )?,
                Token::More => generate_infix(
                    ExpKind::Num(value),
                    OpKind::Comparison(Comparison {
                        comparison: ComparisonKind::More,
                    }),
                    generate_exp(scanner)?,
                )?,
                Token::LessEqual => generate_infix(
                    ExpKind::Num(value),
                    OpKind::Comparison(Comparison {
                        comparison: ComparisonKind::LessEqual,
                    }),
                    generate_exp(scanner)?,
                )?,
                Token::MoreEqual => generate_infix(
                    ExpKind::Num(value),
                    OpKind::Comparison(Comparison {
                        comparison: ComparisonKind::MoreEqual,
                    }),
                    generate_exp(scanner)?,
                )?,
                Token::NotEqual => generate_infix(
                    ExpKind::Num(value),
                    OpKind::Comparison(Comparison {
                        comparison: ComparisonKind::NotEqual,
                    }),
                    generate_exp(scanner)?,
                )?,
                other => {
                    return Err(ASTError::UnexpectedToken(other))
                }
            }
        }
        // parenthesized exp
        Token::LParen => {
            // consume exp
            let exp = generate_exp(scanner)?;
            // consume )
            consume_token(scanner, TokenDiscriminants::RParen)?;
            ExpKind::Paren(exp)
        }
        // unop exp
        Token::Minus => {
            let exp = generate_exp(scanner)?;
            ExpKind::Unary(Unop{unop: UnopKind::Neg}, exp)
        }
        Token::Not => {
            let exp = generate_exp(scanner)?;
            ExpKind::Unary(Unop{unop: UnopKind::Not}, exp)
        }
        // illegal
        other => {
            return Err(ASTError::UnexpectedToken(other))
        }
    };

    Ok(Exp { exp: Box::new(exp) })
}

// special case of generate exp, beggining with a passed name
// that we had to consume in generate_statment or generate_exp to look ahead
// exp ::= name | exp op exp | name "(" exps ")"
fn generate_exp_name(scanner: &mut Scanner, name: String) -> Result<Exp, ASTError> {
    let exp = match scanner.peek_next() {
        // name on it's own
        Token::SColon => ExpKind::Name(name),
        // name followed by parens (function call)
        Token::LParen => {
            // consume (
            consume_token(scanner, TokenDiscriminants::LParen)?;
            // compute function args
            let mut exps = Vec::new();
            while !variant_equal(&scanner.peek_next(), TokenDiscriminants::RParen) {
                exps.push(generate_exp(scanner)?);
            }

            // consume )
            consume_token(scanner, TokenDiscriminants::RParen)?;
            ExpKind::Call(name, Exps {exps})
        },
        // infix starting with name
        other => {
            ExpKind::Infix()
        }
    }

    Ok(Exp {exp: Box::new(exp)})
}

fn generate_nest(scanner: &mut Scanner) -> Result<Nest, ASTError> {
    // consume If or While
    let next = scanner.get_next()?;
    let nest = match next {
        Token::If => {
            // consume cond
            let cond = generate_exp(scanner)?;
            // consume then
            let then = generate_block(scanner)?;
            // if the nest statement is an else
            if variant_equal(&scanner.peek_next(), TokenDiscriminants::Else) {
                // consume else
                consume_token(scanner, TokenDiscriminants::Else)?;
                // consume else_ block
                let else_ = generate_block(scanner)?;
                NestKind::IfElse { cond, then, else_ }
            // otherwise, just an If
            } else {
                NestKind::If { cond, then }
            }
        }
        Token::While => {
            // consume cond
            let cond = generate_exp(scanner)?;
            // consume block
            let block = generate_block(scanner)?;
            NestKind::While { cond, block }
        }
        _ => return Err(ASTError::UnexpectedToken(next)),
    };

    Ok(Nest { nest })
}

fn generate_infix(lhs: ExpKind, op: OpKind, rhs: Exp) -> Result<ExpKind, ASTError> {
    Ok(ExpKind::Infix(Exp { exp: Box::new(lhs) }, Op { op }, rhs))
}

// https://users.rust-lang.org/t/comparing-enums-by-variants/22546/4
fn consume_token(scanner: &mut Scanner, variant: TokenDiscriminants) -> Result<Token, ASTError> {
    let next = scanner.next_token()?;
    if variant_equal(&next, variant) {
        return Ok(next);
    }
    Err(ASTError::UnexpectedToken(next))
}

fn variant_equal(token: &Token, variant: TokenDiscriminants) -> bool {
    let disc: TokenDiscriminants = token.clone().into();
    disc == variant
}
