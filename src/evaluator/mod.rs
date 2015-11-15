mod ops;
mod codegen;
mod vm;
mod standard_clip;
pub mod values;
pub mod clip;
pub mod environment;

use ast::Stmt;

use self::codegen::gen_stmt_list;
use self::ops::Op;
use self::vm::execute;
use self::environment::Environment;
use self::clip::ClipHolder;
use self::values::Value;

use std::collections::HashMap;
use std::mem::size_of;

use libhc::open_libs;


fn print_ops(ops: &Vec<Op>) {
    for (idx, op) in ops.iter().enumerate() {
        println!("{}: {:?}", idx, op);
    }
}

pub fn evaluate<'a>(ast: &'a Vec<Stmt<'a>>, defs: &mut HashMap<Value, Value>) -> Result<(), String> {
    println!("running stack eval...");
    println!("op size: {}", size_of::<Op>());
    println!("value size: {}", size_of::<Value>());
    println!("boxed size: {}", size_of::<Box<Value>>());
    println!("string size: {}", size_of::<String>());
    let libs = open_libs();
    let mut ops = Vec::with_capacity(1024);
    let mut env = Environment::new();
    env.push_frame();
    for (key, rc) in libs.into_iter() {
        env.declare_var(key.to_string(), Value::Clip(ClipHolder::new(rc)));
    }

    match gen_stmt_list(&ast, &mut ops) {
        Ok(_) => (),
        Err(s) => {return Err(s)}
    };
    print_ops(&ops);
    let mut stack = Vec::with_capacity(256);
    match execute(&mut ops, &mut stack, &mut env, defs) {
        Ok(()) => Ok(()),
        Err(s) => Err(s)
    }
}
