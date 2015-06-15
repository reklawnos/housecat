mod ops;
mod codegen;
mod vm;
pub mod values;

use ast::Stmt;

use self::codegen::gen_stmt_list;
use self::ops::Op;
use self::vm::execute;

use std::collections::HashMap;
use std::mem::size_of;
use std::rc::Rc;
use std::cell::RefCell;

use libhc::open_libs;
use self::values::{Value, RustHolder};


fn print_ops(ops: &Vec<Op>) {
    for (idx, op) in ops.iter().enumerate() {
        println!("{}: {:?}", idx, op);
    }
}

pub fn evaluate<'a>(ast: &'a Vec<Stmt<'a>>, defs: &mut HashMap<Value<'a>, Value<'a>>) -> Result<(), String> {
    println!("running stack eval...");
    println!("op size: {}", size_of::<Op>());
    println!("value size: {}", size_of::<Value>());
    println!("boxed size: {}", size_of::<Box<Value>>());
    println!("string size: {}", size_of::<String>());
    let libs = open_libs();
    let mut ops = Vec::with_capacity(1024);
    let mut var_map = HashMap::with_capacity(1024);
    let mut id = 0usize;
    for (key, rc) in libs.into_iter() {
        var_map.insert(key, Value::RustClip(RustHolder{clip: Rc::new(RefCell::new(rc)), id: id}));
        id += 1;
    }
    let mut vars = vec![var_map];
    
    match gen_stmt_list(&ast, &mut ops) {
        Ok(_) => (),
        Err(s) => {return Err(s)}
    };
    print_ops(&ops);
    let mut stack = Vec::with_capacity(2048);
    match execute(&mut ops, &mut stack, &mut vars, defs) {
        Ok(()) => Ok(()),
        Err(s) => Err(s)
    }
}
