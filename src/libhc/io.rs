use values::*;
use evaluator::Evaluator;
use eval_result::Result;
use std::collections::HashMap;

fn print<'a>(args: &Vec<Value<'a>>) -> Result<Value<'a>> {
    if args.len() == 1 {
        println!("{:?}", args[0]);
        Result::Ok(Value::Nil)
    } else {
        Result::Err("Wrong number of args for `print`".to_string())
    }
}

pub fn open_io<'a>(eval: &mut Evaluator<'a>) {
    eval.add_rust_clip("print", Box::new(print), HashMap::new());
}
