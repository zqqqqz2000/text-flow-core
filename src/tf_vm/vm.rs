use std::borrow::{Borrow};
use std::sync::{Arc, RwLock};
use crate::ast::{Value, Op};
use crate::Expr;
use crate::tf_vm::env::{Env, RuntimeTypes};

pub fn b<T>(i: T) -> Box<T> {
    Box::new(i)
}

pub struct VM();

impl VM {
    pub fn new() -> VM {
        VM()
    }
    fn remove_code_pos(&self, expr: Box<Expr>) -> Box<Expr> {
        match *expr {
            Expr::ExprWithCodePos { exp, start: _, end: _ } => self.remove_code_pos(exp),
            _ => expr,
        }
    }
    pub fn eval(&self, env: Arc<RwLock<Env>>, asts: Vec<Box<Expr>>) -> Box<RuntimeTypes> {
        let mut last = b(RuntimeTypes::None);
        for ast in asts {
            let ast = self.remove_code_pos(ast);
            last = match *ast {
                Expr::Block(block) => self.eval(Arc::clone(&env), block),
                Expr::List(list) => b(RuntimeTypes::List(
                    list.into_iter().map(|i| self.eval(Arc::clone(&env), vec![i])).collect()
                )),
                Expr::Value(value) => match value {
                    Value::String(string) => b(RuntimeTypes::String(string)),
                    Value::Int64(int64) => b(RuntimeTypes::Int64(int64)),
                    Value::Int128(int128) => b(RuntimeTypes::Int128(int128)),
                    Value::Regex(regex) => b(RuntimeTypes::Regex(regex)),
                },
                Expr::Variable(name) => {
                    let env = Arc::clone(&env);
                    let r_env = env.read().unwrap();
                    b(r_env.borrow().get(*name.clone()).
                        unwrap_or_else(|| panic!("can't find variable or token `{name:?}`")))
                }
                Expr::Op2 { op, x, y } => match op {
                    Op::Assign => {
                        let y = self.eval(Arc::clone(&env), vec![y]);
                        let env = Arc::clone(&env);
                        let mut w_env = env.write().unwrap();
                        match *x {
                            Expr::Variable(name) => w_env.set(*name, *y),
                            Expr::ExprWithCodePos { exp, start: _, end: _ } => match *exp {
                                Expr::Variable(name) => w_env.set(*name, *y),
                                _ => panic!("assign not impl")
                            },
                            _ => panic!("assign not impl")
                        };
                        b(RuntimeTypes::None)
                    }
                    Op::Add => {
                        let x = self.eval(Arc::clone(&env), vec![x]);
                        let y = self.eval(Arc::clone(&env), vec![y]);
                        match *x {
                            RuntimeTypes::Int64(x) => match *y {
                                RuntimeTypes::Int64(y) => b(RuntimeTypes::Int64(x + y)),
                                RuntimeTypes::Int128(y) => b(RuntimeTypes::Int128(i128::from(x) + y)),
                                _ => panic!("{x:?} + {y:?} not impl")
                            },
                            RuntimeTypes::Int128(x) => match *y {
                                RuntimeTypes::Int64(y) => b(RuntimeTypes::Int128(x + i128::from(y))),
                                RuntimeTypes::Int128(y) => b(RuntimeTypes::Int128(x + y)),
                                _ => panic!("{x:?} + {y:?} not impl")
                            },
                            RuntimeTypes::String(x) => match *y {
                                RuntimeTypes::String(y) => b(RuntimeTypes::String(b(*x + y.as_str()))),
                                _ => panic!("{x:?} + {y:?} not impl")
                            }
                            _ => panic!("{x:?} + {y:?} not impl")
                        }
                    }
                    Op::Sub => {
                        let x = self.eval(Arc::clone(&env), vec![x]);
                        let y = self.eval(Arc::clone(&env), vec![y]);
                        match *x {
                            RuntimeTypes::Int64(x) => match *y {
                                RuntimeTypes::Int64(y) => b(RuntimeTypes::Int64(x - y)),
                                RuntimeTypes::Int128(y) => b(RuntimeTypes::Int128(i128::from(x) - y)),
                                _ => panic!("add not impl")
                            },
                            RuntimeTypes::Int128(x) => match *y {
                                RuntimeTypes::Int64(y) => b(RuntimeTypes::Int128(x - i128::from(y))),
                                RuntimeTypes::Int128(y) => b(RuntimeTypes::Int128(x - y)),
                                _ => panic!("add not impl")
                            },
                            _ => panic!("add not impl")
                        }
                    }
                    Op::Mul => {
                        let x = self.eval(Arc::clone(&env), vec![x]);
                        let y = self.eval(Arc::clone(&env), vec![y]);
                        match *x {
                            RuntimeTypes::Int64(x) => match *y {
                                RuntimeTypes::Int64(y) => b(RuntimeTypes::Int64(x * y)),
                                RuntimeTypes::Int128(y) => b(RuntimeTypes::Int128(i128::from(x) * y)),
                                _ => panic!("add not impl")
                            },
                            RuntimeTypes::Int128(x) => match *y {
                                RuntimeTypes::Int64(y) => b(RuntimeTypes::Int128(x * i128::from(y))),
                                RuntimeTypes::Int128(y) => b(RuntimeTypes::Int128(x * y)),
                                _ => panic!("add not impl")
                            },
                            _ => panic!("add not impl")
                        }
                    }
                    Op::Div => {
                        let x = self.eval(Arc::clone(&env), vec![x]);
                        let y = self.eval(Arc::clone(&env), vec![y]);
                        match *x {
                            RuntimeTypes::Int64(x) => match *y {
                                RuntimeTypes::Int64(y) => b(RuntimeTypes::Int64(x / y)),
                                RuntimeTypes::Int128(y) => b(RuntimeTypes::Int128(i128::from(x) / y)),
                                _ => panic!("add not impl")
                            },
                            RuntimeTypes::Int128(x) => match *y {
                                RuntimeTypes::Int64(y) => b(RuntimeTypes::Int128(x / i128::from(y))),
                                RuntimeTypes::Int128(y) => b(RuntimeTypes::Int128(x / y)),
                                _ => panic!("add not impl")
                            },
                            _ => panic!("add not impl")
                        }
                    }
                    _ => panic!("2op not impl")
                },
                Expr::FuncDef { parameters, body } => b(RuntimeTypes::FuncDef {
                    parameters,
                    body,
                    env: Arc::new(RwLock::new(Env::new(Some(Arc::clone(&env))))),
                }),
                Expr::FuncCall { func, arguments } => {
                    let func_def = self.eval(Arc::clone(&env), vec![func]);
                    let (parameters, func_body, func_env) = match *func_def {
                        RuntimeTypes::FuncDef {
                            parameters,
                            body,
                            env
                        } => (parameters, body, env),
                        _ => panic!("can't call {func_def:?}, it's not a function")
                    };
                    arguments.into_iter().enumerate().for_each(|(i, arguments)| match *self.remove_code_pos(arguments.clone()) {
                        Expr::Op2 { op, x, y } => match op {
                            Op::Assign => match *self.remove_code_pos(x) {
                                Expr::Variable(variable) => func_env.write().unwrap().set(*variable, *self.eval(Arc::clone(&env), vec![y])),
                                _ => panic!("Assign can't be here")
                            },
                            _ => func_env.write().unwrap().set(*parameters[i].clone(), *self.eval(Arc::clone(&env), vec![arguments])),
                        },
                        _ => func_env.write().unwrap().set(*parameters[i].clone(), *self.eval(Arc::clone(&env), vec![arguments])),
                    });
                    self.eval(func_env, vec![func_body])
                }
                _ => panic!("{ast:?} not impl")
            }
        }
        last
    }
}
