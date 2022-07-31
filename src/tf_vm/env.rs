use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use crate::tf_vm::runtimes::RuntimeValue;

pub struct Env {
    parent: Option<Arc<RwLock<Env>>>,
    variables: HashMap<String, RuntimeValue>,
    arc_lock_self: Option<Arc<RwLock<Env>>>,
}

impl Env {
    pub fn new(parent: Option<Arc<RwLock<Env>>>) -> Arc<RwLock<Env>> {
        let arc_lock_self = Arc::new(RwLock::new(Env {
            parent,
            variables: HashMap::new(),
            arc_lock_self: None,
        }));
        arc_lock_self.write().unwrap().arc_lock_self = Some(arc_lock_self.clone());
        arc_lock_self
    }

    pub fn from(variables: HashMap<String, RuntimeValue>, parent: Option<Arc<RwLock<Env>>>) -> Arc<RwLock<Env>> {
        let arc_lock_self = Arc::new(RwLock::new(Env {
            parent,
            variables,
            arc_lock_self: None,
        }));
        arc_lock_self.write().unwrap().arc_lock_self = Some(arc_lock_self.clone());
        arc_lock_self
    }

    pub fn empty() -> Arc<RwLock<Env>> {
        Env::new(None)
    }

    pub fn get(&self, key: String) -> Option<RuntimeValue> {
        self.variables.get(key.as_str()).map(|i| (*i).clone()).
            or_else(|| self.parent.as_ref().map(|env| env.read().unwrap().get(key)).unwrap_or(None))
    }

    pub fn set(&mut self, key: String, value: RuntimeValue) {
        self.variables.insert(key, value);
    }

    pub fn merge(&self, env: Arc<RwLock<Env>>) -> Arc<RwLock<Env>> {
        let r_guard_env = env.read().unwrap();
        assert!(r_guard_env.parent.is_none(), "merged Env's parent should be None");
        Env::from(HashMap::clone(&r_guard_env.variables), self.arc_lock_self.clone())
    }
}
