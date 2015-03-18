use ast::*;
use values::*;
use std::collections::HashMap;

macro_rules! get_evald(
    ($parsed:expr) => ({
        match $parsed {
            Result::Ok(t) => t,
            Result::Err(e) => {return Result::Err(e);}
        }
    });
);

pub enum Result<T> {
    Ok(T),
    Err(String)
}

fn eval_literal<'a>(literal: &'a Literal) -> Result<Value<'a>> {
    match literal {
        &Literal::Bool(b)       => Result::Ok(Value::Bool(b)),
        &Literal::Int(i)        => Result::Ok(Value::Int(i)),
        &Literal::Float(f)      => Result::Ok(Value::Float(f)),
        &Literal::String(ref s) => Result::Ok(Value::String(&s[..])),
        &Literal::Nil           => Result::Ok(Value::Nil),
        _ => panic!("literal not implemented yet!")
    }
}


fn get_value_from_scopes<'a>(scopes: &Vec<HashMap<&'a str, Value<'a>>>, name: &str) -> Option<Value<'a>>{
    for scope in scopes.iter().rev() {
        match scope.get(name) {
            Some(v) => {return Some((*v).clone())},
            None => {}
        }
    }
    None
}

pub fn eval_expr<'a>(expr: &'a Expr, scopes: &Vec<HashMap<&'a str, Value<'a>>>) -> Result<Vec<Value<'a>>> {
    match expr {
        &Expr::Literal(ref l) => Result::Ok(vec![get_evald!(eval_literal(l))]),
        &Expr::Ident(ref s) => {
            let val = get_value_from_scopes(scopes, &(**s));
            match val {
                Some(v) => Result::Ok(vec![v]),
                None => Result::Err(format!("EVAL FAILURE: {} is not in the current scope", s))
            }
        }
        
        _ => panic!("expr not implemented yet!")
    }
}

fn eval_expr_as_ident<'a>(expr: &'a Expr, scopes: &Vec<HashMap<&'a str, Value<'a>>>) -> Result<&'a str> {
    match expr {
        &Expr::Ident(ref s) => Result::Ok(s),
        _ => panic!("expr as ident not implemented yet!")
    }
}

fn eval_stmt_item<'a>(stmt_item: &'a StmtItem, scopes: &Vec<HashMap<&'a str, Value<'a>>>) -> Result<Vec<Value<'a>>> {
    match stmt_item {
        &StmtItem::Bare(ref expr) => eval_expr(expr, scopes),
        _ => panic!("stmt_item not implemented yet!")
    }
}

fn eval_stmt<'a>(stmt: &'a Stmt, scopes: &mut Vec<HashMap<&'a str, Value<'a>>>) -> Result<Vec<Value<'a>>> {
    match stmt {
        &Stmt::Bare(ref items) => {
            let mut result_vec = vec![];
            for i in items.iter() {
                for val in get_evald!(eval_stmt_item(i, scopes)).into_iter() {
                    result_vec.push(val);
                }
            }
            Result::Ok(result_vec)
        }
        &Stmt::Assignment(ref items, ref expr) => {
            let expr_values = get_evald!(eval_expr(expr, scopes));
            let curr_scope = scopes.len() - 1;
            for (i, expr_value) in items.iter().zip(expr_values.into_iter()) {
                match i {
                    &StmtItem::Bare(ref e) => {
                        let ident = get_evald!(eval_expr_as_ident(e, scopes));
                        if scopes[curr_scope].contains_key(ident) {
                            scopes[curr_scope].insert(ident, expr_value);
                        } else {
                            return Result::Err(format!("EVAL FAILURE: '{}' is not declared in the current scope", ident));
                        }
                    },
                    &StmtItem::Var(ref s) => {
                        if scopes[curr_scope].contains_key(s) {
                            return Result::Err(format!("EVAL FAILURE: '{}' is already declared in the current scope", s));
                        } else {
                            scopes[curr_scope].insert(s, expr_value);
                        }
                    }
                    &StmtItem::Def(ref e) => {
                        let ident = get_evald!(eval_expr_as_ident(e, scopes));
                        if scopes[curr_scope].contains_key(ident) {
                            return Result::Err(format!("EVAL FAILURE: '{}' is already defined in the current scope", ident));
                        } else {
                            scopes[curr_scope].insert(ident, expr_value);
                        }
                    } 
                }
            }
            Result::Ok(vec![])
        }
        _ => panic!("stmt not implemented yet!")
    }
}

fn eval_stmt_list<'a>(stmt_list: &'a Vec<Stmt>, scopes: &mut Vec<HashMap<&'a str, Value<'a>>>) -> Result<Vec<Value<'a>>> {
    let mut ret_list = vec![];
    for st in stmt_list.iter() {
        let mut values = get_evald!(eval_stmt(st, scopes));
        ret_list.append(&mut values);
    }
    Result::Ok(ret_list)
}


pub fn eval_file_stmts<'a>(stmt_list: &'a Vec<Stmt>) -> Result<Vec<Value<'a>>> {
    let mut scopes = vec![HashMap::new()];
    eval_stmt_list(stmt_list, &mut scopes)
}
