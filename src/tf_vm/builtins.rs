use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use crate::{Env};
use crate::tf_vm::runtimes::{BuiltinOrExpr, RuntimeType, RuntimeValue};
use crate::utils::b;

fn get_from_env(env: Arc<RwLock<Env>>, key: String) -> RuntimeValue {
    env.read().unwrap().get(key).unwrap()
}

fn get_self_from_env(env: Arc<RwLock<Env>>) -> RuntimeValue {
    get_from_env(env, "self".to_string())
}

pub fn init_builtin() -> Arc<RwLock<Env>> {
    Env::from(HashMap::from([
        ("i64".to_string(), RuntimeValue::RuntimeType(
            RuntimeType::Int64 {
                env: Env::from(HashMap::from([
                    ("str".to_string(), RuntimeValue::FuncDef {
                        parameters: vec![b("self".to_string())],
                        body: BuiltinOrExpr::Builtin(|env| match get_self_from_env(env) {
                            RuntimeValue::Int64(i) => RuntimeValue::String(b(i.to_string())),
                            _ => panic!("internal error, should only be i64")
                        }),
                        env: Env::empty(),
                    }),
                ]), None)
            }
        )),
        ("i128".to_string(), RuntimeValue::RuntimeType(
            RuntimeType::Int128 {
                env: Env::from(HashMap::from([
                    ("str".to_string(), RuntimeValue::FuncDef {
                        parameters: vec![b("self".to_string())],
                        body: BuiltinOrExpr::Builtin(|env| match get_self_from_env(env) {
                            RuntimeValue::Int128(i) => RuntimeValue::String(b(i.to_string())),
                            _ => panic!("internal error, should only be i128")
                        }),
                        env: Env::empty(),
                    }),
                ]), None)
            }
        ))
    ]), None)
}
