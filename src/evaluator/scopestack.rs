use std::collections::HashMap;

use evaluator::{Value, VarType};
use ast::*;

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
                           value: Value<'a>,
                           create_new: bool) -> Result<Value<'a>, String> {
    match idents {
        [s] => {
            unsafe {
                if (*curr_defs).contains_key(s) && !create_new {
                    (*curr_defs).insert(s, VarType::Def(value));
                    Ok(Value::Nil)
                } else if create_new {
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
                                let ret = get_def_from_idents(rest, def, value, create_new);
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

    fn assign_access(&mut self, expr: &'a Expr, value: Value<'a>, create_new: bool) -> Result<Value<'a>, String> {
        let idents = get_evald!(eval_expr_as_ident(expr));
        unsafe{
            for scope in self.scopes.iter_mut().rev() {
                if idents.len() == 1 {
                    match (**scope).get(idents[0]) {
                        Some(&VarType::Var(_)) if !create_new => {
                            (**scope).insert(idents[0], VarType::Var(value));
                            return Ok(Value::Nil);
                        }
                        Some(&VarType::Def(_)) if !create_new => {
                            (**scope).insert(idents[0], VarType::Def(value));
                            return Ok(Value::Nil);
                        }
                        Some(&VarType::Var(_)) if create_new => {
                            return Err(format!("EVAL FAILURE: cannot def to a name that is already a var"));
                        }
                        Some(_) => (),
                        None => {
                            if create_new {
                                (**scope).insert(idents[0], VarType::Def(value));
                                return Ok(Value::Nil);
                            }
                        }
                    }
                } else {
                    match (**scope).get(idents[0]) {
                        Some(v) => {
                            match v.as_ref() {
                                &Value::Clip(ref c) => {
                                    let def = &mut c.borrow_mut().defs;
                                    let retval = get_def_from_idents(&idents[1..], def, value, create_new);
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
    }

    pub fn assign<'b>(&mut self, stmt_item: &'a StmtItem, value: Value<'a>) -> Result<Value<'a>, String> {
        let curr_scope = self.scopes.len() - 1;
        match stmt_item {
            &StmtItem::Bare(ref e) => {
                return self.assign_access(e, value, false);
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
            &StmtItem::Def(ref e) => {
                return self.assign_access(e, value, true);
            } 
        }
        Ok(Value::Nil)
    }
}
