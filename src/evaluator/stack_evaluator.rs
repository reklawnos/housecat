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
    JumpIf(usize), // a -> ..
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
        &Expr::BinOp{ref lhs, ref rhs, ref op, ..} => {
            gen_expr(lhs, ops);
            gen_expr(rhs, ops);
            let new_op = match op {
                &BinOp::Add => Op::Add,
                &BinOp::Mul => Op::Mul,
                _ => panic!("not implemented: bin op types")
            };
            ops.push(new_op);
        }
        _ => panic!("not implemented: expr types")
    }
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
                parser::ParseResult::Ok(expr, _) => {
                    expr
                }
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
            Op::JumpIf(i) => {
                let cond = stack.pop().unwrap();
                match cond {
                    Value::Bool(b) => {
                        if b {
                            pc = i;
                        }
                    }
                    _ => panic!("need boolean for if")
                }
            }
            Op::Add => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                check_bin_op!(a, b, "add", stack, [
                    Value::Int => |x, y| {x + y} => "integer",
                    Value::Float => |x, y| {x + y} => "float",
                    Value::String => |x: String, y: String| {x.clone() + &y[..]} => "string"
                ])
            }
            Op::Mul => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                check_bin_op!(a, b, "multiply", stack, [
                    Value::Int => |x, y| {x * y} => "integer",
                    Value::Float => |x, y| {x * y} => "float"
                ])
            }
            _ => panic!("not implemented yet")
        }
        pc += 1;
    }
    println!("{:?}", stack);
}
