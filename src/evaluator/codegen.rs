use ast::*;
use super::ops::{Op, ClipParts};
use super::values::{Value, FloatWrap};



fn codegen_failure<T>(line_number: usize, message: &str) -> Result<T, String> {
    Err(format!("CODEGEN FAILURE at line {}: {}", line_number + 1, message))
}

fn gen_expr<'a>(expr: &'a Expr<'a>, ops: &mut Vec<Op>) -> Result<(), String> {
    let &Expr{ref expr, ref data} = expr;
    match expr {
        &ExprType::UnOp{ref expr, ref op, ..} => {
            try!(gen_expr(expr, ops));
            let new_op = match op {
                &UnOp::Neg => Op::Neg,
                &UnOp::Not => Op::Not,
                &UnOp::Get => Op::Get
            };
            ops.push(new_op);
            Ok(())
        }
        &ExprType::BinOp{ref lhs, ref rhs, ref op, ..} => {
            try!(gen_expr(lhs, ops));
            try!(gen_expr(rhs, ops));
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
                _ => {return codegen_failure(data.line, "binary op not implemented");}
            };
            ops.push(new_op);
            Ok(())
        }
        &ExprType::Literal{ref value, ..} => {
            match value {
                &Literal::Bool(b) => ops.push(Op::Push(Box::new(Value::Bool(b)))),
                &Literal::Int(i) => ops.push(Op::Push(Box::new(Value::Int(i)))),
                &Literal::Float(f) => ops.push(Op::Push(Box::new(Value::Float(FloatWrap::new(f))))),
                &Literal::String(s) => ops.push(Op::Push(Box::new(Value::String(s.to_string())))),
                &Literal::Nil => ops.push(Op::Push(Box::new(Value::Nil))),
                &Literal::Clip{ref params, ref returns, ref statements} => {
                    let mut func_ops = Vec::new();
                    try!(gen_stmt_list(statements, &mut func_ops));
                    ops.push(Op::PushClip(Box::new(ClipParts{
                        params: params.iter().map(|p| p.to_string()).collect(),
                        returns: returns.iter().map(|r| r.to_string()).collect(),
                        ops: func_ops
                    })));
                }
            }
            Ok(())
        }
        &ExprType::Ident{ref name, ..} => {
            ops.push(Op::Load(name.to_string()));
            Ok(())
        }
        &ExprType::Postfix{ref expr, ref postfixes, ..} => {
            try!(gen_expr(expr, ops));
            for postfix in postfixes.iter() {
                match postfix {
                    &Postfix::Play(ref params) => {
                        for expr in params.iter().rev() {
                            try!(gen_expr(expr, ops));
                        }
                        ops.push(Op::Play(params.len()));
                    }
                    &Postfix::PlaySelf(ref ident, ref params) => {
                        ops.push(Op::Access(Box::new(Value::String(ident.to_string()))));
                        for expr in params.iter().rev() {
                            try!(gen_expr(expr, ops));
                        }
                        ops.push(Op::PlaySelf(params.len()));
                    }
                    &Postfix::Index(ref expr) => {
                        try!(gen_expr(expr, ops));
                        ops.push(Op::GetAndAccess);
                    }
                    &Postfix::Access(ref s) => ops.push(Op::AccessPop(Box::new(Value::String(s.to_string()))))
                }
            }
            Ok(())
        }
        &ExprType::Tuple{ref values, ..} => {
            //Reverse the order, that way we don't have to reverse it at runtime
            for expr in values.iter().rev() {
                try!(gen_expr(expr, ops));
            }
            if values.len() > 1 {
                ops.push(Op::MakeTuple(values.len()));
            }
            Ok(())
        }
    }
}

fn eval_expr_as_ident_values<'a>(expr: &'a Expr) -> Result<Vec<&'a str>, String> {
    let &Expr{ref expr, ref data} = expr;
    match expr {
        &ExprType::Ident{name, ..} => Ok(vec![name]),
        //TODO: implement idents for defining interior values
        &ExprType::Postfix{ref expr, ref postfixes} => {
            let mut result_vec = Vec::new();
            match **expr {
                Expr{expr: ExprType::Ident{name, ..}, ..} => {
                    result_vec.push(name);
                }
                _ => {return codegen_failure(data.line, "cannot assign to a non-ident");}
            }
            for postfix in postfixes.iter() {
                match postfix {
                    &Postfix::Access(s) => {
                        result_vec.push(s);
                    }
                    //TODO: need to do this for index types, too
                    _ => {return codegen_failure(data.line, "cannot assign to a non-ident");}
                }
            }
            Ok(result_vec)
        }
        _ => codegen_failure(data.line, "cannot assign to a non-ident")
    }
}


fn eval_expr_as_ident_str<'a>(expr: &'a Expr) -> Result<&'a str, String> {
    let &Expr{ref expr, ref data} = expr;
    match expr {
        &ExprType::Ident{name, ..} => Ok(name),
        _ => codegen_failure(data.line, "not an ident type")
    }
}

fn gen_stmt<'a>(stmt: &'a Stmt, ops: &mut Vec<Op>) -> Result<(), String> {
    let &Stmt{ref stmt, ref data} = stmt;
    match stmt {
        &StmtType::Assign{ref items, ref expr, ..} => {
            try!(gen_expr(expr, ops));
            if items.len() > 1 {
                panic!("not implemented: tuple assignment");
            }
            match items[0] {
                StmtItem::Var(s) => {
                    ops.push(Op::DeclareAndStore(s.to_string()));
                }
                StmtItem::Bare(ref expr) => {
                    let key = try!(eval_expr_as_ident_str(expr));
                    ops.push(Op::Store(key.to_string()));
                }
                StmtItem::Expr(_) => { return codegen_failure(data.line, "cannot assign to expression"); }
            }
            Ok(())
        }
        &StmtType::Def{ref items, ref expr, ..} => {
            let mut new_ops = Vec::new();
            try!(gen_expr(expr, &mut new_ops));
            println!("new_ops: {:?}", &new_ops);
            ops.append(&mut new_ops);
            if items.len() > 1 {
                panic!("not implemented: tuple defs");
            }
            match items[0] {
                StmtItem::Bare(ref expr) => {
                    let mut keys = try!(eval_expr_as_ident_values(expr));
                    let assign_key = keys.pop().unwrap();
                    if keys.len() > 0 {
                        let base_ident = keys.remove(0);
                        ops.push(Op::Load(base_ident.to_string()));
                        for ident in keys.into_iter() {
                            ops.push(Op::Access(Box::new(Value::String(ident.to_string()))));
                        }
                        ops.push(Op::Def(Box::new(Value::String(assign_key.to_string()))));
                    } else {
                        ops.push(Op::DefSelf(Box::new(Value::String(assign_key.to_string()))));
                    }
                    Ok(())
                }
                StmtItem::Expr(ref expr) => {
                    try!(gen_expr(expr, ops));
                    ops.push(Op::DefPop);
                    Ok(())
                }
                _ => {return codegen_failure(data.line, "cannot def without bare item");}
            }
        }
        &StmtType::Bare{ref items, ..} => {
            for item in items.iter() {
                match item {
                    &StmtItem::Bare(ref expr) => try!(gen_expr(expr, ops)),
                    _ => {return codegen_failure(data.line, "cannot have a non-bare statement item in a bare statement");}
                }
            }
            Ok(())
        }
        &StmtType::If{ref clauses, ..} => {
            let mut if_conditions = Vec::new();
            let mut if_statements = Vec::new();
            let mut else_ops = Vec::new();
            for clause in clauses.iter() {
                match clause {
                    &IfClause::If{ref condition, ref statements} => {
                        let mut new_condition = Vec::new();
                        let mut new_statements = Vec::new();
                        new_statements.push(Op::PushScope);
                        try!(gen_expr(condition, &mut new_condition));
                        try!(gen_stmt_list(statements, &mut new_statements));
                        new_statements.push(Op::PopScope);
                        if_conditions.push(new_condition);
                        if_statements.push(new_statements);
                    }
                    &IfClause::Else(ref statements) => {
                        else_ops.push(Op::PushScope);
                        try!(gen_stmt_list(statements, &mut else_ops));
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
            Ok(())
        }
        &StmtType::While{ref condition, ref statements, ..} => {
            //Index to jump to when continuing = first jump target
            let continue_jump_idx = ops.len();
            ops.push(Op::JumpTarget);
            try!(gen_expr(condition, ops));
            let mut body_ops = Vec::new();
            try!(gen_stmt_list(statements, &mut body_ops));
            //JumpIfFalse to the jump target after the statement list and the continue jump
            let break_jump_idx = ops.len() + body_ops.len() + 2;
            ops.push(Op::JumpIfFalse(break_jump_idx));
            ops.append(&mut body_ops);
            //Jump back to the beginning to continue
            ops.push(Op::Jump(continue_jump_idx));
            ops.push(Op::JumpTarget);
            Ok(())
        }
        &StmtType::For{ref idents, ref iterator, ref statements, ..} => {
            if idents.len() > 1 {
                panic!("not implemented: tuple assignment in for loops");
            }
            try!(gen_expr(iterator, ops));
            ops.push(Op::PushScope);
            ops.push(Op::PushIterator);
            //Index to jump to when continuing = first jump target
            let continue_jump_idx = ops.len();
            //Code for: iterator|next() != nil
            ops.push(Op::JumpTarget);
            ops.push(Op::RetrieveIterator);
            ops.push(Op::Access(Box::new(Value::String("next".to_string()))));
            ops.push(Op::PlaySelf(0));
            ops.push(Op::DeclareAndStore(idents[0].to_string()));
            ops.push(Op::Load(idents[0].to_string()));
            ops.push(Op::Push(Box::new(Value::Nil)));
            ops.push(Op::Neq);
            let mut body_ops = Vec::new();
            try!(gen_stmt_list(statements, &mut body_ops));
            //JumpIfFalse to the jump target after the statement list and the continue jump
            let break_jump_idx = ops.len() + body_ops.len() + 2;
            ops.push(Op::JumpIfFalse(break_jump_idx));
            ops.append(&mut body_ops);
            //Jump back to the beginning to continue
            ops.push(Op::Jump(continue_jump_idx));
            ops.push(Op::JumpTarget);
            ops.push(Op::PopIterator);
            ops.push(Op::PopScope);
            Ok(())
        }
        &StmtType::Return => {ops.push(Op::Return); Ok(())}
    }
}

pub fn gen_stmt_list<'a>(statements: &'a Vec<Stmt<'a>>, ops: &mut Vec<Op>) -> Result<(), String> {
    for statement in statements.iter() {
        try!(gen_stmt(statement, ops));
    }
    Ok(())
}
