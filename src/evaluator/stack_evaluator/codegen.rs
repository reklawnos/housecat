use ast::*;
use super::ops::Op;
//use super::*;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use super::values::{Value, ClipStruct};

pub fn gen_expr<'a>(expr: &'a Expr, ops: &mut Vec<Op<'a>>) {
    match expr {
        &Expr::UnOp{ref expr, ref op, ..} => {
            gen_expr(expr, ops);
            let new_op = match op {
                &UnOp::Neg => Op::Neg,
                &UnOp::Not => Op::Not,
                &UnOp::Get => Op::Get
            };
            ops.push(new_op);
        }
        &Expr::BinOp{ref lhs, ref rhs, ref op, ..} => {
            gen_expr(lhs, ops);
            gen_expr(rhs, ops);
            let new_op = match op {
                &BinOp::Add => Op::Add,
                &BinOp::Sub => Op::Sub,
                &BinOp::Mul => Op::Mul,
                &BinOp::Div => Op::Div,
                &BinOp::Mod => Op::Mod,
                &BinOp::In => Op::In,
                &BinOp::Lt => Op::Lt,
                &BinOp::Lte => Op::Lte,
                &BinOp::Gt => Op::Gt,
                &BinOp::Gte => Op::Gte,
                &BinOp::Eq => Op::Eq,
                &BinOp::Neq => Op::Neq,
                &BinOp::And => Op::And,
                &BinOp::Or => Op::Or,
                _ => panic!("not implemented: bin op types")
            };
            ops.push(new_op);
        }
        &Expr::Literal{ref value, ..} => {
            let v = match value {
                &Literal::Bool(b) => Value::Bool(b),
                &Literal::Int(i) => Value::Int(i),
                &Literal::Float(f) => Value::Float(f),
                &Literal::String(s) => Value::String(s.to_string()),
                &Literal::Nil => Value::Nil,
                &Literal::Clip{ref params, ref returns, ref statements} => {
                    let new_defs = HashMap::new();
                    let mut func_ops = Vec::new();
                    gen_stmt_list(statements, &mut func_ops);
                    let new_clip = ClipStruct {
                        params: params.clone(),
                        returns: returns.clone(),
                        statements: func_ops,
                        defs: new_defs
                    };
                    Value::Clip(Rc::new(RefCell::new(new_clip)))
                }
            };
            ops.push(Op::Push(v));
        }
        &Expr::Ident{ref name, ..} => {
            ops.push(Op::Load(name));
        }
        &Expr::Postfix{ref expr, ref postfixes, ..} => {
            gen_expr(expr, ops);
            for postfix in postfixes.iter() {
                match postfix {
                    &Postfix::Play(_) => panic!("not implemented: play postfix"),
                    &Postfix::Index(_) => panic!("not implemented: index postfix"),
                    &Postfix::Access(ref s) => ops.push(Op::Access(s))
                }
            }
        }
        &Expr::Tuple{ref values, ..} => {
            //Reverse the order, that way we don't have to reverse it at runtime
            for expr in values.iter().rev() {
                gen_expr(expr, ops);
            }
            ops.push(Op::MakeTuple(values.len()));
        }
    }
}

fn eval_expr_as_ident<'a>(expr: &'a Expr) -> Vec<&'a str> {
    match expr {
        &Expr::Ident{name, ..} => vec![name],
        //TODO: implement idents for defining interior values
        &Expr::Postfix{ref expr, ref postfixes, ref data} => {
            let mut result_vec = Vec::new();
            match **expr {
                Expr::Ident{name, ..} => {
                    result_vec.push(name);
                }
                _ => panic!("EVAL FAILURE at line {}: cannot assign to a non-ident", data.line + 1)
            }
            for postfix in postfixes.iter() {
                match postfix {
                    &Postfix::Access(s) => {
                        result_vec.push(s);
                    }
                    //TODO: need to do this for index types, too
                    _ => panic!("EVAL FAILURE at line {}: cannot assign to a non-ident", data.line + 1)
                }
            }
            result_vec
        }
        _ => panic!("EVAL FAILURE: cannot assign to a non-ident")
    }
}

pub fn gen_stmt<'a>(stmt: &'a Stmt, ops: &mut Vec<Op<'a>>) {
    match stmt {
        &Stmt::Assignment{ref items, ref expr, ..} => {
            gen_expr(expr, ops);
            if items.len() > 1 {
                panic!("not implemented: tuple assignment");
            }
            match items[0] {
                StmtItem::Var(s) => {
                    ops.push(Op::Store(s));
                }
                StmtItem::Bare(ref expr) => {
                    let idents = eval_expr_as_ident(expr);
                    if idents.len() > 1 {
                        panic!("not implemented: assign to clip");
                    }
                    ops.push(Op::Store(idents[0]));
                }
                _ => panic!("not implemented: defs or bare assignment")
            }
        }
        &Stmt::Bare{ref items, ..} => {
            for item in items.iter() {
                match item {
                    &StmtItem::Bare(ref expr) => gen_expr(expr, ops),
                    _ => panic!("cannot have a non-bare statement item in a bare statement")
                }
            }
        }
        &Stmt::If{ref clauses, ..} => {
            let mut if_conditions = Vec::new();
            let mut if_statements = Vec::new();
            let mut else_ops = Vec::new();
            for clause in clauses.iter() {
                match clause {
                    &IfClause::If{ref condition, ref statements} => {
                        let mut new_condition = Vec::new();
                        let mut new_statements = Vec::new();
                        new_statements.push(Op::PushScope);
                        gen_expr(condition, &mut new_condition);
                        gen_stmt_list(statements, &mut new_statements);
                        new_statements.push(Op::PopScope);
                        if_conditions.push(new_condition);
                        if_statements.push(new_statements);
                    }
                    &IfClause::Else(ref statements) => {
                        else_ops.push(Op::PushScope);
                        gen_stmt_list(statements, &mut else_ops);
                        else_ops.push(Op::PopScope);
                        break;
                    }
                }
            }
            //If any cases succeed, jump here
            if else_ops.len() > 0 {
                else_ops.push(Op::JumpTarget);
            }
            let mut skip_else_target = ops.len() + else_ops.len() - 1;
            for (cond, stmts) in if_statements.iter().zip(if_conditions.iter()) {
                //Add 1 for the jumps we're going to add
                skip_else_target += cond.len() + stmts.len() + 3;
            }
            for (mut stmts, mut cond) in if_statements.into_iter().zip(if_conditions.into_iter()) {
                //Jump here if false
                let false_target = ops.len() + cond.len() + stmts.len() + 2;
                //println!("cond is {:?}", &cond);
                ops.append(&mut cond);
                ops.push(Op::JumpIfFalse(false_target));
                ops.append(&mut stmts);
                //Got a true value, skip over the other clauses
                ops.push(Op::Jump(skip_else_target));
                //If false, jump here
                ops.push(Op::JumpTarget);
            }
            ops.append(&mut else_ops);

        }
        &Stmt::While{ref condition, ref statements, ..} => {
            //Index to jump to when continuing = first jump target
            let continue_jump_idx = ops.len();
            ops.push(Op::JumpTarget);
            gen_expr(condition, ops);
            let mut body_ops = Vec::new();
            gen_stmt_list(statements, &mut body_ops);
            //JumpIfFalse to the jump target after the statement list and the continue jump
            let break_jump_idx = ops.len() + body_ops.len() + 2;
            ops.push(Op::JumpIfFalse(break_jump_idx));
            ops.append(&mut body_ops);
            //Jump back to the beginning to continue
            ops.push(Op::Jump(continue_jump_idx));
            ops.push(Op::JumpTarget);
        }
        &Stmt::Return{..} => panic!("not implemented: return statement")
    }
}

pub fn gen_stmt_list<'a>(statements: &'a Vec<Stmt<'a>>, ops: &mut Vec<Op<'a>>) {
    for statement in statements.iter() {
        gen_stmt(statement, ops);
    }
}
