use ast::*;
use values::*;
use std::collections::HashMap;
use std::num::{Int, Float};

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

fn int_pow(lhs: i64, rhs: i64) -> i64 {
    if rhs >= 0 {
        lhs.pow(rhs as u32)
    } else {
        0
    }

}

fn eval_literal<'a>(literal: &'a Literal) -> Result<Value<'a>> {
    match literal {
        &Literal::Bool(b) => Result::Ok(Value::Bool(b)),
        &Literal::Int(i) => Result::Ok(Value::Int(i)),
        &Literal::Float(f) => Result::Ok(Value::Float(f)),
        &Literal::String(s) => Result::Ok(Value::String(Box::new(s.to_string()))),
        &Literal::Nil => Result::Ok(Value::Nil),
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

pub fn eval_bin_op<'a>(lhs: &Value, rhs: &Value, op: &BinOp) -> Result<Value<'a>> {
    let result_val = match lhs {
        &Value::Int(lh_i) => {
            match *rhs {
                Value::Int(rh_i) => {
                    match op {
                        &BinOp::Add => Value::Int(lh_i + rh_i),
                        &BinOp::Sub => Value::Int(lh_i - rh_i),
                        &BinOp::Mul => Value::Int(lh_i * rh_i),
                        &BinOp::Div => Value::Int(lh_i / rh_i),
                        &BinOp::Mod => Value::Int(lh_i % rh_i),
                        &BinOp::Exp => Value::Int(int_pow(lh_i, rh_i)),
                        &BinOp::Lt => Value::Bool(lh_i < rh_i),
                        &BinOp::Lte => Value::Bool(lh_i <= rh_i),
                        &BinOp::Gt => Value::Bool(lh_i > rh_i),
                        &BinOp::Gte => Value::Bool(lh_i >= rh_i),
                        &BinOp::Eq => Value::Bool(lh_i == rh_i),
                        &BinOp::Neq => Value::Bool(lh_i != rh_i),
                        &BinOp::Same => Value::Bool(lh_i == rh_i),
                        &BinOp::Nsame => Value::Bool(lh_i != rh_i),
                        op => return Result::Err(format!("EVAL FAILURE: {:?} is not valid for integers", op))
                    }
                },
                _ => return Result::Err(format!("EVAL FAILURE: RHS is not an integer"))
            }
        },
        &Value::Float(lh_f) => {
            match *rhs {
                Value::Float(rh_f) => {
                    match op {
                        &BinOp::Add => Value::Float(lh_f + rh_f),
                        &BinOp::Sub => Value::Float(lh_f - rh_f),
                        &BinOp::Mul => Value::Float(lh_f * rh_f),
                        &BinOp::Div => Value::Float(lh_f / rh_f),
                        &BinOp::Mod => Value::Float(lh_f % rh_f),
                        &BinOp::Exp => Value::Float(lh_f.powf(rh_f)),
                        &BinOp::Lt => Value::Bool(lh_f < rh_f),
                        &BinOp::Lte => Value::Bool(lh_f <= rh_f),
                        &BinOp::Gt => Value::Bool(lh_f > rh_f),
                        &BinOp::Gte => Value::Bool(lh_f >= rh_f),
                        &BinOp::Eq => Value::Bool(lh_f == rh_f),
                        &BinOp::Neq => Value::Bool(lh_f != rh_f),
                        &BinOp::Same => Value::Bool(lh_f == rh_f),
                        &BinOp::Nsame => Value::Bool(lh_f != rh_f),
                        op => return Result::Err(format!("EVAL FAILURE: {:?} is not valid for floats", op))
                    }
                },
                _ => return Result::Err(format!("EVAL FAILURE: RHS is not a float"))
            }
        },
        &Value::Bool(lh_b) => {
            match *rhs {
                Value::Bool(rh_b) => {
                    match op {
                        &BinOp::Eq => Value::Bool(lh_b == rh_b),
                        &BinOp::Neq => Value::Bool(lh_b != rh_b),
                        &BinOp::Same => Value::Bool(lh_b == rh_b),
                        &BinOp::Nsame => Value::Bool(lh_b != rh_b),
                        &BinOp::And => Value::Bool(lh_b && rh_b),
                        &BinOp::Or => Value::Bool(lh_b || rh_b),
                        op => return Result::Err(format!("EVAL FAILURE: {:?} is not valid for bools", op))
                    }
                },
                _ => return Result::Err(format!("EVAL FAILURE: RHS is not a bool"))
            }
        },
        &Value::String(ref lh_s) => {
            match *rhs {
                Value::String(ref rh_s) => {
                    match op {
                        &BinOp::Add => Value::String(Box::new(*lh_s.clone() + rh_s.as_slice())),
                        &BinOp::Eq => Value::Bool(lh_s == rh_s),
                        &BinOp::Neq => Value::Bool(lh_s != rh_s),
                        &BinOp::Same => Value::Bool(lh_s == rh_s),
                        &BinOp::Nsame => Value::Bool(lh_s != rh_s),
                        op => return Result::Err(format!("EVAL FAILURE: {:?} is not valid for strings", op))
                    }
                },
                _ => return Result::Err(format!("EVAL FAILURE: RHS is not a string"))
            }
        },
        _ => panic!("bin op lhs not implemented yet")
    };
    Result::Ok(result_val)
}

pub fn eval_expr<'a>(expr: &'a Expr, scopes: &Vec<HashMap<&'a str, Value<'a>>>) -> Result<Value<'a>> {
    match expr {
        &Expr::Literal{ref value, ..} => Result::Ok(get_evald!(eval_literal(value))),
        &Expr::Ident{ref name, ref data} => {
            let val = get_value_from_scopes(scopes, &(**name));
            match val {
                Some(v) => Result::Ok(v),
                None => Result::Err(format!("EVAL FAILURE at line {}: {} is not in the current scope", data.line + 1, name))
            }
        },
        &Expr::Tuple{ref values, ..} => {
            let mut result_vec = Vec::new();
            for e in values.iter(){
                result_vec.push(get_evald!(eval_expr(e, scopes)));
            }
            Result::Ok(Value::Tuple(result_vec))
        }
        &Expr::BinOp{ref op, ref lhs, ref rhs, ref data} => {
            let lh_val = get_evald!(eval_expr(lhs, scopes));
            let rh_val = get_evald!(eval_expr(rhs, scopes));
            match (&lh_val, &rh_val) {
                (&Value::Tuple(ref lh_vec), &Value::Tuple(ref rh_vec)) => {
                    if lh_vec.len() != rh_vec.len() {
                        return Result::Err(format!("EVAL FAILURE at line {}: tuples are not the same length", data.line + 1));
                    }
                    let mut result_vec = Vec::new();
                    for (ref lh, ref rh) in lh_vec.iter().zip(rh_vec.iter()) {
                        result_vec.push(get_evald!(eval_bin_op(lh, rh, op)));
                    }
                    Result::Ok(Value::Tuple(result_vec))
                }
                (&Value::Tuple(_), _) | (_, &Value::Tuple(_)) => Result::Err(format!("EVAL FAILURE at line {}: both sides must be tuples", data.line + 1)),
                (ref lhs, ref rhs) => {
                    eval_bin_op(lhs, rhs, op)
                }
            }
        }
        _ => panic!("expr not implemented yet!")
    }
}

fn eval_expr_as_ident<'a>(expr: &'a Expr) -> Result<&'a str> {
    match expr {
        &Expr::Ident{name, ..} => Result::Ok(name),
        _ => panic!("expr as ident not implemented yet!")
    }
}

fn eval_bare_stmt_item<'a>(stmt_item: &'a StmtItem, scopes: &Vec<HashMap<&'a str, Value<'a>>>) -> Result<Value<'a>> {
    match stmt_item {
        &StmtItem::Bare(ref expr) => eval_expr(expr, scopes),
        _ => Result::Err(format!("EVAL FAILURE: need a bare expression"))
    }
}

fn assign<'a>(stmt_item: &'a StmtItem, value: Value<'a>, scopes: &mut Vec<HashMap<&'a str, Value<'a>>>, curr_scope: usize) -> Result<()> {
    match stmt_item {
        &StmtItem::Bare(ref e) => {
            let ident = get_evald!(eval_expr_as_ident(e));
            for scope in scopes.iter_mut().rev() {
                if scope.contains_key(ident) {
                    scope.insert(ident, value);
                    return Result::Ok(());
                }
            }
            return Result::Err(format!("EVAL FAILURE: '{}' is not declared in the current scope", ident));
        },
        &StmtItem::Var(ref s) => {
            if scopes[curr_scope].contains_key(s) {
                return Result::Err(format!("EVAL FAILURE: '{}' is already declared in the current scope", s));
            } else {
                scopes[curr_scope].insert(s, value);
            }
        }
        //TODO: make defs work with clips
        &StmtItem::Def(ref e) => {
            let ident = get_evald!(eval_expr_as_ident(e));
            if scopes[curr_scope].contains_key(ident) {
                return Result::Err(format!("EVAL FAILURE: '{}' is already defined in the current scope", ident));
            } else {
                scopes[curr_scope].insert(ident, value);
            }
        } 
    }
    Result::Ok(())
}

fn eval_stmt<'a>(stmt: &'a Stmt, scopes: &mut Vec<HashMap<&'a str, Value<'a>>>) -> Result<Vec<Value<'a>>> {
    match stmt {
        &Stmt::Bare{ref items, ..} => {
            let mut result_vec = vec![];
            for i in items.iter() {
                result_vec.push(get_evald!(eval_bare_stmt_item(i, scopes)));
            }
            Result::Ok(result_vec)
        }
        &Stmt::Assignment{ref items, ref expr, ref data} => {
            let expr_value = get_evald!(eval_expr(expr, scopes));
            let curr_scope = scopes.len() - 1;
            match expr_value {
                Value::Tuple(value_vec) => {
                    if items.len() == value_vec.len() {
                        for (i, e) in items.iter().zip(value_vec.into_iter()) {
                            get_evald!(assign(i, e, scopes, curr_scope));
                        }
                    } else if items.len() == 1 {
                        get_evald!(assign(&items[0], Value::Tuple(value_vec), scopes, curr_scope));
                    } else {
                        return Result::Err(format!("EVAL FAILURE at line {}: wrong arity for this assignment", data.line));
                    }
                } 
                _ => {
                    if items.len() == 1 {
                        get_evald!(assign(&items[0], expr_value, scopes, curr_scope));
                    } else {
                        return Result::Err(format!("EVAL FAILURE at line {}: too many idents to assign to", data.line));
                    }
                }
            }
            Result::Ok(vec![])
        }
        &Stmt::While{ref condition, ref statements, ref data} => {
            scopes.push(HashMap::new());
            let mut result_vec = vec![];
            loop {
                let expr_val = get_evald!(eval_expr(condition, scopes));
                match expr_val {
                    Value::Bool(b) => {
                        if !b {
                            break;
                        }
                    }
                    _ => {
                        return Result::Err(format!("EVAL FAILURE at line {}: while loops must contain a boolean expression", data.line));
                    }
                }
                let vals = get_evald!(eval_stmt_list(statements, scopes));
                for val in vals.into_iter() {
                    result_vec.push(val);
                }
            }
            Result::Ok(result_vec)
        }
        &Stmt::If{ref clauses, ref data} => {
            scopes.push(HashMap::new());
            let mut result_vec = vec![];
            for clause in clauses.iter() {
                match clause {
                    &IfClause::If{ref condition, ref statements} => {
                        let expr_val = get_evald!(eval_expr(condition, scopes));
                        match expr_val {
                            Value::Bool(b) => {
                                if !b {
                                    continue; 
                                }
                            }
                            _ => {
                                return Result::Err(format!("EVAL FAILURE at line {}: if statements must contain a boolean expression", data.line));
                            }
                        }
                        let vals = get_evald!(eval_stmt_list(statements, scopes));
                        for val in vals.into_iter() {
                            result_vec.push(val);
                        }
                        return Result::Ok(result_vec);
                    }
                    &IfClause::Else(ref statements) => {
                        let vals = get_evald!(eval_stmt_list(statements, scopes));
                        for val in vals.into_iter() {
                            result_vec.push(val);
                        }
                        return Result::Ok(result_vec);
                    }
                }
            }
            Result::Ok(result_vec)
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
    //let mut defs = HashMap::new();
    eval_stmt_list(stmt_list, &mut scopes)
}
