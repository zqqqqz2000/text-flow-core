#[cfg(test)]
mod tests {
    use crate::ast::Op::{Add, Ge, Gt, Mul};
    use crate::ast::{Value};
    use crate::Expr;
    use crate::Expr::{ExprWithCodePos, FuncCall, Get, Op2, Variable};
    use crate::utils::b;
    lalrpop_mod!(pub text_flow);

    #[test]
    fn parse() {
        let parser = text_flow::ExprsParser::new();
        let cases = vec![
            ("", vec![]),
            ("1", vec![
                b(
                    ExprWithCodePos {
                        exp: b(ExprWithCodePos {
                            exp: b(Expr::Value(Value::Int64(1i64))),
                            start: 0,
                            end: 1,
                        }),
                        start: 0,
                        end: 1,
                    }
                )
            ]),
            ("111111111111111111", vec![
                b(
                    ExprWithCodePos {
                        exp: b(ExprWithCodePos {
                            exp: b(Expr::Value(Value::Int128(111111111111111111i128))),
                            start: 0,
                            end: 18,
                        }),
                        start: 0,
                        end: 18,
                    }
                )
            ]),
            ("\"abc\"", vec![
                b(
                    ExprWithCodePos {
                        exp: b(ExprWithCodePos {
                            exp: b(Expr::Value(Value::String(b("abc".to_string())))),
                            start: 0,
                            end: 5,
                        }),
                        start: 0,
                        end: 5,
                    }
                )
            ]),
            ("'abc'", vec![
                b(
                    ExprWithCodePos {
                        exp: b(ExprWithCodePos {
                            exp: b(Expr::Value(Value::String(b("abc".to_string())))),
                            start: 0,
                            end: 5,
                        }),
                        start: 0,
                        end: 5,
                    }
                )
            ]),
            ("/abc/", vec![
                b(
                    ExprWithCodePos {
                        exp: b(ExprWithCodePos {
                            exp: b(Expr::Value(Value::Regex(b("abc".to_string())))),
                            start: 0,
                            end: 5,
                        }),
                        start: 0,
                        end: 5,
                    }
                )
            ]),
            ("a+b", vec![
                b(
                    ExprWithCodePos {
                        exp: b(Op2 {
                            op: Add,
                            x: b(ExprWithCodePos {
                                exp: b(Variable(b("a".to_string()))),
                                start: 0,
                                end: 1
                            }),
                            y: b(ExprWithCodePos {
                                exp: b(Variable(b("b".to_string()))),
                                start: 2,
                                end: 3
                            }),
                        }),
                        start: 0,
                        end: 3,
                    }
                )
            ]),
            ("a>b", vec![
                b(
                    ExprWithCodePos {
                        exp: b(Op2 {
                            op: Gt,
                            x: b(ExprWithCodePos {
                                exp: b(Variable(b("a".to_string()))),
                                start: 0,
                                end: 1
                            }),
                            y: b(ExprWithCodePos {
                                exp: b(Variable(b("b".to_string()))),
                                start: 2,
                                end: 3
                            }),
                        }),
                        start: 0,
                        end: 3,
                    }
                )
            ]),
            ("a>=b", vec![
                b(
                    ExprWithCodePos {
                        exp: b(Op2 {
                            op: Ge,
                            x: b(ExprWithCodePos {
                                exp: b(Variable(b("a".to_string()))),
                                start: 0,
                                end: 1
                            }),
                            y: b(ExprWithCodePos {
                                exp: b(Variable(b("b".to_string()))),
                                start: 3,
                                end: 4
                            }),
                        }),
                        start: 0,
                        end: 4,
                    }
                )
            ]),
            ("a*b+c", vec![
                b(
                    ExprWithCodePos {
                        exp: b(Op2 {
                            op: Add,
                            x: b(Op2 {
                                    op: Mul,
                                    x: b(ExprWithCodePos {
                                        exp: b(Variable(b("a".to_string()))),
                                        start: 0,
                                        end: 1
                                    }),
                                    y: b(ExprWithCodePos {
                                        exp: b(Variable(b("b".to_string()))),
                                        start: 2,
                                        end: 3
                                    }),
                                }),
                            y: b(ExprWithCodePos {
                                exp: b(Variable(b("c".to_string()))),
                                start: 4,
                                end: 5
                            }),
                        }),
                        start: 0,
                        end: 5,
                    }
                )
            ]),
            ("c+a*b", vec![
                b(
                    ExprWithCodePos {
                        exp: b(Op2 {
                            op: Add,
                            x: b(ExprWithCodePos {
                                exp: b(Variable(b("c".to_string()))),
                                start: 0,
                                end: 1
                            }),
                            y: b(Op2 {
                                op: Mul,
                                x: b(ExprWithCodePos {
                                    exp: b(Variable(b("a".to_string()))),
                                    start: 2,
                                    end: 3
                                }),
                                y: b(ExprWithCodePos {
                                    exp: b(Variable(b("b".to_string()))),
                                    start: 4,
                                    end: 5
                                }),
                            }),
                        }),
                        start: 0,
                        end: 5,
                    }
                )
            ]),
            ("a.b", vec![
                b(ExprWithCodePos {
                    exp: b(ExprWithCodePos{
                        exp:b(Get {
                            from: b(ExprWithCodePos {
                                exp: b(Variable(b("a".to_string()))),
                                start: 0,
                                end: 1
                            }),
                            key: b(Variable(b("b".to_string()))),
                            is_expr: false
                        }),
                        start: 0,
                        end: 3
                    }),
                    start: 0,
                    end: 3
                })
            ]),
            ("a+b[]", vec![
                b(
                    ExprWithCodePos {
                        exp: b(Op2 {
                            op: Add,
                            x: b(ExprWithCodePos {
                                exp: b(Variable(b("a".to_string()))),
                                start: 0,
                                end: 1
                            }),
                            y: b(ExprWithCodePos {
                                exp: b(FuncCall {
                                    func: b(ExprWithCodePos {
                                        exp: b(Variable(b("b".to_string()))),
                                        start: 2,
                                        end: 3
                                    }),
                                    arguments: vec![]
                                }),
                                start: 2,
                                end: 5
                            }),
                        }),
                        start: 0,
                        end: 5,
                    }
                )
            ]),
        ];
        for (expr, should_ast) in cases {
            assert_eq!(parser.parse(expr).unwrap(), should_ast);
        }
    }
}
