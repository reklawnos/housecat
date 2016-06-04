use std::fmt::{Debug, Display, Formatter, Error};
use std::fmt::Result as FmtResult;
use num::Float;
use std::mem;

use super::clip::{ClipHolder};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Value {
    Int(i64),
    Float(FloatWrap),
    Bool(bool),
    String(String),
    Tuple(Vec<Value>),
    Clip(ClipHolder),
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
            &Value::Nil => write!(formatter, "nil"),
        }
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
