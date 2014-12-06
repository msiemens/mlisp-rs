use std::collections::HashMap;
use std::fmt;
use lval::LVal;


pub struct LEnv {
    pub parent: Option<*mut LEnv>,
    contents: HashMap<String, LVal>
}

impl LEnv {
    pub fn new() -> LEnv {
        LEnv {
            parent: None,
            contents: HashMap::new()
        }
    }

    pub fn get(&self, key: &str) -> LVal {
        if let Some(value) = self.contents.get(key) {
            value.clone()
        } else {
            // Search in parent env, if possible
            if let Some(ref env) = self.parent {
                unsafe { (**env).get(key) }
            } else {
                err!("unbound symbol: {}", key)
            }
        }
    }

    pub fn put(&mut self, key: LVal, value: LVal) {
        self.contents.insert(key.as_sym().clone(), value.clone());
    }

    pub fn def(&mut self, key: LVal, value: LVal) {
        if let Some(ref env) = self.parent {
            unsafe { (**env).def(key, value); }
        } else {
            self.put(key, value);
        }
    }

    pub fn look_up(&self, search: &LVal) -> Option<&str> {
        self.contents.iter()
            .find(|&(_, value)| value == search)
            .map(|(key, _)| key[])
    }
}

impl fmt::Show for LEnv {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.contents)
    }
}

impl PartialEq for LEnv {
    fn eq(&self, other: &LEnv) -> bool {
        self.contents.eq(&other.contents)
    }
}

impl Clone for LEnv {
    fn clone(&self) -> LEnv {
        LEnv {
            parent: self.parent,
            contents: self.contents.clone()
        }
    }
}