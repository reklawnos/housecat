use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter, Error};
use std::fmt::Result as FmtResult;
use num::Float;
use std::mem;
use std::hash::{Hash, Hasher};
use std::cmp::Eq;

use super::ops::Op;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Value {
    Int(i64),
    Float(FloatWrap),
    Bool(bool),
    String(String),
    Tuple(Vec<Value>),
    Clip(ClipHolder),
    RustClip(RustHolder),
    Nil
}

impl Display for Value {
    fn fmt<'r>(&'r self, formatter: &mut Formatter) -> FmtResult {
        match self {
            &Value::Int(i) => write!(formatter, "{}", i),
            &Value::Float(ref f) => write!(formatter, "{}", f),
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


#[derive(Clone, Debug)]
pub struct RustHolder {
    pub clip: Rc<RefCell<Box<RustClip>>>,
    pub id: usize
}

impl Eq for RustHolder {}

impl PartialEq for RustHolder {
    fn eq(&self, other: &RustHolder) -> bool {
        self.id == other.id
    }

    fn ne(&self, other: &RustHolder) -> bool {
        self.id != other.id
    }
}

impl Hash for RustHolder {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

pub trait RustClip: Debug{
    fn get(&self, &str) -> Option<Value>;
    fn set(&mut self, &str, Value) -> Result<(), String>;
    fn call(&mut self, Vec<Value>) -> Result<Value, String>;
}


#[derive(PartialEq, Clone, Debug)]
pub struct ClipHolder(pub Rc<RefCell<ClipStruct>>);

impl Eq for ClipHolder {}

impl Hash for ClipHolder {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.borrow().hash(state);
    }
}


pub struct ClipStruct {
    pub params: Vec<String>,
    pub returns: Vec<String>,
    pub statements: Vec<Op>,
    pub defs: HashMap<Value, Value>
}

impl PartialEq for ClipStruct {
    fn eq(&self, other: &ClipStruct) -> bool {
        let self_ptr: *const ClipStruct = self;
        let other_ptr: *const ClipStruct = other;
        self_ptr == other_ptr
    }

    fn ne(&self, other: &ClipStruct) -> bool {
        !self.eq(other)
    }
}

impl Eq for ClipStruct {}

impl Hash for ClipStruct {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let self_ptr: *const ClipStruct = self;
        self_ptr.hash(state);
    }
}

impl Debug for ClipStruct {
    fn fmt(&self, formatter: &mut Formatter) -> FmtResult {
        formatter.write_str("<Clip>")
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct FloatWrap(u64);

impl FloatWrap {
    pub fn new(mut val: f64) -> FloatWrap {
        // make all NaNs have the same representation
        if val.is_nan() {
            val = Float::nan()
        }
        unsafe {
            FloatWrap(mem::transmute(val))
        }
    }

    pub fn get(&self) -> f64 {
        let cl = self.clone();
        unsafe {
            mem::transmute(cl)
        }
    }
}

impl Debug for FloatWrap {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{:?}", self.get())
    }
}

impl Display for FloatWrap {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", self.get())
    }
}
