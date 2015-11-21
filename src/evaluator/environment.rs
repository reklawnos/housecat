use super::value::Value;
use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct ValueHolder {
    value: Value
}

impl ValueHolder {
    pub fn new(value: Value) -> ValueHolder {
        ValueHolder{value: value}
    }

    pub fn set(&mut self, value: Value) {
        self.value = value;
    }

    pub fn get(&self) -> Value {
        return self.value.clone();
    }
}

#[derive(Debug, Clone)]
enum EnvValue {
    Immutable(Value),
    Plain(Value),
    Referenced(Rc<RefCell<ValueHolder>>)
}

pub enum RefType {
    Copy(Value),
    Ref(Rc<RefCell<ValueHolder>>),
    None
}

pub struct Environment {
    defs: Vec<HashMap<String, EnvValue>>
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            defs: Vec::new()
        }
    }

    pub fn declare_var(&mut self, name: String, value: Value) {
        let top_idx = self.defs.len() - 1;
        self.defs[top_idx].insert(name, EnvValue::Plain(value));
    }

    pub fn declare_immutable(&mut self, name: String, value: Value) {
        let top_idx = self.defs.len() - 1;
        self.defs[top_idx].insert(name, EnvValue::Immutable(value));
    }

    pub fn set_var(&mut self, name: String, value: Value) -> Result<(), String>{
        for scope in self.defs.iter_mut().rev() {
            let val = scope.remove(&name[..]);
            match val {
                Some(EnvValue::Immutable(old_value)) => {
                    scope.insert(name.clone(), EnvValue::Immutable(old_value));
                    return Err(format!("Cannot assign to {} because it is immutable", name));
                }
                Some(EnvValue::Plain(_)) => { scope.insert(name, EnvValue::Plain(value)); }
                Some(EnvValue::Referenced(ref rv)) => {
                    rv.borrow_mut().set(value);
                }
                _ => continue
            }
            return Ok(());
        }
        Err(format!("Expected to find ident `{}`, but it wasn't found in any scope", name))
    }

    pub fn get_var(&mut self, name: &String) -> Option<Value> {
        for scope in self.defs.iter().rev() {
            if let Some(v) = scope.get(&name[..]) {
                match *v {
                    EnvValue::Immutable(ref value) => { return Some(value.clone()); }
                    EnvValue::Plain(ref value) => { return Some(value.clone()); }
                    EnvValue::Referenced(ref value) => { return Some(value.borrow().get()); }
                }
            }
        }
        None
    }

    pub fn get_ref(&mut self, name: String) -> RefType {
        for scope in self.defs.iter_mut().rev() {
            let val = scope.remove(&name[..]);
            if let Some(v) = val {
                match v {
                    EnvValue::Immutable(value) => {
                        scope.insert(name, EnvValue::Immutable(value.clone()));
                        return RefType::Copy(value);
                    }
                    EnvValue::Plain(value) => {
                        let result = Rc::new(RefCell::new(ValueHolder::new(value)));
                        scope.insert(name, EnvValue::Referenced(result.clone()));
                        return RefType::Ref(result);
                    }
                    EnvValue::Referenced(value) => {
                        scope.insert(name, EnvValue::Referenced(value.clone()));
                        return RefType::Ref(value);
                    }
                }
            }
        }
        RefType::None
    }

    pub fn push_frame(&mut self) {
        self.defs.push(HashMap::new());
    }

    pub fn pop_frame(&mut self) {
        self.defs.pop();
    }
}
