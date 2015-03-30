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
    pub defs: HashMap<&'a str, VarType<'a>>
}

#[derive(Debug)]
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

    pub fn as_mut<'r>(&'r mut self) -> &'r mut Value<'a> {
        match *self {
            VarType::Var(ref mut v) => v,
            VarType::Def(ref mut v) => v
        }
    }

    pub fn unwrap(self) -> Value<'a> {
        match self {
            VarType::Var(v) => v,
            VarType::Def(v) => v
        }
    }
}


pub enum ScopeType<'a> {
    Clip(Rc<RefCell<ClipStruct<'a>>>),
    Block(HashMap<&'a str, VarType<'a>>)
}

// impl<'a> ScopeType<'a> {
//     pub fn as_mut<'r>(&'r mut self) -> &'r mut HashMap<&'a str, VarType<'a>> {
//         match *self {
//             ScopeType::Clip(c) => {
//                 &mut c.borrow_mut().defs
//             }
//             ScopeType::Block(ref mut s) => s
//         }
//     }
// }

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
