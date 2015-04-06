use evaluator::Evaluator;
use evaluator::Value;
use std::collections::HashMap;

#[allow(unused_variables, dead_code)]
fn print<'a>(args: &Vec<Value<'a>>, eval: &mut Evaluator<'a>) -> Result<Value<'a>, String> {
    if args.len() == 1 {
        println!("{}", args[0]);
        Result::Ok(Value::Nil)
    } else {
        Result::Err("Wrong number of args for `print`".to_string())
    }
}

#[allow(dead_code)]
pub fn open_io<'a>(eval: &mut Evaluator<'a>) {
    eval.add_rust_clip("print", Box::new(print), HashMap::new());
}
