pub mod values;
mod scopestack;

use ast::*;
use std::collections::HashMap;
use std::num::{Int, Float};
use std::rc::Rc;
use std::cell::RefCell;
use super::*;

pub use self::values::{Value, VarType, RustClip, ClipStruct};
use self::scopestack::ScopeStack;

fn int_pow(lhs: i64, rhs: i64) -> i64 {
    if rhs >= 0 {
        lhs.pow(rhs as u32)
    } else {
        0
    }
}

pub struct AstEvaluator<'a> {
    rust_clips: HashMap<&'a str, VarType<'a>>
}

impl<'a> Evaluator<'a> for AstEvaluator<'a> {
    fn add_rust_clip(&mut self,
                         name: &'a str,
                         func: Box<RustClipFunc<'a>>,
                         defs: HashMap<&'a str, VarType<'a>>) {
        let new_clip = RustClip::new(func, defs);
        self.rust_clips.insert(name, VarType::Var(Value::RustClip(Rc::new(RefCell::new(new_clip)))));
    }

    fn eval_file_stmts(&mut self,
                           stmt_list: &'a Vec<Stmt<'a>>,
                           params: &'a Vec<&'a str>,
                           returns: &'a Vec<&'a str>) -> Result<Rc<RefCell<ClipStruct<'a>>>, String> {
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
            let mut scopes = ScopeStack::new();
            scopes.push(&mut self.rust_clips);
            scopes.push(&mut borrowed_clip.defs);
            try!(self.eval_stmt_list(borrowed_clip.statements, &mut scopes));
        }
        Ok(file_pointer)
    }
}

impl<'a> AstEvaluator<'a> {
    pub fn new() -> AstEvaluator<'a> {
        AstEvaluator{rust_clips: HashMap::new()}
    }

    fn eval_literal(literal: &'a Literal) -> Result<Value<'a>, String> {
        match literal {
            &Literal::Bool(b) => Ok(Value::Bool(b)),
            &Literal::Int(i) => Ok(Value::Int(i)),
            &Literal::Float(f) => Ok(Value::Float(f)),
            &Literal::String(s) => Ok(Value::String(s.to_string())),
            &Literal::Nil => Ok(Value::Nil),
            &Literal::Clip{ref params, ref returns, ref statements} => {
                let new_defs = HashMap::new();
                let new_clip = ClipStruct {
                    params: params,
                    returns: returns,
                    statements: statements,
                    defs: new_defs
                };
                Ok(Value::Clip(Rc::new(RefCell::new(new_clip))))
            }
        }
    }

    pub fn eval_bin_op(lhs: &Value, rhs: &Value, op: &BinOp) -> Result<Value<'a>, String> {
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
                            op => return Err(format!("EVAL FAILURE: {:?} is not valid for integers", op))
                        }
                    },
                    _ => return Err(format!("EVAL FAILURE: RHS is not an integer"))
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
                            op => return Err(format!("EVAL FAILURE: {:?} is not valid for floats", op))
                        }
                    },
                    _ => return Err(format!("EVAL FAILURE: RHS is not a float"))
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
                            op => return Err(format!("EVAL FAILURE: {:?} is not valid for bools", op))
                        }
                    },
                    _ => return Err(format!("EVAL FAILURE: RHS is not a bool"))
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
                            op => return Err(format!("EVAL FAILURE: {:?} is not valid for strings", op))
                        }
                    },
                    _ => return Err(format!("EVAL FAILURE: RHS is not a string"))
                }
            },
            _ => panic!("bin op lhs not implemented yet")
        };
        Ok(result_val)
    }

    pub fn eval_expr(&mut self, expr: &'a Expr, scopes: &mut ScopeStack<'a>) -> Result<Value<'a>, String> {
        match expr {
            &Expr::Literal{ref value, ..} => Ok(try!(AstEvaluator::eval_literal(value))),
            &Expr::Ident{ref name, ref data} => {
                let val = scopes.get(&(**name));
                match val {
                    Some(v) => Ok(v),
                    None => Err(format!("EVAL FAILURE at line {}: {} is not in the current scope", data.line + 1, name))
                }
            },
            &Expr::Tuple{ref values, ..} => {
                let mut result_vec = Vec::new();
                for e in values.iter(){
                    result_vec.push(try!(self.eval_expr(e, scopes)));
                }
                Ok(Value::Tuple(result_vec))
            }
            &Expr::BinOp{ref op, ref lhs, ref rhs, ref data} => {
                let lh_val = try!(self.eval_expr(lhs, scopes));
                let rh_val = try!(self.eval_expr(rhs, scopes));
                match (&lh_val, &rh_val) {
                    (&Value::Tuple(ref lh_vec), &Value::Tuple(ref rh_vec)) => {
                        if lh_vec.len() != rh_vec.len() {
                            return Err(format!("EVAL FAILURE at line {}: tuples are not the same length", data.line + 1));
                        }
                        let mut result_vec = Vec::new();
                        for (ref lh, ref rh) in lh_vec.iter().zip(rh_vec.iter()) {
                            result_vec.push(try!(AstEvaluator::eval_bin_op(lh, rh, op)));
                        }
                        Ok(Value::Tuple(result_vec))
                    }
                    (&Value::Tuple(_), _) | (_, &Value::Tuple(_)) => Err(format!("EVAL FAILURE at line {}: both sides must be tuples", data.line + 1)),
                    (ref lhs, ref rhs) => {
                        AstEvaluator::eval_bin_op(lhs, rhs, op)
                    }
                }
            }
            &Expr::UnOp{ref op, ref expr, ref data} => {
                let val = try!(self.eval_expr(expr, scopes));
                match op {
                    &UnOp::Neg => {
                        match &val {
                            &Value::Int(i) => Ok(Value::Int(-i)),
                            &Value::Float(f) => Ok(Value::Float(-f)),
                            _ => Err(format!("EVAL FAILURE at line {}: cannot negate a non-number type", data.line + 1))
                        }
                    },
                    &UnOp::Not => {
                        match &val {
                            &Value::Bool(b) => Ok(Value::Bool(!b)),
                            _ => Err(format!("EVAL FAILURE at line {}: cannot negate a non-boolean type", data.line + 1))
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
                                    try!(self.eval_stmt_list(borrowed_clip.statements, scopes));
                                    scopes.pop();
                                }
                                Ok(Value::Clip(c))
                            }
                            _ => Err(format!("EVAL FAILURE at line {}: get operator only valid on clip values", data.line + 1))
                        }
                    }
                }
            }
            &Expr::Postfix{ref expr, ref postfixes, ref data} => {
                let mut curr_val = try!(self.eval_expr(expr, scopes));
                let mut scopes_to_pop = 0;
                for postfix in postfixes.iter() {
                    match postfix {
                        &Postfix::Play(ref args) => {
                            let mut arg_values = Vec::new();
                            for arg in args.iter() {
                                arg_values.push(try!(self.eval_expr(arg, scopes)));
                            }
                            match curr_val {
                                Value::Clip(c) => {
                                    let mut borrowed_clip = c.borrow_mut();
                                    if borrowed_clip.params.len() != arg_values.len() {
                                        return Err(format!("EVAL FAILURE at line {}: clip expects a different number of params", data.line + 1));
                                    }
                                    //Add params to the incoming scope
                                    for (key, value) in borrowed_clip.params.iter().zip(arg_values.into_iter()) {
                                        borrowed_clip.defs.insert(*key, VarType::Var(value));
                                    }
                                    for key in borrowed_clip.returns.iter() {
                                        borrowed_clip.defs.insert(*key, VarType::Var(Value::Nil));
                                    }
                                    scopes.push(&mut borrowed_clip.defs);
                                    try!(self.eval_stmt_list(borrowed_clip.statements, scopes));
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
                                Value::RustClip(b) => {
                                    curr_val = try!(b.borrow().call(&arg_values, self));
                                }
                                _ => {return Err(format!("EVAL FAILURE at line {}: can only play clips and RustClips", data.line + 1));}
                            }
                        }
                        &Postfix::Access(s) => {
                            match curr_val {
                                Value::Clip(c) => {
                                    let mut borrowed_clip = c.borrow_mut();
                                    let new_val = match borrowed_clip.defs.get(s) {
                                        Some(v) => (*v.as_ref()).clone(),
                                        None => {
                                            return Err(format!("EVAL FAILURE at line {}: clip has no field '{}'", data.line + 1, s));
                                        }
                                    };
                                    scopes.push(&mut borrowed_clip.defs);
                                    scopes_to_pop += 1;
                                    curr_val = new_val;
                                }
                                Value::RustClip(c) => {
                                    let mut borrowed_clip = c.borrow_mut();
                                    let new_val = match borrowed_clip.defs.get(s) {
                                        Some(v) => (*v.as_ref()).clone(),
                                        None => {
                                            return Err(format!("EVAL FAILURE at line {}: rust clip has no field '{}'", data.line + 1, s));
                                        }
                                    };
                                    scopes.push(&mut borrowed_clip.defs);
                                    scopes_to_pop += 1;
                                    curr_val = new_val;
                                }
                                //TODO: maybe add fields to other types?
                                _ => {
                                    return Err(format!("EVAL FAILURE at line {}: cannot access non-clip field", data.line + 1));
                                }
                            }
                        }
                        //TODO: Implement indexing
                        &Postfix::Index(_) => panic!("postfix type not implemented yet"),
                        &Postfix::PlaySelf(_, _) => panic!("postfix play self not implemented")
                    }
                }
                Ok(curr_val)
            }
        }
    }

    fn eval_bare_stmt_item(&mut self, stmt_item: &'a StmtItem,
                               scopes: &mut ScopeStack<'a>) -> Result<Value<'a>, String> {
        match stmt_item {
            &StmtItem::Bare(ref expr) => self.eval_expr(expr, scopes),
            _ => Err(format!("EVAL FAILURE: need a bare expression"))
        }
    }

    fn eval_stmt_list(&mut self, stmt_list: &'a Vec<Stmt>,
                          scopes: &mut ScopeStack<'a>) -> Result<Vec<Value<'a>>, String> {
        let mut ret_list: Vec<Value<'a>> = vec![];
        for st in stmt_list.iter() {
            let mut values: Vec<Value<'a>> = match st {
                &Stmt::Bare{ref items, ..} => {
                    let mut result_vec = vec![];
                    for i in items.iter() {
                        result_vec.push(try!(self.eval_bare_stmt_item(i, scopes)));
                    }
                    result_vec
                }
                &Stmt::Assignment{ref items, ref expr, ref data} => {
                    let expr_value = try!(self.eval_expr(expr, scopes));
                    match expr_value {
                        Value::Tuple(value_vec) => {
                            if items.len() == value_vec.len() {
                                for (i, e) in items.iter().zip(value_vec.into_iter()) {
                                    match scopes.assign(i, e) {
                                        Ok(_) => {continue},
                                        Err(s) => {return Err(s);}
                                    }
                                }
                            } else if items.len() == 1 {
                                try!(scopes.assign(&items[0], Value::Tuple(value_vec)));
                            } else {
                                return Err(format!("EVAL FAILURE at line {}: wrong arity for this assignment", data.line + 1));
                            }
                        } 
                        _ => {
                            if items.len() == 1 {
                                try!(scopes.assign(&items[0], expr_value));
                            } else {
                                return Err(format!("EVAL FAILURE at line {}: too many idents to assign to", data.line + 1));
                            }
                        }
                    }
                    vec![]
                }
                &Stmt::While{ref condition, ref statements, ref data} => {
                    let mut result_vec = vec![];
                    loop {
                        let expr_val = try!(self.eval_expr(condition, scopes));
                        match expr_val {
                            Value::Bool(b) => {
                                if !b {
                                    break;
                                }
                            }
                            _ => {
                                return Err(format!("EVAL FAILURE at line {}: while loops must contain a boolean expression", data.line + 1));
                            }
                        }
                        let mut new_scope = HashMap::new();
                        scopes.push(&mut new_scope);
                        let vals = try!(self.eval_stmt_list(statements, scopes));
                        for val in vals.into_iter() {
                            result_vec.push(val);
                        }
                        scopes.pop();
                    }
                    result_vec
                }
                &Stmt::If{ref clauses, ref data} => {
                    let mut result_vec = vec![];
                    for clause in clauses.iter() {
                        match clause {
                            &IfClause::If{ref condition, ref statements} => {
                                let expr_val = try!(self.eval_expr(condition, scopes));
                                match expr_val {
                                    Value::Bool(b) => {
                                        if !b {
                                            continue; 
                                        }
                                    }
                                    _ => {
                                        return Err(format!("EVAL FAILURE at line {}: if statements must contain a boolean expression", data.line + 1));
                                    }
                                }
                                let mut new_scope = HashMap::new();
                                scopes.push(&mut new_scope);
                                let vals = try!(self.eval_stmt_list(statements, scopes));
                                for val in vals.into_iter() {
                                    result_vec.push(val);
                                }
                                scopes.pop();
                                break;
                            }
                            &IfClause::Else(ref statements) => {
                                let mut new_scope = HashMap::new();
                                scopes.push(&mut new_scope);
                                let vals = try!(self.eval_stmt_list(statements, scopes));
                                for val in vals.into_iter() {
                                    result_vec.push(val);
                                }
                                scopes.pop();
                                break;
                            }
                        }
                    }
                    result_vec
                }
                &Stmt::Return{..} => {
                    break;
                }
            };
            ret_list.append(&mut values);
        }
        Ok(ret_list)
    }
}
