use evaluator::stack_evaluator::values::{Value, RustClip, RustHolder};
use std::collections::HashMap;


#[derive(Debug)]
struct Print;

#[allow(unused_variables, dead_code)]
impl<'a> RustClip<'a> for Print {
    fn get(&self, key: &str) -> Option<Value<'a>> {
        None
    }
    fn set(&mut self, key: &str, value: Value<'a>) -> Result<(), &str> {
        Err("Cannot set a def on print")
    }
    fn call(&mut self, args: Vec<Value<'a>>) -> Result<Value<'a>, &str> {
        if args.len() == 1 {
            println!("{}", args[0]);
            Ok(Value::Nil)
        } else {
            Err("Wrong number of args for `print`")
        }
    }
}

#[allow(dead_code)]
pub fn open_io<'a>() -> HashMap<&'a str, Box<RustClip<'a>>> {
    let mut func_map = HashMap::new();
    func_map.insert("print", Box::new(Print) as Box<RustClip>);
    func_map
}
