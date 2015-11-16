use evaluator::value::{Value, RustClip, ClipStruct, ClipHolder};
use std::rc::Rc;
use std::cell::RefCell;
use interpreter::Interpreter;

#[derive(Debug)]
pub struct Import;

#[allow(unused_variables, dead_code)]
impl<'a> RustClip for Import {
    fn get(&self, key: &str) -> Option<Value> {
        None
    }
    fn set(&mut self, key: &str, value: Value) -> Result<(), String> {
        Err("Cannot set a def on import".to_string())
    }
    fn call(&mut self, args: Vec<Value>) -> Result<Value, String> {
        if args.len() == 1 {
            println!("importing file: {:?}", args[0]);
            let mut interpreter = Interpreter::new();
            match args[0] {
                Value::String(ref s) => {
                    let defs = try!(interpreter.interpret_file(&s[..]));
                    let c = defs.clone();
                    let new_clip = ClipStruct {
                        params: Vec::new(),
                        returns: Vec::new(),
                        statements: Vec::new(),
                        defs: c
                    };
                    Ok(Value::Clip(ClipHolder(Rc::new(RefCell::new(new_clip)))))
                }
                _ => Result::Err("Can only use a string parameter for `import`".to_string())
            }
        } else {
            Result::Err("Wrong number of args for `import`".to_string())
        }
    }
}
