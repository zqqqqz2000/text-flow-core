use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use crate::{Env};
use crate::tf_vm::runtimes::{BuiltinOrExpr, RuntimeType, RuntimeValue};
use crate::tf_vm::utils::{get_name_from_env, get_self_from_env, set_name_from_env};
use crate::utils::b;

pub fn init_builtin() -> Arc<RwLock<Env>> {
    let env = Env::empty();
    let gen_get_type = || ("type".to_string(), RuntimeValue::FuncDef {
        parameters: vec![b("self".to_string())],
        body: BuiltinOrExpr::Builtin(|env| {
            RuntimeValue::String(b(get_self_from_env(env.clone()).unwrap().get_type(env).name()))
        }),
        env: env.clone(),
    });
    env.write().unwrap().update_variables(HashMap::from([
        ("i64".to_string(), RuntimeValue::RuntimeType(
            RuntimeType::Int64 {
                env: Env::from(HashMap::from([
                    ("str".to_string(), RuntimeValue::FuncDef {
                        parameters: vec![b("self".to_string())],
                        body: BuiltinOrExpr::Builtin(|env| match get_self_from_env(env).unwrap() {
                            RuntimeValue::Int64(i) => RuntimeValue::String(b(i.to_string())),
                            _ => panic!("internal error, should only be i64")
                        }),
                        env: env.clone(),
                    }),
                    gen_get_type()
                ]), None)
            }
        )),
        ("i128".to_string(), RuntimeValue::RuntimeType(
            RuntimeType::Int128 {
                env: Env::from(HashMap::from([
                    ("str".to_string(), RuntimeValue::FuncDef {
                        parameters: vec![b("self".to_string())],
                        body: BuiltinOrExpr::Builtin(|env| match get_self_from_env(env).unwrap() {
                            RuntimeValue::Int128(i) => RuntimeValue::String(b(i.to_string())),
                            _ => panic!("internal error, should only be i128")
                        }),
                        env: env.clone(),
                    }),
                    gen_get_type()
                ]), None)
            }
        )),
        ("str".to_string(), RuntimeValue::RuntimeType(
            RuntimeType::String {
                env: Env::from(HashMap::from([
                    gen_get_type()
                ]), None)
            }
        )),
        ("reg".to_string(), RuntimeValue::RuntimeType(
            RuntimeType::Regex {
                env: Env::from(HashMap::from([
                    gen_get_type()
                ]), None)
            }
        )),
        ("list".to_string(), RuntimeValue::RuntimeType(
            RuntimeType::List {
                env: Env::from(HashMap::from([
                    gen_get_type(),
                    ("len".to_string(), RuntimeValue::FuncDef {
                        parameters: vec![b("self".to_string())],
                        body: BuiltinOrExpr::Builtin(
                            |env| match get_self_from_env(env).unwrap() {
                                RuntimeValue::List(list) => RuntimeValue::Int128(
                                    list.len() as i128
                                ),
                                _ => panic!("only list have len")
                            }
                        ),
                        env: env.clone(),
                    }),
                    ("iter".to_string(), RuntimeValue::FuncDef {
                        parameters: vec![b("self".to_string())],
                        body: BuiltinOrExpr::Builtin(
                            |env| {
                                let self_value = get_self_from_env(env.clone()).unwrap();
                                if let RuntimeValue::List(_) = self_value {
                                    RuntimeValue::WithEnv {
                                        env: Env::from(HashMap::from([
                                            ("next".to_string(), RuntimeValue::FuncDef {
                                                parameters: vec![b("self".to_string())],
                                                body: BuiltinOrExpr::Builtin(
                                                    |env| {
                                                        let self_value = get_self_from_env(env.clone()).unwrap();
                                                        if let RuntimeValue::WithEnv { env, value } = self_value {
                                                            if let RuntimeValue::Int64(current) = get_name_from_env(env.clone(), "current".to_string()).unwrap_or(RuntimeValue::Int64(0)) {
                                                                if let RuntimeValue::List(list) = *value {
                                                                    set_name_from_env(env, "current".to_string(), RuntimeValue::Int64(current + 1));
                                                                    *list[current as usize].clone()
                                                                } else {
                                                                    panic!("must be list")
                                                                }
                                                            } else {
                                                                panic!("must be int64")
                                                            }
                                                        } else {
                                                            panic!("must be obj")
                                                        }
                                                    }
                                                ),
                                                env: env.clone(),
                                            })
                                        ]), None),
                                        value: b(self_value),
                                    }
                                } else {
                                    panic!("only list can")
                                }
                            },
                        ),
                        env: env.clone(),
                    })
                ]), None)
            }
        )),
        ("obj".to_string(), RuntimeValue::FuncDef {
            parameters: vec![b("value".to_string())],
            body: BuiltinOrExpr::Builtin(|env| RuntimeValue::WithEnv {
                env: Env::new(Some(env.clone())),
                value: b(get_name_from_env(env, "value".to_string()).unwrap_or(RuntimeValue::None)),
            }),
            env: env.clone(),
        })
    ]));
    env
}
