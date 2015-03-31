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

fn init_builtins<'a>() -> HashMap<&'a str, VarType<'a>> {
    let mut map = HashMap::new();
    map.insert("print", VarType::Var(Value::Builtin(Builtin::Print)));
    map
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
        &Literal::String(s) => Result::Ok(Value::String(s.to_string())),
        &Literal::Nil => Result::Ok(Value::Nil),
        &Literal::Clip{ref params, ref returns, ref statements} => {
            let new_defs = HashMap::new();
            let new_clip = ClipStruct {
                params: params,
                returns: returns,
                statements: statements,
                defs: new_defs
            };
            Result::Ok(Value::Clip(Rc::new(RefCell::new(new_clip))))
        }
    }
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
                        &BinOp::Add => Value::String(lh_s.clone() + &rh_s[..]),
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

pub fn eval_expr<'a>(expr: &'a Expr, scopes: &mut ScopeStack<'a>) -> Result<Value<'a>> {
    match expr {
        &Expr::Literal{ref value, ..} => Result::Ok(get_evald!(eval_literal(value))),
        &Expr::Ident{ref name, ref data} => {
            let val = scopes.get(&(**name));
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
        &Expr::UnOp{ref op, ref expr, ref data} => {
            let val = get_evald!(eval_expr(expr, scopes));
            match op {
                &UnOp::Neg => {
                    match &val {
                        &Value::Int(i) => Result::Ok(Value::Int(-i)),
                        &Value::Float(f) => Result::Ok(Value::Float(-f)),
                        _ => Result::Err(format!("EVAL FAILURE at line {}: cannot negate a non-number type", data.line + 1))
                    }
                },
                &UnOp::Not => {
                    match &val {
                        &Value::Bool(b) => Result::Ok(Value::Bool(!b)),
                        _ => Result::Err(format!("EVAL FAILURE at line {}: cannot negate a non-boolean type", data.line + 1))
                    }
                }
                &UnOp::Get => {
                    match val {
                        Value::Clip(c) => {
                            {
                                let mut borrowed_clip = c.borrow_mut();
                                //Add nil as params to the incoming scope
                                for key in borrowed_clip.params.iter() {
                                    borrowed_clip.defs.insert(*key, VarType::Var(Value::Nil));
                                }
                                for key in borrowed_clip.returns.iter() {
                                    borrowed_clip.defs.insert(*key, VarType::Var(Value::Nil));
                                }
                                scopes.push(&mut borrowed_clip.defs);
                                get_evald!(eval_stmt_list(borrowed_clip.statements, scopes));
                                scopes.pop();
                            }
                            Result::Ok(Value::Clip(c))
                        }
                        _ => Result::Err(format!("EVAL FAILURE at line {}: get operator only valid on clip values", data.line + 1))
                    }
                }
            }
        }
        &Expr::Postfix{ref expr, ref postfixes, ref data} => {
            let mut curr_val = get_evald!(eval_expr(expr, scopes));
            let mut scopes_to_pop = 0;
            for postfix in postfixes.iter() {
                match postfix {
                    &Postfix::Play(ref args) => {
                        let mut arg_values = Vec::new();
                        for arg in args.iter() {
                            arg_values.push(get_evald!(eval_expr(arg, scopes)));
                        }
                        match curr_val {
                            Value::Clip(c) => {
                                let mut borrowed_clip = c.borrow_mut();
                                if borrowed_clip.params.len() != arg_values.len() {
                                    return Result::Err(format!("EVAL FAILURE at line {}: clip expects a different number of params", data.line + 1));
                                }
                                //Add params to the incoming scope
                                for (key, value) in borrowed_clip.params.iter().zip(arg_values.into_iter()) {
                                    borrowed_clip.defs.insert(*key, VarType::Var(value));
                                }
                                for key in borrowed_clip.returns.iter() {
                                    borrowed_clip.defs.insert(*key, VarType::Var(Value::Nil));
                                }
                                scopes.push(&mut borrowed_clip.defs);
                                get_evald!(eval_stmt_list(borrowed_clip.statements, scopes));
                                scopes.pop();
                                match borrowed_clip.returns.len() {
                                    0 => {
                                        curr_val = Value::Nil;
                                    }
                                    1 => {
                                        let return_key = borrowed_clip.returns[0];
                                        curr_val = borrowed_clip.defs.remove(return_key).unwrap().unwrap();
                                    }
                                    _ => {
                                        let mut result_vec = Vec::new();
                                        for ret in borrowed_clip.returns.iter() {
                                            result_vec.push(borrowed_clip.defs.remove(ret).unwrap().unwrap());
                                        }
                                        curr_val = Value::Tuple(result_vec);
                                    }
                                }
                                for _ in 0..scopes_to_pop {
                                    scopes.pop()
                                }
                                scopes_to_pop = 0;
                            }
                            Value::Builtin(b) => {
                                curr_val = get_evald!(b.call(&arg_values));
                            }
                            _ => {return Result::Err(format!("EVAL FAILURE at line {}: can only play clips and builtins", data.line + 1));}
                        }
                    }
                    &Postfix::Access(s) => {
                        match curr_val {
                            Value::Clip(c) => {
                                let mut borrowed_clip = c.borrow_mut();
                                let new_val = match borrowed_clip.defs.get(s) {
                                    Some(v) => (*v.as_ref()).clone(),
                                    None => {
                                        return Result::Err(format!("EVAL FAILURE at line {}: clip has no field '{}'", data.line + 1, s));
                                    }
                                };
                                scopes.push(&mut borrowed_clip.defs);
                                scopes_to_pop += 1;
                                curr_val = new_val;
                            }
                            //TODO: maybe add fields to other types?
                            _ => {
                                return Result::Err(format!("EVAL FAILURE at line {}: cannot access non-clip field", data.line + 1));
                            }
                        }
                    }
                    //TODO: Implement indexing
                    &Postfix::Index(_) => panic!("postfix type not implemented yet")
                }
            }
            Result::Ok(curr_val)
        }
    }
}

fn eval_bare_stmt_item<'a>(stmt_item: &'a StmtItem, scopes: &mut ScopeStack<'a>) -> Result<Value<'a>> {
    match stmt_item {
        &StmtItem::Bare(ref expr) => eval_expr(expr, scopes),
        _ => Result::Err(format!("EVAL FAILURE: need a bare expression"))
    }
}

fn eval_stmt_list<'a>(stmt_list: &'a Vec<Stmt>, scopes: &mut ScopeStack<'a>) -> Result<Vec<Value<'a>>> {
    let mut ret_list = vec![];
    for st in stmt_list.iter() {
        let mut values = get_evald!(match st {
            &Stmt::Bare{ref items, ..} => {
                let mut result_vec = vec![];
                for i in items.iter() {
                    result_vec.push(get_evald!(eval_bare_stmt_item(i, scopes)));
                }
                Result::Ok(result_vec)
            }
            &Stmt::Assignment{ref items, ref expr, ref data} => {
                let expr_value = get_evald!(eval_expr(expr, scopes));
                match expr_value {
                    Value::Tuple(value_vec) => {
                        if items.len() == value_vec.len() {
                            for (i, e) in items.iter().zip(value_vec.into_iter()) {
                                scopes.assign(i, e);
                            }
                        } else if items.len() == 1 {
                            get_evald!(scopes.assign(&items[0], Value::Tuple(value_vec)));
                        } else {
                            return Result::Err(format!("EVAL FAILURE at line {}: wrong arity for this assignment", data.line + 1));
                        }
                    } 
                    _ => {
                        if items.len() == 1 {
                            get_evald!(scopes.assign(&items[0], expr_value));
                        } else {
                            return Result::Err(format!("EVAL FAILURE at line {}: too many idents to assign to", data.line + 1));
                        }
                    }
                }
                Result::Ok(vec![])
            }
            &Stmt::While{ref condition, ref statements, ref data} => {
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
                            return Result::Err(format!("EVAL FAILURE at line {}: while loops must contain a boolean expression", data.line + 1));
                        }
                    }
                    let mut new_scope = HashMap::new();
                    scopes.push(&mut new_scope);
                    let vals = get_evald!(eval_stmt_list(statements, scopes));
                    for val in vals.into_iter() {
                        result_vec.push(val);
                    }
                    scopes.pop();
                }
                Result::Ok(result_vec)
            }
            &Stmt::If{ref clauses, ref data} => {
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
                                    return Result::Err(format!("EVAL FAILURE at line {}: if statements must contain a boolean expression", data.line + 1));
                                }
                            }
                            let mut new_scope = HashMap::new();
                            scopes.push(&mut new_scope);
                            let vals = get_evald!(eval_stmt_list(statements, scopes));
                            for val in vals.into_iter() {
                                result_vec.push(val);
                            }
                            scopes.pop();
                            break;
                        }
                        &IfClause::Else(ref statements) => {
                            let mut new_scope = HashMap::new();
                            scopes.push(&mut new_scope);
                            let vals = get_evald!(eval_stmt_list(statements, scopes));
                            for val in vals.into_iter() {
                                result_vec.push(val);
                            }
                            scopes.pop();
                            break;
                        }
                    }
                }
                Result::Ok(result_vec)
            }
            &Stmt::Return{..} => {
                break;
            }
        });
        ret_list.append(&mut values);
    }
    Result::Ok(ret_list)
}


pub fn eval_file_stmts<'a>(stmt_list: &'a Vec<Stmt<'a>>, params: &'a Vec<&'a str>, returns: &'a Vec<&'a str>) -> Result<Rc<RefCell<ClipStruct<'a>>>> {
    let file_defs = HashMap::new();
    let file_clip = ClipStruct {
        params: params,
        returns: returns,
        statements: stmt_list,
        defs: file_defs
    };
    let file_pointer = Rc::new(RefCell::new(file_clip));
    {
        let mut borrowed_clip = file_pointer.borrow_mut();
        let mut builtins = init_builtins();
        let mut scopes = ScopeStack::new();
        scopes.push(&mut builtins);
        scopes.push(&mut borrowed_clip.defs);
        get_evald!(eval_stmt_list(borrowed_clip.statements, &mut scopes));
    }
    Result::Ok(file_pointer)
}
