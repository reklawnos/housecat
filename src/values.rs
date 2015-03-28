use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use ast::Stmt;
use eval_result::Result;

#[derive(Debug, Clone)]
pub enum Value<'a> {
    Int(i64),
    Float(f64),
    Bool(bool),
    String(String),
    Tuple(Vec<Value<'a>>),
    Clip(Rc<RefCell<ClipStruct<'a>>>),
    Builtin(Builtin),
    Nil
}

#[derive(Debug)]
pub struct ClipStruct<'a> {
    pub params: &'a Vec<&'a str>,
    pub returns: &'a Vec<&'a str>,
    pub statements: &'a Vec<Stmt<'a>>,
    pub defs: HashMap<&'a str, Value<'a>>
}

#[derive(Debug, Clone)]
pub enum Builtin {
    Print
}

impl Builtin {
    pub fn call<'a>(&self, args: &Vec<Value<'a>>) -> Result<Value<'a>>{
        match self {
            &Builtin::Print => {
                if args.len() == 1 {
                    println!("{:?}", args[0]);
                    Result::Ok(Value::Nil)
                } else {
                    Result::Err("EVAL FAILURE: wrong number of args for `print`".to_string())
                }
            }
        }
    }
}
