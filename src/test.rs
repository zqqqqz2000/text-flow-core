#[cfg(test)]
mod tests {
    use crate::ast::Value;
    use crate::Expr;
    use crate::Expr::ExprWithCodePos;
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
        ];
        for (expr, should_ast) in cases {
            assert_eq!(parser.parse(expr).unwrap(), should_ast);
        }
    }
}
