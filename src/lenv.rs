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
        match self.contents.get(key) {
            Some(value) => value.clone(),
            None => {
                // Search in parent env, if possible
                match self.parent {
                    Some(ref env) => unsafe {
                        (**env).get(key)
                    },
                    None => err!("unbound symbol: {}", key)
                }
            }
        }
    }

    pub fn put(&mut self, mut key: LVal, value: LVal) {
        self.contents.insert(key.as_sym().clone(), value.clone());
    }

    pub fn def(&mut self, key: LVal, value: LVal) {
        match self.parent {
            Some(ref env) => {
                unsafe { (**env).def(key, value); }
            },
            None => {
                self.put(key, value);
            }
        }
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