use ast::*;
use values::*;
use std::collections::HashMap;

macro_rules! get_execd(
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

fn eval_literal(literal: &Literal) -> Result<Value> {
    match literal {
        &Literal::Bool(b)       => Result::Ok(Value::Bool(b)),
        &Literal::Int(i)        => Result::Ok(Value::Int(i)),
        &Literal::Float(f)      => Result::Ok(Value::Float(f)),
        &Literal::String(ref s) => Result::Ok(Value::String(s.clone())),
        &Literal::Nil           => Result::Ok(Value::Nil),
        _ => panic!("literal not implemented yet!")
    }
}


fn get_value_from_scopes(scopes: &Vec<HashMap<String, Value>>, name: &String) -> Option<Value>{
    for scope in scopes.iter().rev() {
        match scope.get(name) {
            Some(v) => {return Some((*v).clone())},
            None => {}
        }
    }
    None
}

pub fn eval_expr(expr: &Expr, scope: &HashMap<String, Value>) -> Result<Value> {
    match expr {
        &Expr::Literal(ref l) => eval_literal(l),
        &Expr::Ident(ref s) => {
            let val = get_value_from_scopes(&vec![*scope], &(**s));
            match val {
                Some(v) => Result::Ok(v),
                None => Result::Err(format!("EXEC FAILURE: {} is not in the current scope", s))
            }
        }
        
        _ => panic!("expr not implemented yet!")
    }
}

fn eval_stmt_item(stmt_item: &StmtItem, scope: &HashMap<String, Value>) -> Result<Value> {
    match stmt_item {
        &StmtItem::Bare(ref expr) => eval_expr(expr, scope),
        _ => panic!("stmt_item not implemented yet!")
    }
}

fn eval_stmt(stmt: &Stmt, scope: &HashMap<String, Value>) -> Result<Vec<Value>> {
    match stmt {
        &Stmt::Bare(ref items) => {
            let mut result_vec = vec![];
            for i in items.iter() {
                result_vec.push(get_execd!(eval_stmt_item(i, scope)));
            }
            Result::Ok(result_vec)
        }
        // &Stmt::Assignment(ref items, ref expr) => {
        //     let mut result_vec = vec![];
        //     for i in items.iter() {
        //         result_vec.push(get_execd!(eval_stmt_item(i, scope)));
        //     }
        //     let expr_value = eval_expr(expr, scope);

        // }
        _ => panic!("stmt not implemented yet!")
    }
}

fn eval_stmt_list(stmt_list: &Vec<Stmt>, scope: &HashMap<String, Value>) -> Result<Vec<Value>> {
    let mut ret_list = vec![];
    for st in stmt_list.iter() {
        let mut values = get_execd!(eval_stmt(st, scope));
        ret_list.append(&mut values);
    }
    Result::Ok(ret_list)
}


pub fn eval_file_stmts(stmt_list: &Vec<Stmt>) -> Result<Vec<Value>> {
    let mut scope_hash = HashMap::new();
    eval_stmt_list(stmt_list, &scope_hash)
}
