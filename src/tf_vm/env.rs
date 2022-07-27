use std::collections::HashMap;

pub struct Env {
    parent: Option<Box<Env>>,
    global: Option<Box<Env>>,
    variables: HashMap<String, RuntimeTypes>
}

#[derive(Debug, Clone)]
pub enum RuntimeTypes {
    Int64(i64),
    Int128(i128),
    String(Box<String>),
    Regex(Box<String>),
    List(Vec<Box<RuntimeTypes>>),
    None,
}

impl Env {
    pub fn new(global: Option<Box<Env>>, parent: Option<Box<Env>>) -> Env {
        Env {
            parent,
            global,
            variables: HashMap::new()
        }
    }

    pub fn get(&self, key: String) -> Option<RuntimeTypes> {
        self.variables.get(key.as_str()).map(|i|i.clone()).
            or_else(||self.parent.as_ref().map(|env|env.get(key.to_string())).unwrap_or(None)).
            or_else(||self.global.as_ref().map(|env|env.get(key)).unwrap_or(None))
    }

    pub fn set(&mut self, key: String, value: RuntimeTypes) {
        self.variables.insert(key, value);
    }
}