use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use std::fmt::Result as FmtResult;

use ast::*;
use evaluator::RustClipFunc;

macro_rules! get_evald(
    ($parsed:expr) => ({
        match $parsed {
            Ok(t) => t,
            Err(e) => {return Err(e);}
        }
    });
);

#[derive(Debug, Clone)]
pub enum Value<'a> {
    Int(i64),
    Float(f64),
    Bool(bool),
    String(String),
    Tuple(Vec<Value<'a>>),
    Clip(Rc<RefCell<ClipStruct<'a>>>),
    RustClip(Rc<RefCell<RustClip<'a>>>),
    Nil
}

impl<'a> Display for Value<'a> {
    fn fmt<'r>(&'r self, formatter: &mut Formatter) -> FmtResult {
        match self {
            &Value::Int(i) => write!(formatter, "{}", i),
            &Value::Float(f) => write!(formatter, "{}", f),
            &Value::Bool(b) => write!(formatter, "{}", b),
            &Value::String(ref s) => write!(formatter, "{}", s),
            &Value::Tuple(ref v) => {
                match write!(formatter, "(") {
                    Ok(()) => (),
                    Err(e) => {return Err(e);}
                }
                let len = v.len();
                for (idx, val) in v.iter().enumerate() {
                    match write!(formatter, "{}", val) {
                        Ok(()) => (),
                        Err(e) => {return Err(e);}
                    }
                    if idx != len - 1 {
                        match write!(formatter, ", ") {
                            Ok(()) => (),
                            Err(e) => {return Err(e);}
                        }
                    }
                }
                match write!(formatter, ")") {
                    Ok(()) => Ok(()),
                    Err(e) => Err(e)
                }
            }
            &Value::Clip(_) => write!(formatter, "<Clip>"),
            &Value::RustClip(_) => write!(formatter, "<RustClip>"),
            &Value::Nil => write!(formatter, "nil"),
        }
        
    }
}

#[derive(Debug)]
pub struct ClipStruct<'a> {
    pub params: &'a Vec<&'a str>,
    pub returns: &'a Vec<&'a str>,
    pub statements: &'a Vec<Stmt<'a>>,
    pub defs: HashMap<&'a str, VarType<'a>>
}

#[derive(Debug, Clone)]
pub enum VarType<'a> {
    Var(Value<'a>),
    Def(Value<'a>)
}

impl<'a> VarType<'a> {
    pub fn as_ref<'r>(&'r self) -> &'r Value<'a> {
        match *self {
            VarType::Var(ref v) => v,
            VarType::Def(ref v) => v 
        }
    }

    pub fn unwrap(self) -> Value<'a> {
        match self {
            VarType::Var(v) => v,
            VarType::Def(v) => v
        }
    }
}

use evaluator::Evaluator;

pub struct RustClip<'a> {
    func: Box<RustClipFunc<'a>>,
    pub defs: HashMap<&'a str, VarType<'a>>
}

impl<'a> RustClip<'a> {
    pub fn new(func: Box<RustClipFunc<'a>>,
               defs: HashMap<&'a str, VarType<'a>>) -> RustClip<'a> {
        RustClip{func: func, defs: defs}
    }
    pub fn call(&self, args: &Vec<Value<'a>>, eval: &mut Evaluator<'a>) -> Result<Value<'a>, String> {
        (*self.func)(args, eval)
    }
}

impl<'a> Debug for RustClip<'a> {
    fn fmt(&self, formatter: &mut Formatter) -> FmtResult {
        formatter.write_str("<RustClip>")
    }
}
