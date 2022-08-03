use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use crate::ast::{Value, Op};
use crate::Expr;
use crate::tf_vm::env::Env;
use crate::tf_vm::runtimes::{BuiltinOrExpr, RuntimeValue};
use crate::utils::b;

pub struct VM;

fn get_from_vec(v: &Vec<Box<RuntimeValue>>, value: &RuntimeValue) -> Box<RuntimeValue> {
    match value {
        RuntimeValue::Int64(k) => {
            v[k.clone() as usize].clone()
        }
        RuntimeValue::Int128(k) => {
            v[k.clone() as usize].clone()
        }
        _ => panic!("can't get from {v:?}")
    }
}

impl VM {
    pub fn new() -> VM {
        VM
    }

    fn remove_code_pos(&self, expr: Box<Expr>) -> Box<Expr> {
        match *expr {
            Expr::ExprWithCodePos { exp, start: _, end: _ } => self.remove_code_pos(exp),
            _ => expr,
        }
    }
    pub fn eval(&self, env: Arc<RwLock<Env>>, asts: Vec<Box<Expr>>) -> Box<RuntimeValue> {
        let mut last = b(RuntimeValue::None);
        for ast in asts {
            let ast = self.remove_code_pos(ast);
            last = match *ast {
                Expr::Block(block) => self.eval(Arc::clone(&env), block),
                Expr::List(list) => b(RuntimeValue::List(
                    list.into_iter().map(|i| self.eval(Arc::clone(&env), vec![i])).collect()
                )),
                Expr::Value(value) => match value {
                    Value::String(string) => b(RuntimeValue::String(string)),
                    Value::Int64(int64) => b(RuntimeValue::Int64(int64)),
                    Value::Int128(int128) => b(RuntimeValue::Int128(int128)),
                    Value::Regex(regex) => b(RuntimeValue::Regex(regex)),
                },
                Expr::Variable(name) => {
                    let env = Arc::clone(&env);
                    let r_env = env.read().unwrap();
                    b(r_env.get(*name.clone()).
                        unwrap_or_else(|| panic!("can't find variable or token `{name:?}`")))
                }
                Expr::Op2 { op, x, y } => match op {
                    Op::Assign => {
                        let y = self.eval(Arc::clone(&env), vec![y]);
                        let env = Arc::clone(&env);
                        match *x {
                            Expr::Variable(name) => env.write().unwrap().set(*name, *y),
                            Expr::ExprWithCodePos { exp, start: _, end: _ } => match *exp {
                                Expr::Variable(name) => env.write().unwrap().set(*name, *y),
                                _ => panic!("assign not impl")
                            },
                            Expr::Get { from, key, is_expr } => {
                                let from = self.eval(Arc::clone(&env), vec![from]);
                                if !is_expr {
                                    match *from {
                                        RuntimeValue::WithEnv { env, value: _ } => {
                                            match *key {
                                                Expr::Variable(variable) => env.write().unwrap().set(*variable, *y),
                                                _ => panic!("only token can be assign")
                                            }
                                        }
                                        _ => panic!("only object can be assign value")
                                    }
                                } else {
                                    panic!("not implement assign to expr")
                                }
                            }
                            _ => panic!("assign not impl")
                        };
                        b(RuntimeValue::None)
                    }
                    Op::Add => {
                        let x = self.eval(Arc::clone(&env), vec![x]);
                        let y = self.eval(Arc::clone(&env), vec![y]);
                        match *x {
                            RuntimeValue::Int64(x) => match *y {
                                RuntimeValue::Int64(y) => b(RuntimeValue::Int64(x + y)),
                                RuntimeValue::Int128(y) => b(RuntimeValue::Int128(i128::from(x) + y)),
                                _ => panic!("{x:?} + {y:?} not impl")
                            },
                            RuntimeValue::Int128(x) => match *y {
                                RuntimeValue::Int64(y) => b(RuntimeValue::Int128(x + i128::from(y))),
                                RuntimeValue::Int128(y) => b(RuntimeValue::Int128(x + y)),
                                _ => panic!("{x:?} + {y:?} not impl")
                            },
                            RuntimeValue::String(x) => match *y {
                                RuntimeValue::String(y) => b(RuntimeValue::String(b(*x + y.as_str()))),
                                _ => panic!("{x:?} + {y:?} not impl")
                            }
                            _ => panic!("{x:?} + {y:?} not impl")
                        }
                    }
                    Op::Sub => {
                        let x = self.eval(Arc::clone(&env), vec![x]);
                        let y = self.eval(Arc::clone(&env), vec![y]);
                        match *x {
                            RuntimeValue::Int64(x) => match *y {
                                RuntimeValue::Int64(y) => b(RuntimeValue::Int64(x - y)),
                                RuntimeValue::Int128(y) => b(RuntimeValue::Int128(i128::from(x) - y)),
                                _ => panic!("add not impl")
                            },
                            RuntimeValue::Int128(x) => match *y {
                                RuntimeValue::Int64(y) => b(RuntimeValue::Int128(x - i128::from(y))),
                                RuntimeValue::Int128(y) => b(RuntimeValue::Int128(x - y)),
                                _ => panic!("add not impl")
                            },
                            _ => panic!("add not impl")
                        }
                    }
                    Op::Mul => {
                        let x = self.eval(Arc::clone(&env), vec![x]);
                        let y = self.eval(Arc::clone(&env), vec![y]);
                        match *x {
                            RuntimeValue::Int64(x) => match *y {
                                RuntimeValue::Int64(y) => b(RuntimeValue::Int64(x * y)),
                                RuntimeValue::Int128(y) => b(RuntimeValue::Int128(i128::from(x) * y)),
                                _ => panic!("add not impl")
                            },
                            RuntimeValue::Int128(x) => match *y {
                                RuntimeValue::Int64(y) => b(RuntimeValue::Int128(x * i128::from(y))),
                                RuntimeValue::Int128(y) => b(RuntimeValue::Int128(x * y)),
                                _ => panic!("add not impl")
                            },
                            _ => panic!("add not impl")
                        }
                    }
                    Op::Div => {
                        let x = self.eval(Arc::clone(&env), vec![x]);
                        let y = self.eval(Arc::clone(&env), vec![y]);
                        match *x {
                            RuntimeValue::Int64(x) => match *y {
                                RuntimeValue::Int64(y) => b(RuntimeValue::Int64(x / y)),
                                RuntimeValue::Int128(y) => b(RuntimeValue::Int128(i128::from(x) / y)),
                                _ => panic!("add not impl")
                            },
                            RuntimeValue::Int128(x) => match *y {
                                RuntimeValue::Int64(y) => b(RuntimeValue::Int128(x / i128::from(y))),
                                RuntimeValue::Int128(y) => b(RuntimeValue::Int128(x / y)),
                                _ => panic!("add not impl")
                            },
                            _ => panic!("add not impl")
                        }
                    }
                    _ => panic!("2op not impl")
                },
                Expr::FuncDef { parameters, body } => b(RuntimeValue::FuncDef {
                    parameters,
                    body: BuiltinOrExpr::Expr(body),
                    env: Env::new(Some(Arc::clone(&env))),
                }),
                Expr::Get { from, key, is_expr } => {
                    let from = self.eval(Arc::clone(&env), vec![from]);
                    if is_expr {
                        let key = self.eval(Arc::clone(&env), vec![key]);
                        match *from {
                            RuntimeValue::List(v) => {
                                get_from_vec(&v, key.as_ref())
                            }
                            _ => panic!("can't get from {from:?}, {key:?}")
                        }
                    } else {
                        match from.as_ref() {
                            RuntimeValue::WithEnv { env, value: _ } => {
                                match *key {
                                    Expr::Variable(variable) => {
                                        let scoped_value = env.read().unwrap().get(*variable.clone()).unwrap();
                                        match &scoped_value {
                                            RuntimeValue::FuncDef { parameters: _, body: _, env } => b({
                                                RuntimeValue::WithEnv {
                                                    env: Env::from(HashMap::from([
                                                        ("self".to_string(), *from)
                                                    ]), Some(env.clone())),
                                                    value: b(scoped_value),
                                                }
                                            }),
                                            _ => b(scoped_value)
                                        }
                                    }
                                    _ => panic!("can only get token")
                                }
                            }
                            _ => {
                                let t = from.get_type(Arc::clone(&env));
                                match *key {
                                    Expr::Variable(v) => {
                                        let value = t.get_env().read().unwrap().get(*v).unwrap();
                                        match &value {
                                            RuntimeValue::FuncDef { parameters: _, body: _, env } => b({
                                                RuntimeValue::WithEnv {
                                                    env: Env::from(HashMap::from([
                                                        ("self".to_string(), *from)
                                                    ]), Some(env.clone())),
                                                    value: b(value),
                                                }
                                            }),
                                            _ => b(value)
                                        }
                                    }
                                    Expr::Value(value) => match *from {
                                        RuntimeValue::List(v) => {
                                            get_from_vec(
                                                &v,
                                                self.eval(Arc::clone(&env), vec![b(Expr::Value(value))]).as_ref(),
                                            )
                                        }
                                        _ => panic!("can't get {value:?} from {from:?}")
                                    }
                                    _ => panic!("only can get variable or value from type {t:?}, not {key:?}")
                                }
                            }
                        }
                    }
                }
                Expr::FuncCall { func, arguments } => {
                    let func_def = self.eval(Arc::clone(&env), vec![func]);
                    let (parameters, func_body, func_env) = match *func_def {
                        RuntimeValue::FuncDef {
                            parameters,
                            body,
                            env
                        } => (parameters, body, env),
                        RuntimeValue::WithEnv {
                            value,
                            env: sub_env
                        } => match *value {
                            RuntimeValue::FuncDef { parameters, body, env: _ } => {
                                (parameters, body, sub_env)
                            }
                            _ => panic!("can't call {value:?}, it's not a function")
                        },
                        _ => panic!("can't call {func_def:?}, it's not a function")
                    };
                    arguments.into_iter().enumerate().for_each(|(i, arguments)| {
                        let argument = *self.eval(Arc::clone(&env), vec![arguments.clone()]);
                        match *self.remove_code_pos(arguments) {
                            Expr::Op2 { op, x, y } => match op {
                                Op::Assign => match *self.remove_code_pos(x) {
                                    Expr::Variable(variable) => func_env.write().unwrap().set(*variable, *self.eval(Arc::clone(&env), vec![y])),
                                    _ => panic!("Assign can't be here")
                                },
                                _ => func_env.write().unwrap().set(*parameters[i].clone(), argument),
                            },
                            _ => func_env.write().unwrap().set(*parameters[i].clone(), argument),
                        }
                    });
                    match func_body {
                        BuiltinOrExpr::Expr(expr) => {
                            self.eval(func_env, vec![expr])
                        }
                        BuiltinOrExpr::Builtin(builtin) => {
                            b(builtin(func_env))
                        }
                    }
                }
                _ => panic!("{ast:?} not impl")
            }
        }
        last
    }
}
