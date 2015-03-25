use ast::*;
use values::*;
use std::collections::HashMap;
use std::num::{Int, Float};
use std::rc::Rc;
use std::cell::RefCell;
use eval_result::Result;

macro_rules! get_evald(
    ($parsed:expr) => ({
        match $parsed {
            Result::Ok(t) => t,
            Result::Err(e) => {return Result::Err(e);}
        }
    });
);

fn init_builtins<'a>() -> HashMap<&'a str, Value<'a>> {
    let mut map = HashMap::new();
    map.insert("print", Value::Builtin(Builtin::Print));
    map
}

fn int_pow(lhs: i64, rhs: i64) -> i64 {
    if rhs >= 0 {
        lhs.pow(rhs as u32)
    } else {
        0
    }

}

fn eval_literal<'a>(literal: &'a Literal, scopes: &mut Vec<HashMap<&'a str, Value<'a>>>, defs: &HashMap<&'a str, Value<'a>>) -> Result<Value<'a>> {
    match literal {
        &Literal::Bool(b) => Result::Ok(Value::Bool(b)),
        &Literal::Int(i) => Result::Ok(Value::Int(i)),
        &Literal::Float(f) => Result::Ok(Value::Float(f)),
        &Literal::String(s) => Result::Ok(Value::String(Box::new(s.to_string()))),
        &Literal::Nil => Result::Ok(Value::Nil),
        &Literal::Clip{ref params, ref returns, ref statements} => {
            let mut new_defs = HashMap::new();
            //TODO: fix scoping here: the defs from the containing scope need to be available
            //scopes.push(defs);
            eval_stmt_list(statements, scopes, &mut new_defs, false);
            //scopes.pop();
            Result::Ok(
                Value::Clip(
                    Rc::new(
                        RefCell::new(
                            ClipStruct{
                                params: params.clone(),
                                returns: returns.clone(),
                                statements: statements,
                                defs: new_defs
                            }
                        )
                    )
                )
            )
        }
    }
}


fn get_value_from_scopes<'a>(scopes: &Vec<HashMap<&'a str, Value<'a>>>, defs: &HashMap<&'a str, Value<'a>>, name: &str) -> Option<Value<'a>>{
    match defs.get(name) {
        Some(v) => {return Some((*v).clone())},
        None => {}
    }
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

pub fn eval_expr<'a>(expr: &'a Expr, scopes: &mut Vec<HashMap<&'a str, Value<'a>>>, defs: &mut HashMap<&'a str, Value<'a>>) -> Result<Value<'a>> {
    match expr {
        &Expr::Literal{ref value, ..} => Result::Ok(get_evald!(eval_literal(value, scopes, defs))),
        &Expr::Ident{ref name, ref data} => {
            let val = get_value_from_scopes(scopes, defs, &(**name));
            match val {
                Some(v) => Result::Ok(v),
                None => Result::Err(format!("EVAL FAILURE at line {}: {} is not in the current scope", data.line + 1, name))
            }
        },
        &Expr::Tuple{ref values, ..} => {
            let mut result_vec = Vec::new();
            for e in values.iter(){
                result_vec.push(get_evald!(eval_expr(e, scopes, defs)));
            }
            Result::Ok(Value::Tuple(result_vec))
        }
        &Expr::BinOp{ref op, ref lhs, ref rhs, ref data} => {
            let lh_val = get_evald!(eval_expr(lhs, scopes, defs));
            let rh_val = get_evald!(eval_expr(rhs, scopes, defs));
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
        &Expr::UnOp{ref op, ref expr, ref data} => {
            let val = get_evald!(eval_expr(expr, scopes, defs));
            match op {
                &UnOp::Neg => {
                    match &val {
                        &Value::Int(i) => Result::Ok(Value::Int(-i)),
                        &Value::Float(f) => Result::Ok(Value::Float(-f)),
                        _ => Result::Err(format!("EVAL FAILURE at line {}: cannot negate a non-number type", data.line))
                    }
                },
                &UnOp::Not => {
                    match &val {
                        &Value::Bool(b) => Result::Ok(Value::Bool(!b)),
                        _ => Result::Err(format!("EVAL FAILURE at line {}: cannot negate a non-boolean type", data.line))
                    }
                }
            }
        }
        &Expr::Postfix{ref expr, ref postfixes, ..} => {
            let mut curr_val = get_evald!(eval_expr(expr, scopes, defs));
            for postfix in postfixes.iter() {
                match postfix {
                    &Postfix::Play(ref args) => {
                        let mut arg_values = Vec::new();
                        for arg in args.iter() {
                            arg_values.push(get_evald!(eval_expr(arg, scopes, defs)));
                        }
                        match curr_val {
                            Value::Clip(ref c) => {
                                //TODO: do return variables need to be defined, or just var'd?
                                //TODO: set up return values and parameters
                                let mut borrowed_clip = c.borrow_mut();
                                scopes.push(HashMap::new());
                                eval_stmt_list(borrowed_clip.statements, scopes, &mut borrowed_clip.defs, true);
                                scopes.pop();
                            }
                            Value::Builtin(b) => {
                                curr_val = get_evald!(b.call(&arg_values));
                            }
                            _ => {return Result::Err(format!("EVAL FAILURE: can only play clips and builtins"));}
                        }
                    }
                    &Postfix::Access(s) => {
                        match curr_val {
                            Value::Clip(c) => {
                                let borrowed_clip = c.borrow();
                                let new_val = match borrowed_clip.defs.get(s) {
                                    Some(v) => v.clone(),
                                    None => {return Result::Err(format!("EVAL FAILURE: clip has no field '{}'", s));}
                                };
                                curr_val = new_val;
                            }
                            _ => panic!("cannot access a non-clip at this point")
                        }
                    }
                    _ => panic!("postfix type not implemented yet")
                }
            }
            Result::Ok(curr_val)
        }
        //_ => panic!("expr not implemented yet!")
    }
}

fn eval_expr_as_ident<'a>(expr: &'a Expr) -> Result<&'a str> {
    match expr {
        &Expr::Ident{name, ..} => Result::Ok(name),
        _ => panic!("expr as ident not implemented yet! {:?}", expr)
    }
}

fn eval_bare_stmt_item<'a>(stmt_item: &'a StmtItem, scopes: &mut Vec<HashMap<&'a str, Value<'a>>>, defs: &mut HashMap<&'a str, Value<'a>>) -> Result<Value<'a>> {
    match stmt_item {
        &StmtItem::Bare(ref expr) => eval_expr(expr, scopes, defs),
        _ => Result::Err(format!("EVAL FAILURE: need a bare expression"))
    }
}

fn assign<'a>(stmt_item: &'a StmtItem, value: Value<'a>, scopes: &mut Vec<HashMap<&'a str, Value<'a>>>, curr_scope: usize, defs: &mut HashMap<&'a str, Value<'a>>, skip_defs: bool) -> Result<Value<'a>> {
    match stmt_item {
        &StmtItem::Bare(ref e) => {
            let ident = get_evald!(eval_expr_as_ident(e));
            if defs.contains_key(ident) {
                defs.insert(ident, value);
                return Result::Ok(Value::Nothing);
            }
            for scope in scopes.iter_mut().rev() {
                if scope.contains_key(ident) {
                    scope.insert(ident, value);
                    return Result::Ok(Value::Nothing);
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
            if !skip_defs {
                let ident = get_evald!(eval_expr_as_ident(e));
                if defs.contains_key(ident) {
                    return Result::Err(format!("EVAL FAILURE: '{}' is already defined in the current clip", ident));
                } else {
                    defs.insert(ident, value);
                }
            }
        } 
    }
    Result::Ok(Value::Nothing)
}

fn eval_stmt<'a>(stmt: &'a Stmt, scopes: &mut Vec<HashMap<&'a str, Value<'a>>>, defs: &mut HashMap<&'a str, Value<'a>>, skip_defs: bool) -> Result<Vec<Value<'a>>> {
    match stmt {
        &Stmt::Bare{ref items, ..} => {
            let mut result_vec = vec![];
            for i in items.iter() {
                result_vec.push(get_evald!(eval_bare_stmt_item(i, scopes, defs)));
            }
            Result::Ok(result_vec)
        }
        &Stmt::Assignment{ref items, ref expr, ref data} => {
            let expr_value = get_evald!(eval_expr(expr, scopes, defs));
            let curr_scope = scopes.len() - 1;
            match expr_value {
                Value::Tuple(value_vec) => {
                    if items.len() == value_vec.len() {
                        for (i, e) in items.iter().zip(value_vec.into_iter()) {
                            get_evald!(assign(i, e, scopes, curr_scope, defs, skip_defs));
                        }
                    } else if items.len() == 1 {
                        get_evald!(assign(&items[0], Value::Tuple(value_vec), scopes, curr_scope, defs, skip_defs));
                    } else {
                        return Result::Err(format!("EVAL FAILURE at line {}: wrong arity for this assignment", data.line));
                    }
                } 
                _ => {
                    if items.len() == 1 {
                        get_evald!(assign(&items[0], expr_value, scopes, curr_scope, defs, skip_defs));
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
                let expr_val = get_evald!(eval_expr(condition, scopes, defs));
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
                let vals = get_evald!(eval_stmt_list(statements, scopes, defs, skip_defs));
                for val in vals.into_iter() {
                    result_vec.push(val);
                }
            }
            Result::Ok(result_vec)
        }
        &Stmt::If{ref clauses, ref data} => {
            let mut result_vec = vec![];
            for clause in clauses.iter() {
                match clause {
                    &IfClause::If{ref condition, ref statements} => {
                        let expr_val = get_evald!(eval_expr(condition, scopes, defs));
                        match expr_val {
                            Value::Bool(b) => {
                                if !b {
                                    continue; 
                                }
                            }
                            _ => {
                                scopes.pop();
                                return Result::Err(format!("EVAL FAILURE at line {}: if statements must contain a boolean expression", data.line));
                            }
                        }
                        scopes.push(HashMap::new());
                        let vals = get_evald!(eval_stmt_list(statements, scopes, defs, skip_defs));
                        for val in vals.into_iter() {
                            result_vec.push(val);
                        }
                        scopes.pop();
                        return Result::Ok(result_vec);
                    }
                    &IfClause::Else(ref statements) => {
                        scopes.push(HashMap::new());
                        let vals = get_evald!(eval_stmt_list(statements, scopes, defs, skip_defs));
                        for val in vals.into_iter() {
                            result_vec.push(val);
                        }
                        scopes.pop();
                        return Result::Ok(result_vec);
                    }
                }
            }
            Result::Ok(result_vec)
        }
        _ => panic!("stmt not implemented yet!")
    }
}

fn eval_stmt_list<'a>(stmt_list: &'a Vec<Stmt>, scopes: &mut Vec<HashMap<&'a str, Value<'a>>>, defs: &mut HashMap<&'a str, Value<'a>>, skip_defs: bool) -> Result<Vec<Value<'a>>> {
    let mut ret_list = vec![];
    for st in stmt_list.iter() {
        let mut values = get_evald!(eval_stmt(st, scopes, defs, skip_defs));
        ret_list.append(&mut values);
    }
    Result::Ok(ret_list)
}


pub fn eval_file_stmts<'a>(stmt_list: &'a Vec<Stmt>) -> Result<Vec<Value<'a>>> {
    let mut scopes = vec![init_builtins(), HashMap::new()];
    let mut defs = HashMap::new();
    eval_stmt_list(stmt_list, &mut scopes, &mut defs, false)
}
