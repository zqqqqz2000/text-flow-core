use std::sync::{Arc, RwLock};
use crate::{Env, Expr};
use crate::tf_vm::runtimes::RuntimeValue;

pub fn get_var_from_env(env: Arc<RwLock<Env>>, variable: Expr) -> Option<RuntimeValue> {
    if let Expr::Variable(var_name) = variable {
        env.read().unwrap().get(*var_name)
    } else {
        None
    }
}

pub fn get_name_from_env(env: Arc<RwLock<Env>>, key: String) -> Option<RuntimeValue> {
    env.read().unwrap().get(key)
}

pub fn set_name_from_env(env: Arc<RwLock<Env>>, key: String, value: RuntimeValue) {
    env.write().unwrap().set(key, value)
}

pub fn get_self_from_env(env: Arc<RwLock<Env>>) -> Option<RuntimeValue> {
    get_name_from_env(env, "self".to_string())
}
