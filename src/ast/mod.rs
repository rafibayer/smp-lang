use crate::interpreter::symbols::*;
use crate::scanner::{Scanner, ScannerError};
use crate::tokens::{Token, TokenDiscriminants};

mod operator;

#[cfg(test)]
mod test;

const DISCRIMINANT_ERROR: &str = "Enum variant did not match discriminant";

#[derive(Debug)]
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

// Generates AST for function def args
fn generate_args(scanner: &mut Scanner) -> Result<Args, ASTError> {
    let mut names = Vec::new();
    while !variant_equal(&scanner.peek_next(), TokenDiscriminants::RParen) {
        // consume name
        let arg = match consume_token(scanner, TokenDiscriminants::Name)? {
            Token::Name(value) => value,
            _ => panic!("{}", DISCRIMINANT_ERROR),
        };
        names.push(arg);

        let next = scanner.peek_next();
        if variant_equal(&next, TokenDiscriminants::Comma) {
            // consume ,
            consume_token(scanner, TokenDiscriminants::Comma)?;
        } else if !variant_equal(&next, TokenDiscriminants::RParen) {
            // if we see no more commas after an arg, we must be at the last arg
            // therefore, ) must be next (however we leave consuming it to the caller)
            return Err(ASTError::UnexpectedToken(next));
        }
    }
    Ok(Args { names })
}

// Generates AST for a block
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

// Generates AST for statment
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
                // array assingment and access
                Token::LBracket => {
                    // consume [
                    consume_token(scanner, TokenDiscriminants::LBracket)?;
                    // consume index exp
                    let index_exp = generate_exp(scanner)?;
                    // consume ]
                    consume_token(scanner, TokenDiscriminants::RBracket)?;

                    // ArrayAssign
                    if variant_equal(&scanner.peek_next(), TokenDiscriminants::Assign) {
                        // consume :=
                        consume_token(scanner, TokenDiscriminants::Assign)?;
                        // consume value exp
                        let value = generate_exp(scanner)?;
                        // consume ;
                        consume_token(scanner, TokenDiscriminants::SColon)?;
                        StatementKind::ArrayAssign {
                            name,
                            index_exp,
                            value,
                        }
                    // Array Access
                    } else {
                        // consume array access
                        let preexp = Exp {
                            exp: Box::new(ExpKind::ArrayAccess {
                                name: name,
                                index: index_exp,
                            }),
                        };
                        // consume rest of expression
                        let exp = generate_exp_preexp(scanner, preexp)?;
                        // consume ;
                        consume_token(scanner, TokenDiscriminants::SColon)?;
                        StatementKind::Exp(exp)
                    }
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

// Generates AST for exp
fn generate_exp(scanner: &mut Scanner) -> Result<Exp, ASTError> {
    let exp = match scanner.next_token()? {
        // let all name-first expressions get handled by special case
        Token::Name(name) => {
            return generate_exp_name(scanner, name);
        }
        // builtins
        Token::Sqrt => {
            // consume (
            consume_token(scanner, TokenDiscriminants::LParen)?;
            // consume actual
            let preexp = Exp {
                exp: Box::new(ExpKind::BuiltIn(BuiltIn {
                    builtin: BuiltInKind::Sqrt(generate_exp(scanner)?),
                })),
            };
            // consume )
            consume_token(scanner, TokenDiscriminants::RParen)?;
            return generate_exp_preexp(scanner, preexp);
        }
        Token::Len => {
            // consume (
            consume_token(scanner, TokenDiscriminants::LParen)?;
            // consume actual
            let preexp = Exp {
                exp: Box::new(ExpKind::BuiltIn(BuiltIn {
                    builtin: BuiltInKind::Len(generate_exp(scanner)?),
                })),
            };
            // consume )
            consume_token(scanner, TokenDiscriminants::RParen)?;
            return generate_exp_preexp(scanner, preexp);
        },
        Token::Round => {
            // consume (
            consume_token(scanner, TokenDiscriminants::LParen)?;
            // consume actual
            let preexp = Exp {
                exp: Box::new(ExpKind::BuiltIn(BuiltIn {
                    builtin: BuiltInKind::Round(generate_exp(scanner)?),
                })),
            };
            // consume )
            consume_token(scanner, TokenDiscriminants::RParen)?;
            return generate_exp_preexp(scanner, preexp);
        },
        // num and infix cases
        Token::Num(value) => {
            match scanner.peek_next() {
                // just a number followed by ; or , or )
                // todo, i think if i include curly here, i can ditch parens in nest
                Token::SColon | Token::Comma | Token::RParen | Token::RBracket => {
                    ExpKind::Num(value)
                }
                // Infix operators
                _ => {
                    // generate infix: num op exp
                    // operator is next, and is consumed by next_token, leaving generate_exp to get exp
                    generate_infix(
                        ExpKind::Num(value),
                        operator::lookup_infix(scanner.next_token()?)?,
                        generate_exp(scanner)?,
                    )?
                }
            }
        }
        // array initialization
        Token::LBracket => {
            // consume size exp
            let exp = generate_exp(scanner)?;

            // consume ]
            consume_token(scanner, TokenDiscriminants::RBracket)?;

            ExpKind::ArrayInit { size: exp }
        }
        // parenthesized exp
        Token::LParen => {
            // consume exp
            let exp = generate_exp(scanner)?;
            // consume )
            consume_token(scanner, TokenDiscriminants::RParen)?;

            // either returns just expression, or full expression with next
            // infix operator
            return generate_exp_preexp(scanner, exp);
        }

        // unop exp
        Token::Minus => {
            let exp = generate_exp(scanner)?;
            ExpKind::Unary(
                Unop {
                    unop: UnopKind::Neg,
                },
                exp,
            )
        }
        Token::Not => {
            let exp = generate_exp(scanner)?;
            ExpKind::Unary(
                Unop {
                    unop: UnopKind::Not,
                },
                exp,
            )
        }
        // illegal
        other => {
            dbg!();
            return Err(ASTError::UnexpectedToken(other));
        }
    };

    Ok(Exp { exp: Box::new(exp) })
}

// either returns just expression, or full expression with next
// infix operator.
// 1+1 => Infix(1, +, 1)
// (1) + 1 => Infix((1), +, 1)
// this is to help with cases where a statment contains a paren exp followed by an operator.
fn generate_exp_preexp(scanner: &mut Scanner, preexp: Exp) -> Result<Exp, ASTError> {
    match scanner.peek_next() {
        // lone expression
        Token::SColon => Ok(preexp),
        // block start
        Token::LCurly => Ok(preexp),
        // part of a nested paren
        Token::RParen => Ok(preexp),
        // else must be infix
        _ => Ok(Exp {
            // consumes infix
            exp: Box::new(ExpKind::Infix(
                preexp,
                Op {
                    op: operator::lookup_infix(scanner.next_token()?)?,
                },
                generate_exp(scanner)?,
            )),
        }),
    }
}

// special case of generate exp, beggining with a passed name
// that we had to consume in generate_statment or generate_exp to look ahead
// exp ::= name | exp op exp | name "(" exps ")"
fn generate_exp_name(scanner: &mut Scanner, name: String) -> Result<Exp, ASTError> {
    let exp = match scanner.peek_next() {
        // name on it's own
        Token::SColon | Token::Comma | Token::RParen | Token::RBracket => ExpKind::Name(name),
        // name followed by parens (function call)
        Token::LParen => {
            // consume (
            consume_token(scanner, TokenDiscriminants::LParen)?;
            // compute function args
            let mut exps = Vec::new();
            while !variant_equal(&scanner.peek_next(), TokenDiscriminants::RParen) {
                let param = generate_exp(scanner)?;
                exps.push(param);
                if variant_equal(&scanner.peek_next(), TokenDiscriminants::Comma) {
                    consume_token(scanner, TokenDiscriminants::Comma)?;
                }
            }

            // consume )
            consume_token(scanner, TokenDiscriminants::RParen)?;
            ExpKind::Call(name, Exps { exps })
        }
        Token::LBracket => {
            // consume [
            consume_token(scanner, TokenDiscriminants::LBracket)?;
            // consume exp
            let index = generate_exp(scanner)?;
            // consume ]
            consume_token(scanner, TokenDiscriminants::RBracket)?;
            ExpKind::ArrayAccess { name, index }
        }
        // infix starting with name
        _ => {
            // generate infix: name op exp
            // operator is next, and is consumed by next_token, leaving generate_exp to get exp
            generate_infix(
                ExpKind::Name(name),
                operator::lookup_infix(scanner.next_token()?)?,
                generate_exp(scanner)?,
            )?
        }
    };

    Ok(Exp { exp: Box::new(exp) })
}

// generate AST for Nest: If, If/Else, and While
fn generate_nest(scanner: &mut Scanner) -> Result<Nest, ASTError> {
    // consume If or While
    let next = scanner.next_token()?;
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
        _ => {
            dbg!();

            return Err(ASTError::UnexpectedToken(next));
        }
    };

    Ok(Nest { nest })
}

// Generates AST for Infix expression
fn generate_infix(lhs: ExpKind, op: OpKind, rhs: Exp) -> Result<ExpKind, ASTError> {
    Ok(ExpKind::Infix(Exp { exp: Box::new(lhs) }, Op { op }, rhs))
}

// Consumes a token from the scanner specified by variant.
// Returns an ASTError if the next token was not the expected token.
fn consume_token(scanner: &mut Scanner, variant: TokenDiscriminants) -> Result<Token, ASTError> {
    let next = scanner.next_token()?;
    if variant_equal(&next, variant) {
        return Ok(next);
    }
    // todo: remove
    let disc: TokenDiscriminants = next.clone().into();
    println!("expected: {:?}, got: {:?}", variant, disc);
    Err(ASTError::UnexpectedToken(next))
}

// Returns true if the given token matches the given variant
// https://users.rust-lang.org/t/comparing-enums-by-variants/22546/4
fn variant_equal(token: &Token, variant: TokenDiscriminants) -> bool {
    let disc: TokenDiscriminants = token.clone().into();
    disc == variant
}
