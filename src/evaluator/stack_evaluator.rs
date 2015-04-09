use ast::*;
use std::collections::HashMap;
use std::num::{Int, Float};
use std::rc::Rc;
use std::cell::RefCell;
use super::*;
use lexer::Lexer;
use parser;

#[derive(Debug)]
enum Op<'a> {
    //Stack manipulation
    Push(Value<'a>), // _ -> a
    Pop, // a, .. -> ..
    Jump(usize), // .. -> ..
    JumpIfFalse(usize), // a -> ..
    JumpTarget, // .. -> ..
    //Scoping
    PushScope,
    PopScope,
    //Unary ops
    Get, // a, .. -> $a ..
    Neg, // a, .. -> -a ..
    Not, // a, .. -> !a ..
    //Binary ops
    Add, // b, a, .. -> a + b, ..
    Sub, // b, a, .. -> a - b, ..
    Mul, // b, a, .. -> a * b, ..
    Div, // b, a, .. -> a / b, ..
    Mod, // b, a, .. -> a % b, ..
    In, // b, a, .. -> a in b, ..
    Lt, // b, a, .. -> a < b, ..
    Lte, // b, a, .. -> a <= b, ..
    Gt, // b, a, .. -> a > b, ..
    Gte, // b, a, .. -> a >= b, ..
    Eq, // b, a, .. -> a = b, ..
    Neq, // b, a, .. -> a != b, ..
    And, // b, a, .. -> a && b, ..
    Or, // b, a, .. -> a || b, ..
    //Variables
    AssignDef(&'a str), // a, .. -> ..
    AssignVar(&'a str), // a, .. -> ..
    //Postfixes
    Access(&'a str), // a, .. -> a.b, ..
}

macro_rules! check_bin_op(
    ($a:expr, $b:expr, $op_name:expr, $stack:expr, [ $($t:path => $f:expr => $name:expr ),+ ]) => ({
        match $a {
            $(
                $t(lhs) => {
                    match $b {
                        $t(rhs) => {
                            $stack.push($t($f(lhs, rhs)));
                        }
                        v => panic!("can't do {} on {:?}", $op_name, v)
                    }
                }
            )+
            v => panic!("can't do {} on {:?}", $op_name, v)
        }
    });
);

fn gen_expr<'a>(expr: &'a Expr, ops: &mut Vec<Op<'a>>) {
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
                    let new_clip = ClipStruct {
                        params: params,
                        returns: returns,
                        statements: statements,
                        defs: new_defs
                    };
                    Value::Clip(Rc::new(RefCell::new(new_clip)))
                }
            };
            ops.push(Op::Push(v));
        }
        _ => panic!("not implemented: expr types")
    }
}

fn gen_stmt<'a>(stmt: &'a Stmt, ops: &mut Vec<Op<'a>>) {
    match stmt {
        &Stmt::Assignment{ref items, ref expr, ..} => {
            gen_expr(expr, ops);
            if items.len() > 1 {
                panic!("not implemented: tuple assignment");
            }
            match items[0] {
                StmtItem::Var(s) => {
                    ops.push(Op::AssignVar(s));
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
                        //If false, jump here
                        new_statements.push(Op::JumpTarget);
                        if_conditions.push(new_condition);
                        if_statements.push(new_statements);
                    }
                    &IfClause::Else(ref statements) => {
                        else_ops.push(Op::PushScope);
                        gen_stmt_list(statements, &mut else_ops);
                        else_ops.push(Op::PopScope);
                        //If all other cases fail, jump here
                        else_ops.push(Op::JumpTarget);
                        break;
                    }
                }
            }
            let mut skip_else_target = ops.len() + else_ops.len();
            for (cond, stmts) in if_statements.iter().zip(if_conditions.iter()) {
                //Add 1 for the jumps we're going to add
                skip_else_target += cond.len() + stmts.len() + 1;
            }
            for (mut cond, mut stmts) in if_statements.into_iter().zip(if_conditions.into_iter()) {
                //Jump here if false
                let false_target = ops.len() + cond.len() + stmts.len();
                ops.append(&mut cond);
                ops.push(Op::JumpIfFalse(false_target));
                ops.append(&mut stmts);
                //Got a true value, skip over the other clauses
                ops.push(Op::Jump(skip_else_target));
            }

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
        _ => panic!("not implemented: statement types")
    }
}

fn gen_stmt_list<'a>(statements: &Vec<Stmt<'a>>, ops: &mut Vec<Op<'a>>) {

}


pub fn test_stack(){
    println!("testing stack eval...");
    let mut lexer = Lexer::new();
    let file_string = "1 + (2 * (5 + 4) + 3) + 2".to_string();
    let result = lexer.lex(file_string);
    //let mut statements = Vec::new();
    let ast = match result {
        Err(_) => {
            panic!("failed to lex");
        }
        Ok(toks) => {
            let parse_result = parser::expr::parse_expr(&toks[..]);
            match parse_result {
                parser::ParseResult::Ok(expr, _) => expr,
                parser::ParseResult::Err(s) => panic!("failed to parse")
            }
        }
    };
    let mut stack = Vec::new();
    //let ops = vec![Op::Push(Value::String("hello ".to_string())), Op::Push(Value::String("world!".to_string())), Op::Add];
    let mut ops = Vec::new();
    gen_expr(&ast, &mut ops);
    println!("ops are: {:?}", ops);
    let mut pc: usize = 0;
    let len = ops.len();
    while pc < len {
        match ops[pc] {
            Op::Push(ref v) => {stack.push(v.clone());},
            Op::Pop => {stack.pop();},
            Op::Jump(i) => {pc = i;},
            Op::JumpIfFalse(i) => {
                let cond = stack.pop().unwrap();
                match cond {
                    Value::Bool(b) => {
                        if !b {
                            pc = i;
                        }
                    }
                    _ => panic!("need boolean for if")
                }
            }
            Op::JumpTarget => (),
            Op::PushScope => (),
            Op::PopScope => (),
            //Binary ops
            Op::Add => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                check_bin_op!(a, b, "add", stack, [
                    Value::Int => |x, y| {x + y} => "integer",
                    Value::Float => |x, y| {x + y} => "float",
                    Value::String => |x: String, y: String| {x.clone() + &y[..]} => "string"
                ])
            }
            Op::Sub => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                check_bin_op!(a, b, "sub", stack, [
                    Value::Int => |x, y| {x - y} => "integer",
                    Value::Float => |x, y| {x - y} => "float"
                ])
            }
            Op::Mul => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                check_bin_op!(a, b, "mul", stack, [
                    Value::Int => |x, y| {x * y} => "integer",
                    Value::Float => |x, y| {x * y} => "float"
                ])
            }
            Op::Div => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                check_bin_op!(a, b, "div", stack, [
                    Value::Int => |x, y| {x / y} => "integer",
                    Value::Float => |x, y| {x / y} => "float"
                ])
            }
            Op::Mod => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                check_bin_op!(a, b, "mod", stack, [
                    Value::Int => |x, y| {x % y} => "integer",
                    Value::Float => |x, y| {x % y} => "float"
                ])
            }
            _ => panic!("not implemented yet")
        }
        pc += 1;
    }
    println!("{:?}", stack);
}
