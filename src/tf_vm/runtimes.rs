use std::sync::{Arc, RwLock, RwLockReadGuard};
use crate::{Env, Expr};
use derivative::Derivative;

#[derive(Derivative)]
#[derivative(Debug, Clone)]
pub enum BuiltinOrExpr {
    Builtin(fn(Arc<RwLock<Env>>) -> RuntimeValue),
    Expr(Box<Expr>),
}

#[derive(Derivative)]
#[derivative(Debug, Clone)]
pub enum RuntimeValue {
    Int64(i64),
    Int128(i128),
    String(Box<String>),
    Regex(Box<String>),
    List(Vec<Box<RuntimeValue>>),
    None,
    FuncDef {
        parameters: Vec<Box<String>>,
        body: BuiltinOrExpr,
        #[derivative(Debug = "ignore")]
        env: Arc<RwLock<Env>>,
    },
    RuntimeType(RuntimeType),
    WithEnv {
        value: Box<RuntimeValue>,
        #[derivative(Debug = "ignore")]
        env: Arc<RwLock<Env>>,
    },
}

#[derive(Derivative)]
#[derivative(Debug, Clone)]
pub enum RuntimeType {
    Int64 {
        #[derivative(Debug = "ignore")]
        env: Arc<RwLock<Env>>,
    },
    Int128 {
        #[derivative(Debug = "ignore")]
        env: Arc<RwLock<Env>>,
    },
    String {
        #[derivative(Debug = "ignore")]
        env: Arc<RwLock<Env>>,
    },
    Regex {
        #[derivative(Debug = "ignore")]
        env: Arc<RwLock<Env>>,
    },
    List {
        #[derivative(Debug = "ignore")]
        env: Arc<RwLock<Env>>,
    },
    FuncDef {
        #[derivative(Debug = "ignore")]
        env: Arc<RwLock<Env>>,
    },
    None {
        #[derivative(Debug = "ignore")]
        env: Arc<RwLock<Env>>,
    },
}

impl RuntimeType {
    pub fn get_env(&self) -> Arc<RwLock<Env>> {
        use RuntimeType::{*};
        match self {
            Int64 { env } | Int128 { env } | String { env } | Regex { env } | List { env } | FuncDef { env } | None { env } => {
                env.clone()
            }
        }
    }

    pub fn name(&self) -> String {
        use RuntimeType::{*};
        match self {
            Int64 { env: _ } => "i64".to_string(),
            Int128 { env: _ } => "i128".to_string(),
            String { env: _ } => "str".to_string(),
            Regex { env: _ } => "reg".to_string(),
            List { env: _ } => "list".to_string(),
            FuncDef { env: _ } => "fun".to_string(),
            None { env: _ } => "none".to_string()
        }
    }
}


fn get_value_type_name(t: &RuntimeValue) -> String {
    use RuntimeValue::{*};
    match t {
        Int64(_) => "i64".to_string(),
        Int128(_) => "i128".to_string(),
        String(_) => "str".to_string(),
        Regex(_) => "reg".to_string(),
        List(_) => "list".to_string(),
        FuncDef { parameters: _, body: _, env: _ } => "fun".to_string(),
        None => "none".to_string(),
        RuntimeType(t) => t.name(),
        WithEnv {value, env: _} => get_value_type_name(value)
    }
}

fn get_type_env(env: RwLockReadGuard<Env>, value: &RuntimeValue) -> Arc<RwLock<Env>> {
    match env.get(get_value_type_name(value)).unwrap() {
        RuntimeValue::RuntimeType(t) => t.get_env(),
        _ => panic!("value is not runtimeType, is")
    }
}

impl RuntimeValue {
    pub fn get_type(&self, env: Arc<RwLock<Env>>) -> RuntimeType {
        let rw_guard_env = env.read().unwrap();
        let type_env = get_type_env(rw_guard_env, self);
        match self {
            RuntimeValue::Int64(_) => RuntimeType::Int64 { env: type_env },
            RuntimeValue::Int128(_) => RuntimeType::Int128 { env: type_env },
            RuntimeValue::String(_) => RuntimeType::String { env: type_env },
            RuntimeValue::Regex(_) => RuntimeType::Regex { env: type_env },
            RuntimeValue::List(_) => RuntimeType::List { env: type_env },
            RuntimeValue::None => RuntimeType::None { env: type_env },
            RuntimeValue::FuncDef { parameters: _, body: _, env: _ } => RuntimeType::FuncDef { env: type_env },
            RuntimeValue::RuntimeType(r) => r.clone(),
            RuntimeValue::WithEnv {env: _, value} => value.get_type(env)
        }
    }
}
