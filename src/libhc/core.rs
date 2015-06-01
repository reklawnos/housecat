use evaluator::stack_evaluator::values::{Value, RustClip};

#[derive(Debug)]
pub struct Import;

#[allow(unused_variables, dead_code)]
impl<'a> RustClip<'a> for Import {
    fn get(&self, key: &str) -> Option<Value<'a>> {
        None
    }
    fn set(&mut self, key: &str, value: Value<'a>) -> Result<(), &str> {
        Err("Cannot set a def on import")
    }
    fn call(&mut self, args: Vec<Value<'a>>) -> Result<Value<'a>, &str> {
        if args.len() == 1 {
            println!("importing file: {:?}", args[0]);

            Result::Ok(Value::Nil)
        } else {
            Result::Err("Wrong number of args for `import`")
        }
    }
}
