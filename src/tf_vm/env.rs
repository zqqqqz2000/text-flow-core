use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use crate::Expr;
use derivative::Derivative;

pub struct Env {
    parent: Option<Arc<RwLock<Env>>>,
    variables: HashMap<String, RuntimeTypes>,
}

#[derive(Derivative)]
#[derivative(Debug, Clone)]
pub enum RuntimeTypes {
    Int64(i64),
    Int128(i128),
    String(Box<String>),
    Regex(Box<String>),
    List(Vec<Box<RuntimeTypes>>),
    None,
    FuncDef {
        parameters: Vec<Box<String>>,
        body: Box<Expr>,
        #[derivative(Debug = "ignore")]
        env: Arc<RwLock<Env>>,
    },
}

impl Env {
    pub fn new(parent: Option<Arc<RwLock<Env>>>) -> Env {
        Env {
            parent,
            variables: HashMap::new(),
        }
    }

    pub fn get(&self, key: String) -> Option<RuntimeTypes> {
        self.variables.get(key.as_str()).map(|i| (*i).clone()).
            or_else(|| self.parent.as_ref().map(|env| env.read().unwrap().get(key)).unwrap_or(None))
    }

    pub fn set(&mut self, key: String, value: RuntimeTypes) {
        self.variables.insert(key, value);
    }
}