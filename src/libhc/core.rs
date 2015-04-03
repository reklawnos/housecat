use evaluator::Evaluator;
use evaluator::values::*;
use eval_result::Result;
use std::collections::HashMap;

fn import<'a>(args: &Vec<Value<'a>>, eval: &mut Evaluator<'a>) -> Result<Value<'a>> {
    if args.len() == 1 {
        println!("importing file: {:?}", args[0]);
        Result::Ok(Value::Nil)
    } else {
        Result::Err("Wrong number of args for `import`".to_string())
    }
}

pub fn open_core<'a>(eval: &mut Evaluator<'a>) {
    eval.add_rust_clip("import", Box::new(import), HashMap::new());
}
