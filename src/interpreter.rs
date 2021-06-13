pub mod defs;
pub mod environment;
pub mod symbols;

#[cfg(test)]
mod test;

use defs::Defs;
pub use environment::{Environment, Value, ValueDiscriminants};
use std::{rc::Rc, usize};
use symbols::*;

const MAIN: &str = "main";
const EPSILON: f64 = 0.0000001;

#[derive(Debug)]
pub enum InterpreterError {
    UnboundVar(String),
    UnboundArr(String),
    UnboundFunc(String),
    TypeError {
        found_type: ValueDiscriminants,
        expected_type: ValueDiscriminants,
    },
    NoMainDefined,
    ArgMismatch {
        got: usize,
        expected: usize,
    },
    ValuelessExpression(Exp),
    DivideByZero,
}

// Interpreter evaluates a program Symbol (AST).
pub struct Interpreter {
    program: Program,
    defs: Defs,
}

impl Interpreter {
    pub fn new(program: Program) -> Interpreter {
        Interpreter {
            program,
            defs: Defs::new(),
        }
    }

    pub fn execute(mut self) -> Result<Option<Value>, InterpreterError> {
        // evaluate defs in program
        let mut env = Environment::new();

        // evaluate all defs
        self.eval_program();

        // execute main
        self.eval_call(&MAIN.to_string(), &Exps { exps: Vec::new() }, &mut env)
    }

    fn eval_program(&mut self) {
        for def in &self.program.defs {
            self.defs.bind_func(def.name.clone(), Rc::new(def.clone()));
        }
    }

    fn eval_call(
        &self,
        name: &str,
        exps: &Exps,
        env: &mut Environment,
    ) -> Result<Option<Value>, InterpreterError> {
        // compute arg actuals
        let mut actuals = Vec::new();
        for exp in &exps.exps {
            actuals.push(self.eval_exp(exp, env)?);
        }

        // get function
        let func = self.defs.get_func(&name.to_string())?;

        // ensure num actuals matches num args
        if actuals.len() != func.args.names.len() {
            return Err(InterpreterError::ArgMismatch {
                got: actuals.len(),
                expected: func.args.names.len(),
            });
        }

        // create a new environment with args bound to actuals
        let mut func_env = Environment::new();
        for (i, actual) in actuals.iter().enumerate() {
            func_env.bind_var(func.args.names[i].clone(), actual.clone());
        }

        // evaluate func block under new environment
        self.eval_block(&func.block, &mut func_env)
    }

    fn eval_exp(&self, exp: &Exp, env: &mut Environment) -> Result<Value, InterpreterError> {
        match &*exp.exp {
            ExpKind::Name(name) => env.get_var(&name),
            ExpKind::Num(value) => Ok(Value::from(*value)),
            ExpKind::Infix(lhs, op, rhs) => self.eval_infix(lhs, op, rhs, env),
            ExpKind::Call(name, exps) => {
                Interpreter::get_expression_result_value(&exp, self.eval_call(name, exps, env))
            }
            ExpKind::Paren(exp) => self.eval_exp(exp, env),
            ExpKind::Unary(op, exp) => self.eval_unop(op, exp, env),
            ExpKind::ArrayInit { size } => {
                let size = self.eval_exp(size, env)?;
                Ok(Value::from(vec![0f64; Value::into_f64(size)? as usize]))
            },
            ExpKind::ArrayAccess { name, index } => {
                let arr = Value::into_vec(env.get_var(name)?)?;
                let i = Value::into_f64(self.eval_exp(index, env)?)?;
                Ok(Value::from(arr[i as usize]))
            }
        }
    }

    fn eval_block(
        &self,
        block: &Block,
        env: &mut Environment,
    ) -> Result<Option<Value>, InterpreterError> {
        for statement in &block.statements {
            let res = self.eval_statement(statement, env)?;
            // if the statment is a return statment, stop evaluating and return as block result
            if res.is_some() {
                return Ok(res);
            }
        }

        // otherwise block has no value, return none
        Ok(None)
    }

    fn eval_statement(
        &self,
        statement: &Statement,
        env: &mut Environment,
    ) -> Result<Option<Value>, InterpreterError> {
        match &statement.statement {
            StatementKind::Return(exp) => Ok(Some(self.eval_exp(&exp, env)?)),
            StatementKind::Assign { name, exp } => {
                // bind the variable
                let value = self.eval_exp(&exp, env)?;
                env.bind_var(name.clone(), value);
                // binds evalute to nothing
                Ok(None)
            }
            StatementKind::ArrayAssign {
                name,
                index_exp,
                value,
            } => {
                let mut old = Value::into_vec(env.get_var(name)?)?;
                let index = Value::into_f64(self.eval_exp(&index_exp, env)?)? as usize;
                let new_val = self.eval_exp(value, env)?;
                old[index] = Value::into_f64(new_val)?;

                env.bind_var(name.clone(), Value::from(old));
                Ok(None)
            }
            StatementKind::Exp(exp) => {
                // statments composed of a single expression print but evaluate to nothing.
                // e.g. 5+5;
                // this will print "5" but the statement has no value

                println!("{}", self.eval_exp(&exp, env)?);
                Ok(None)
            }
            StatementKind::Nest(nest) => self.eval_nest(nest, env),
        }
    }

    fn eval_infix(
        &self,
        lhs: &Exp,
        op: &Op,
        rhs: &Exp,
        env: &mut Environment,
    ) -> Result<Value, InterpreterError> {
        let lhs_val = Value::into_f64(self.eval_exp(lhs, env)?)?;
        let rhs_val = Value::into_f64(self.eval_exp(rhs, env)?)?;

        match &op.op {
            OpKind::Logical(logical) => self.eval_logical(lhs, &logical, rhs, env),
            OpKind::Comparison(comparison) => self.eval_comparison(lhs, comparison, rhs, env),
            OpKind::Plus => Ok(Value::from(lhs_val + rhs_val)),
            OpKind::Mul => Ok(Value::from(lhs_val * rhs_val)),
            OpKind::Minus => Ok(Value::from(lhs_val - rhs_val)),
            OpKind::Div => {
                if rhs_val.abs() < EPSILON {
                    return Err(InterpreterError::DivideByZero);
                }
                Ok(Value::from(lhs_val / rhs_val))
            }
            OpKind::Mod => {
                if rhs_val.abs() < EPSILON {
                    return Err(InterpreterError::DivideByZero);
                }
                Ok(Value::from(lhs_val % rhs_val))
            }
        }
    }

    fn eval_unop(
        &self,
        unop: &Unop,
        exp: &Exp,
        env: &mut Environment,
    ) -> Result<Value, InterpreterError> {
        let value = Value::into_f64(self.eval_exp(exp, env)?)?;

        match unop.unop {
            UnopKind::Not => Ok(Value::from(Interpreter::bool_to_float(
                !Interpreter::truthy(value),
            ))),
            UnopKind::Neg => Ok(Value::from(-value)),
        }
    }

    fn eval_logical(
        &self,
        lhs: &Exp,
        logical: &Logical,
        rhs: &Exp,
        env: &mut Environment,
    ) -> Result<Value, InterpreterError> {
        let lhs_val = Interpreter::truthy(Value::into_f64(self.eval_exp(lhs, env)?)?);
        let rhs_val = Interpreter::truthy(Value::into_f64(self.eval_exp(rhs, env)?)?);

        match logical.logical {
            LogicalKind::Or => Ok(Value::from(Interpreter::bool_to_float(lhs_val || rhs_val))),
            LogicalKind::And => Ok(Value::from(Interpreter::bool_to_float(lhs_val && rhs_val))),
        }
    }

    fn eval_comparison(
        &self,
        lhs: &Exp,
        comparison: &Comparison,
        rhs: &Exp,
        env: &mut Environment,
    ) -> Result<Value, InterpreterError> {
        let lhs_val = Value::into_f64(self.eval_exp(lhs, env)?)?;
        let rhs_val = Value::into_f64(self.eval_exp(rhs, env)?)?;

        match comparison.comparison {
            ComparisonKind::Equals => Ok(Value::from(Interpreter::bool_to_float(
                (lhs_val - rhs_val).abs() < EPSILON,
            ))),
            // TODO: epsilon checking for comparisons?
            ComparisonKind::Less => Ok(Value::from(Interpreter::bool_to_float(lhs_val < rhs_val))),
            ComparisonKind::More => Ok(Value::from(Interpreter::bool_to_float(lhs_val > rhs_val))),
            ComparisonKind::LessEqual => {
                Ok(Value::from(Interpreter::bool_to_float(lhs_val <= rhs_val)))
            }
            ComparisonKind::MoreEqual => {
                Ok(Value::from(Interpreter::bool_to_float(lhs_val >= rhs_val)))
            }
            ComparisonKind::NotEqual => Ok(Value::from(Interpreter::bool_to_float(
                (lhs_val - rhs_val).abs() > EPSILON,
            ))),
        }
    }

    fn truthy(value: f64) -> bool {
        value.abs() > EPSILON
    }

    fn bool_to_float(bool: bool) -> f64 {
        (bool as u32) as f64
    }

    fn eval_nest(
        &self,
        nest: &Nest,
        env: &mut Environment,
    ) -> Result<Option<Value>, InterpreterError> {
        match &nest.nest {
            NestKind::If { cond, then } => {
                // evaluate truthiness of conditional expression
                let cond_val = Interpreter::truthy(match self.eval_exp(&cond, env) {
                    Ok(val) => Value::into_f64(val)?,
                    Err(err) => return Err(err),
                });

                // if the condition is true, evaluate the block
                if cond_val {
                    match self.eval_block(&then, env) {
                        // return the result of the block (will have value if block returned)
                        Ok(opt) => return Ok(opt),
                        Err(err) => return Err(err),
                    }
                }

                // if the condition is not true, do nothing
                Ok(None)
            }
            NestKind::IfElse { cond, then, else_ } => {
                // evaluate truthiness of conditional expression
                let cond_val = Interpreter::truthy(match self.eval_exp(&cond, env) {
                    Ok(val) => Value::into_f64(val)?,
                    Err(err) => return Err(err),
                });

                // if the condition is true, evaluate the block
                if cond_val {
                    match self.eval_block(&then, env) {
                        // return the result of the block (will have value if block returned)
                        Ok(opt) => Ok(opt),
                        Err(err) => Err(err),
                    }
                } else {
                    // else, evaluate the else block
                    match self.eval_block(&else_, env) {
                        // return the result of the block (will have value if block returned)
                        Ok(opt) => Ok(opt),
                        Err(err) => Err(err),
                    }
                }
            }
            NestKind::While { cond, block } => {
                // evaluate truthiness of conditional expression
                let mut cond_val = Interpreter::truthy(match self.eval_exp(&cond, env) {
                    Ok(val) => Value::into_f64(val)?,
                    Err(err) => return Err(err),
                });

                // while the condition is truthy
                while cond_val {
                    // execute the block
                    match self.eval_block(block, env) {
                        Ok(opt) => {
                            // return the result of the block (will have value if block returned).
                            // here we only return if block is some, as we don't want to exit block
                            // if there was no return
                            if opt.is_some() {
                                return Ok(opt);
                            }
                        }
                        Err(err) => return Err(err),
                    }

                    // update the condition
                    cond_val = Interpreter::truthy(match self.eval_exp(&cond, env) {
                        Ok(val) => Value::into_f64(val)?,
                        Err(err) => return Err(err),
                    });
                }
                // exit loop
                Ok(None)
            }
        }
    }
    // Attempts to get the value of an expression that may not return a value.
    // if no value can be unwrapped, returns a ValuelessExpression interpreter error
    fn get_expression_result_value(
        exp: &Exp,
        res: Result<Option<Value>, InterpreterError>,
    ) -> Result<Value, InterpreterError> {
        match res? {
            Some(value) => Ok(value),
            None => Err(InterpreterError::ValuelessExpression(exp.clone())),
        }
    }
}
