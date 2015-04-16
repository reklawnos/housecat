use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter, Error};
use std::fmt::Result as FmtResult;
use num::Float;
use std::mem;
use core::hash::{Hash, Hasher};
use core::cmp::Eq;

use super::ops::Op;
//use super::RustClipFuncStack;
//use evaluator::Evaluator;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Value<'a> {
    Int(i64),
    Float(FloatWrap),
    Bool(bool),
    String(String),
    Tuple(Vec<Value<'a>>),
    Clip(ClipHolder<'a>),
    RustClip(RustHolder<'a>),
    Nil
}

impl<'a> Display for Value<'a> {
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


#[derive(PartialEq, Clone, Debug)]
pub struct ClipHolder<'a>(pub Rc<RefCell<ClipStruct<'a>>>);

impl<'a> Eq for ClipHolder<'a> {}

impl<'a> Hash for ClipHolder<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.borrow().hash(state);
    }
}


#[derive(PartialEq, Clone, Debug)]
pub struct RustHolder<'a>(pub Rc<RefCell<RustClip<'a>>>);

impl<'a> Eq for RustHolder<'a> {}

impl<'a> Hash for RustHolder<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.borrow().hash(state);
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


pub struct ClipStruct<'a> {
    pub params: Vec<&'a str>,
    pub returns: Vec<&'a str>,
    pub statements: Vec<Op<'a>>,
    pub defs: HashMap<Value<'a>, Value<'a>>
}

impl<'a> PartialEq for ClipStruct<'a> {
    fn eq(&self, other: &ClipStruct<'a>) -> bool {
        let self_ptr: *const ClipStruct<'a> = self;
        let other_ptr: *const ClipStruct<'a> = other;
        self_ptr == other_ptr
    }

    fn ne(&self, other: &ClipStruct<'a>) -> bool {
        !self.eq(other)
    }
}

impl<'a> Eq for ClipStruct<'a> {}

impl<'a> Hash for ClipStruct<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let self_ptr: *const ClipStruct<'a> = self;
        self_ptr.hash(state);
    }
}

impl<'a> Debug for ClipStruct<'a> {
    fn fmt(&self, formatter: &mut Formatter) -> FmtResult {
        formatter.write_str("<Clip>")
    }
}


pub struct RustClip<'a> {
    //func: Box<RustClipFuncStack<'a>>,
    pub defs: HashMap<Value<'a>, Value<'a>>
}

impl<'a> PartialEq for RustClip<'a> {
    fn eq(&self, other: &RustClip<'a>) -> bool {
        let self_ptr: *const RustClip<'a> = self;
        let other_ptr: *const RustClip<'a> = other;
        self_ptr == other_ptr
    }

    fn ne(&self, other: &RustClip<'a>) -> bool {
        !self.eq(other)
    }
}

impl<'a> Eq for RustClip<'a> {}

impl<'a> Hash for RustClip<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let self_ptr: *const RustClip<'a> = self;
        self_ptr.hash(state);
    }
}


// impl<'a> RustClip<'a> {
//     pub fn new(func: Box<RustClipFuncStack<'a>>,
//                defs: HashMap<Value<'a>, Value<'a>>) -> RustClip<'a> {
//         RustClip{func: func, defs: defs}
//     }
//     pub fn call(&self, args: &Vec<Value<'a>>, eval: &mut Evaluator<'a>) -> Result<Value<'a>, String> {
//         (*self.func)(args, eval)
//     }
// }

impl<'a> Debug for RustClip<'a> {
    fn fmt(&self, formatter: &mut Formatter) -> FmtResult {
        formatter.write_str("<RustClip>")
    }
}
