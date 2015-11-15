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

impl StdClip {
    pub fn new(params: Vec<String>, returns: Vec<String>, ops: Vec<Op>) -> StdClip {
        StdClip {
            params: params,
            returns: returns,
            ops: ops,
            defs: HashMap::new()
        }
    }
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
            environment.declare_var(ident.clone(), value);
        }
        for ident in self.returns.iter() {
            environment.declare_var(ident.clone(), Value::Int(-10));
        }
        let mut stack = Vec::new();
        try!(execute(&self.ops, &mut stack, environment, &mut self.defs));
        if self.returns.len() == 0 {
            Ok(Value::Nil)
        } else if self.returns.len() == 1 {
            Ok(environment.get_var(&self.returns[0]).unwrap_or(Value::Nil))
        } else {
            let ret_vec = self.returns.iter().map(|ret| environment.get_var(ret).unwrap_or(Value::Nil)).collect();
            Ok(Value::Tuple(ret_vec))
        }
    }
}
