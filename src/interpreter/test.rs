

#[cfg(test)]
mod tests {
    use super::super::*;
    
    #[test]
    fn test_assign_return() {
        /* AST for program:
        def main() {
            a := 1;
            return a;
        }
        */
        let prog = Program {
            defs: vec![
                Def {
                    // def main
                    name: String::from("main"),
                    // ()
                    args: Args {
                        names: Vec::new()
                    },
                    // {
                    block: Block {
                        statements: vec![
                            // a :=
                            Statement {
                                statement: StatementKind::Assign {
                                    name: String::from("a"),
                                    exp: Exp {
                                        // 1;
                                        exp: Box::new(ExpKind::Num(1f64))
                                    }
                                }
                            },
                            // return
                            Statement {
                                statement: StatementKind::Return(
                                    Exp {
                                        // a;
                                        exp: Box::new(ExpKind::Name(String::from("a")))
                                    }
                                )
                            }
                        ]
                    // }
                    },
                },
            ],
        };

        let inter = Interpreter::new(prog);
        assert_eq!(1f64, Value::into_f64(inter.execute().unwrap().unwrap()).unwrap());
    }

    #[test]
    fn test_call_fn() {
        /*
        AST for program
        def other(a) {
            return a;
        }

        def main() {
            return other(3);
        }
        */
        let prog = Program {
            defs: vec![
                Def {
                    /*
                    def other(a) {
                        return a;
                    }
                    */
                    name: String::from("other"),
                    args: Args { names: vec![String::from("a")]},
                    block: Block {
                        statements: vec![
                            Statement {
                                statement: StatementKind::Return(
                                    Exp {
                                        exp: Box::new(ExpKind::Name(String::from("a")))
                                    }
                                )
                            }
                        ]
                    }
                },
                /*
                def main() {
                    return other(3);
                }
                */
                Def {
                    name: String::from("main"),
                    args: Args { names: Vec::new() },
                    block: Block {
                        statements: vec![
                            Statement {
                                // return other(3);
                                statement: StatementKind::Return(
                                    Exp {
                                        exp: Box::new(ExpKind::Call(String::from("other"),
                                        Exps { exps: vec![ Exp { exp: Box::new(ExpKind::Num(3f64))}] }))
                                    }
                                )
                            }
                        ]
                    },
                },
            ],
        };

        let inter = Interpreter::new(prog);
        assert_eq!(3f64, Value::into_f64(inter.execute().unwrap().unwrap()).unwrap());
    }
}
