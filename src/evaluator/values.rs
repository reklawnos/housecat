use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use ast::*;
use std::fmt::{Debug, Display, Formatter};
use std::fmt::Result as FmtResult;

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

pub struct ScopeStack<'a>{
    scopes: Vec<*mut HashMap<&'a str, VarType<'a>>>
}


fn eval_expr_as_ident<'a>(expr: &'a Expr) -> Result<Vec<&'a str>, String> {
    match expr {
        &Expr::Ident{name, ..} => Ok(vec![name]),
        //TODO: implement idents for defining interior values
        &Expr::Postfix{ref expr, ref postfixes, ref data} => {
            let mut result_vec = Vec::new();
            match **expr {
                Expr::Ident{name, ..} => {
                    result_vec.push(name);
                }
                _ => {return Err(format!("EVAL FAILURE at line {}: cannot assign to a non-ident", data.line + 1))}
            }
            for postfix in postfixes.iter() {
                match postfix {
                    &Postfix::Access(s) => {
                        result_vec.push(s);
                    }
                    //TODO: need to do this for index types, too
                    _ => {return Err(format!("EVAL FAILURE at line {}: cannot assign to a non-ident", data.line + 1))}
                }
            }
            Ok(result_vec)
        }
        _ => Err(format!("EVAL FAILURE: cannot assign to a non-ident"))
    }
}

fn get_def_from_idents<'a>(idents: &[&'a str],
                           curr_defs: *mut HashMap<&'a str, VarType<'a>>,
                           value: Value<'a>) -> Result<Value<'a>, String> {
    match idents {
        [s] => {
            unsafe {
                if (*curr_defs).contains_key(s) {
                    (*curr_defs).insert(s, VarType::Def(value));
                    Ok(Value::Nil)
                } else {
                    Err(format!("EVAL FAILURE: '{}' is not declared in the current scope", s))
                }
            }
        }
        [s, rest..] => {
            unsafe {
                match (*curr_defs).get(s) {
                    Some(var) => {
                        match var.as_ref() {
                            &Value::Clip(ref c) => {
                                let clone = c.clone();
                                let def = &mut clone.borrow_mut().defs;
                                let ret = get_def_from_idents(rest, def, value);
                                ret
                            }
                            _ => {
                                Err(format!("EVAL FAILURE: cannot assign values to a non-clip"))
                            }
                        }
                    }
                    None => {
                        Err(format!("EVAL FAILURE: ident not found"))
                    }
                }
            }
        }
        [] => Err(format!("EVAL FAILURE: missing an ident here"))
    }
}

impl<'a> ScopeStack<'a> {
    pub fn new() -> ScopeStack<'a> {
       ScopeStack{scopes: vec![]}
    }

    pub fn push(&mut self, new_scope: &mut HashMap<&'a str, VarType<'a>>) {
        self.scopes.push(new_scope);
    }

    pub fn pop(&mut self) {
        self.scopes.pop();
    }

    pub fn get(&mut self, name: &'a str) -> Option<Value<'a>>{
        let iter = self.scopes.iter_mut();
        for scope in iter.rev() {
            unsafe {
                match (**scope).get(name) {
                    Some(&VarType::Var(ref v)) | Some(&VarType::Def(ref v)) => {return Some(v.clone())},
                    None => {}
                }
            }
        }
        None
    }

    pub fn assign<'b>(&mut self, stmt_item: &'a StmtItem, value: Value<'a>) -> Result<Value<'a>, String> {
        let curr_scope = self.scopes.len() - 1;
        match stmt_item {
            &StmtItem::Bare(ref e) => {
                let idents = get_evald!(eval_expr_as_ident(e));
                unsafe{
                    for scope in self.scopes.iter_mut().rev() {
                        if idents.len() == 1 {
                            match (**scope).get(idents[0]) {
                                Some(&VarType::Var(_)) => {
                                    (**scope).insert(idents[0], VarType::Var(value));
                                    return Ok(Value::Nil);
                                }
                                Some(&VarType::Def(_)) => {
                                    (**scope).insert(idents[0], VarType::Def(value));
                                    return Ok(Value::Nil);
                                }
                                None => ()
                            }
                        } else {
                            match (**scope).get(idents[0]) {
                                Some(v) => {
                                    match v.as_ref() {
                                        &Value::Clip(ref c) => {
                                            let def = &mut c.borrow_mut().defs;
                                            let retval = get_def_from_idents(&idents[1..], def, value);
                                            return retval;
                                        }
                                        _ => {
                                            return Err(format!("EVAL FAILURE: cannot assign values to a non-clip"));
                                        }
                                    }
                                }
                                None => {
                                    continue;
                                }
                            }
                        }
                    }
                }
                return Err(format!("EVAL FAILURE: '{}' was not found in any scope", idents[0]));
            },
            &StmtItem::Var(ref s) => {
                unsafe {
                    let ref mut curr_scope_val = *self.scopes[curr_scope];
                    if curr_scope_val.contains_key(s) {
                        return Err(format!("EVAL FAILURE: '{}' is already declared in the current scope", s));
                    } else {
                        curr_scope_val.insert(s, VarType::Var(value));
                    }
                }
            }
            //TODO: Allow inserting defs into a clip
            &StmtItem::Def(ref e) => {
                let idents = get_evald!(eval_expr_as_ident(e));
                //Only define if it's not yet defined
                unsafe {
                    let ref mut curr_scope_val = *self.scopes[curr_scope];
                    if !curr_scope_val.contains_key(idents[0]) {
                        curr_scope_val.insert(idents[0], VarType::Def(value));
                    }
                }
            } 
        }
        Ok(Value::Nil)
    }
}

use evaluator::Evaluator;

pub struct RustClip<'a> {
    func: Box<Fn(&Vec<Value<'a>>, &mut Evaluator<'a>) -> Result<Value<'a>, String>>,
    pub defs: HashMap<&'a str, VarType<'a>>
}

impl<'a> RustClip<'a> {
    pub fn new(func: Box<Fn(&Vec<Value<'a>>, &mut Evaluator<'a>) -> Result<Value<'a>, String>>,
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
