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

pub use self::values::Value;

pub type RustClipFuncStack<'a> = Fn(&Vec<Value<'a>>, &mut Evaluator<'a>) -> Result<Value<'a>, String>;


fn print_ops(ops: &Vec<Op>) {
    for (idx, op) in ops.iter().enumerate() {
        println!("{}: {:?}", idx, op);
    }
}

pub fn test_stack(){
    println!("testing stack eval...");
    let mut lexer = Lexer::new();
    let file_string =
    "\
    var x: 1\
    while x < 1000000
        \"go\"
        x
        x: x + 1
    end
    if false
        1
        2
        3
    end".to_string();
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
    let mut ops = Vec::new();
    gen_stmt_list(&ast, &mut ops);
    //println!("ops are: {:?}", ops);
    print_ops(&ops);
    execute(&ops);
}
