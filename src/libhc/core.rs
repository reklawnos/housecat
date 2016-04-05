use evaluator::value::Value;
use evaluator::clip::{Clip, ClipHolder};
use evaluator::standard_clip::StdClip;
use evaluator::environment::Environment;
use interpreter::Interpreter;

#[derive(Debug)]
pub struct Import;

#[allow(unused_variables, dead_code)]
impl<'a> Clip for Import {
    fn get(&self, key: &Value) -> Value {
        Value::Nil
    }

    fn set(&mut self, key: Value, value: Value) -> Result<(), String> {
        Err("Cannot set a def on import built-in".to_string())
    }

    fn play(&mut self, args: Vec<Value>, environment: &mut Environment)
         -> Result<Value, String> {
        if args.len() == 1 {
            let mut interpreter = Interpreter::new();
            match args[0] {
                Value::String(ref s) => {
                    let defs = try!(interpreter.interpret_file(&s[..]));
                    let c = defs.clone();
                    let new_clip = StdClip::new_with_defs(
                        Vec::new(),
                        Vec::new(),
                        Vec::new(),
                        c
                    );
                    Ok(Value::Clip(ClipHolder::new(Box::new(new_clip))))
                }
                _ => Result::Err("Can only use a string parameter for `import`".to_string())
            }
        } else {
            Result::Err("Wrong number of args for `import`".to_string())
        }
    }
}
