use std::collections::HashMap;
use lval::LVal;


pub struct LEnv {
    contents: HashMap<String, LVal>
}

impl LEnv {
    pub fn new() -> LEnv {
        LEnv {
            contents: HashMap::new()
        }
    }

    pub fn get(&self, key: &str) -> LVal {
        match self.contents.get(key) {
            Some(value) => value.clone(),
            None => err!("unbound symbol: {}", key)
        }
    }

    pub fn put(&mut self, mut key: LVal, value: LVal) {
        self.contents.insert(key.as_sym().clone(), value.clone());
    }
}