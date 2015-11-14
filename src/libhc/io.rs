use evaluator::values::Value;
use evaluator::clip::Clip;
use evaluator::environment::Environment;

#[derive(Debug)]
pub struct Print;

#[allow(unused_variables, dead_code)]
impl Clip for Print {
    fn get(&self, key: &Value) -> Value {
        Value::Nil
    }

    fn set(&mut self, key: Value, value: Value) -> Result<(), String> {
        Err("Cannot set a def on print built-in".to_string())
    }

    fn play(&mut self, args: Vec<Value>, environment: &mut Environment)
         -> Result<Value, String> {
        if args.len() == 1 {
            println!("{}", args[0]);
            Ok(Value::Nil)
        } else {
            Err("Wrong number of args for `print`".to_string())
        }
    }
}
