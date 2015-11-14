use super::values::Value;
use std::collections::HashMap;

pub struct Environment {
    defs: Vec<HashMap<String, Value>>
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            defs: Vec::new()
        }
    }

    pub fn set_var(&mut self, name: String, value: Value) {
        let top_idx = self.defs.len() - 1;
        self.defs[top_idx].insert(name, value);
    }

    pub fn get_var(&mut self, name: &String) -> Option<Value> {
        for scope in self.defs.iter().rev() {
            match scope.get(&name[..]) {
                Some(v) => { return Some(v.clone()); }
                None => continue
            }
        }
        None
    }

    pub fn push_frame(&mut self) {
        self.defs.push(HashMap::new());
    }

    pub fn pop_frame(&mut self) {
        self.defs.pop();
    }
}
