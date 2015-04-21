mod ops;
mod codegen;
mod vm;
pub mod values;

use super::*;
use lexer::Lexer;
use parser;

use self::codegen::gen_stmt_list;
use self::ops::Op;
use self::vm::execute;

use std::collections::HashMap;
use std::mem::size_of;
use std::rc::Rc;
use std::cell::RefCell;

use libhc::open_libs;
use self::values::RustHolder;


fn print_ops(ops: &Vec<Op>) {
    for (idx, op) in ops.iter().enumerate() {
        println!("{}: {:?}", idx, op);
    }
}

pub fn test_stack(file_string: String) {
    println!("testing stack eval...");
    println!("op size: {}", size_of::<Op>());
    println!("value size: {}", size_of::<Value>());
    println!("boxed size: {}", size_of::<Box<Value>>());
    println!("string size: {}", size_of::<String>());
    let mut lexer = Lexer::new();
    let result = lexer.lex(file_string);
    let mut statements = Vec::new();
    let ast = match result {
        Err(_) => {
            panic!("failed to lex");
        }
        Ok(toks) => {
            let parse_result = parser::parse_tokens(&toks[..], &mut statements);
            match parse_result {
                Ok(v) => v,
                Err(s) => panic!("failed to parse: {}", s)
            }
        }
    };
    let libs = open_libs();
    let mut ops = Vec::with_capacity(1024);
    let mut defs = HashMap::with_capacity(100);
    let mut var_map = HashMap::with_capacity(1024);
    let mut id = 0usize;
    for (key, rc) in libs.into_iter() {
        var_map.insert(key, Value::RustClip(RustHolder{clip: Rc::new(RefCell::new(rc)), id: id}));
        id += 1;
    }
    let mut vars = vec![var_map];
    
    gen_stmt_list(&ast, &mut ops);
    //println!("ops are: {:?}", ops);
    print_ops(&ops);
    let mut stack = Vec::with_capacity(2048);
    execute(&mut ops, &mut stack, &mut vars, &mut defs);
}
