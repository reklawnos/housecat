use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use ast::Stmt;

#[derive(Debug, Clone)]
pub enum Value<'a> {
    Int(i64),
    Float(f64),
    Bool(bool),
    String(Box<String>),
    Tuple(Vec<Value<'a>>),
    Clip(Rc<RefCell<ClipStruct<'a>>>),
    Nil,
    Nothing //Type of function call with no returns
}

#[derive(Debug)]
pub struct ClipStruct<'a> {
    params: Vec<&'a str>,
    returns: Vec<&'a str>,
    statements: Vec<Stmt<'a>>,
    defs: HashMap<&'a str, Value<'a>>
}
