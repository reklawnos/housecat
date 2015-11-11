use evaluator::values::{Value, RustClip};

#[derive(Debug)]
pub struct Print;

#[allow(unused_variables, dead_code)]
impl<'a> RustClip for Print {
    fn get(&self, key: &str) -> Option<Value> {
        None
    }
    fn set(&mut self, key: &str, value: Value) -> Result<(), String> {
        Err("Cannot set a def on print".to_string())
    }
    fn call(&mut self, args: Vec<Value>) -> Result<Value, String> {
        if args.len() == 1 {
            println!("{}", args[0]);
            Ok(Value::Nil)
        } else {
            Err("Wrong number of args for `print`".to_string())
        }
    }
}
