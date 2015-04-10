use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use std::fmt::Result as FmtResult;
use super::ops::Op;
use super::RustClipFuncStack;

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

impl<'a> PartialEq for Value<'a> {
    fn eq(&self, other: &Value<'a>) -> bool {
        match (self, other) {
            (&Value::Int(l), &Value::Int(r)) => l == r,
            (&Value::Float(l), &Value::Float(r)) => l == r,
            (&Value::Bool(l), &Value::Bool(r)) => l == r,
            (&Value::String(ref l), &Value::String(ref r)) => l == r,
            (&Value::Tuple(ref l), &Value::Tuple(ref r)) => l == r,
            (&Value::Clip(ref l), &Value::Clip(ref r)) => {
                unsafe {
                    let l_ptr = l.as_unsafe_cell().get();
                    let r_ptr = r.as_unsafe_cell().get();
                    l_ptr == r_ptr
                }
            }
            (&Value::RustClip(ref l), &Value::RustClip(ref r)) => {
                unsafe {
                    let l_ptr = l.as_unsafe_cell().get();
                    let r_ptr = r.as_unsafe_cell().get();
                    l_ptr == r_ptr
                }
            },
            (&Value::Nil, &Value::Nil) => true,
            (_, _) => false
        }
    }

    fn ne(&self, other: &Value<'a>) -> bool {
        !self.eq(other)
    }
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
    pub params: Vec<&'a str>,
    pub returns: Vec<&'a str>,
    pub statements: Vec<Op<'a>>,
    pub defs: HashMap<&'a str, VarType<'a>>
}

pub struct RustClip<'a> {
    func: Box<RustClipFuncStack<'a>>,
    pub defs: HashMap<&'a str, VarType<'a>>
}

impl<'a> Debug for RustClip<'a> {
    fn fmt(&self, formatter: &mut Formatter) -> FmtResult {
        formatter.write_str("<RustClip>")
    }
}

#[derive(Debug, Clone, PartialEq)]
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
