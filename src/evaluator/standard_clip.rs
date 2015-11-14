use std::cmp::Eq;
use std::fmt::Debug;
use std::rc::Rc;
use std::cell::{RefCell, Ref, RefMut};
use std::hash::{Hash, Hasher};
use std::mem;
use std::collections::HashMap;

use super::values::Value;
use super::environment::Environment;
use super::clip::Clip;
use super::vm::execute;
use super::ops::Op;

#[derive(Debug)]
pub struct StdClip {
    params: Vec<String>,
    returns: Vec<String>,
    ops: Vec<Op>,
    defs: HashMap<Value, Value>
}

impl Clip for StdClip {
    fn get(&self, key: &Value) -> Value{
        match self.defs.get(key) {
            Some(v) => v.clone(),
            None => Value::Nil
        }
    }

    fn set(&mut self, key: Value, value: Value) -> Result<(), String> {
        self.defs.insert(key, value);
        Ok(())
    }

    fn play(&mut self, params: Vec<Value>, environment: &mut Environment) -> Result<Value, String> {
        for (ident, value) in self.params.iter().zip(params.into_iter()) {
            environment.set_var(ident.clone(), value);
        }
        let mut stack = Vec::new();
        try!(execute(&self.ops, &mut stack, environment, &mut self.defs));
        if self.returns.len() == 0 {
            Ok(Value::Nil)
        } else if self.returns.len() == 1 {
            Ok(environment.get_var(&self.returns[0]).unwrap_or(Value::Nil).clone())
        } else {
            let mut ret_vec = self.returns.iter().map(|ret| environment.get_var(ret).unwrap_or(Value::Nil).clone()).collect();
            Ok(Value::Tuple(ret_vec))
        }
    }
}
