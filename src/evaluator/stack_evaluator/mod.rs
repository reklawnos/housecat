mod ops;
pub mod values;

use ast::*;
use std::collections::HashMap;
use std::num::{Int, Float};
use super::*;
use lexer::Lexer;
use parser;
mod codegen;

use self::codegen::gen_expr;
use self::ops::Op;
pub use self::values::Value;

type RustClipFuncStack<'a> = Fn(&Vec<Value<'a>>, &mut Evaluator<'a>) -> Result<Value<'a>, String>;


macro_rules! check_bin_op(
    ($a:expr, $b:expr, $op_name:expr, $stack:expr, [ $($lhs_type:path, $rhs_type:path => $f:expr => $result_type:path),+ ]) => ({
        match $a {
            $(
                $lhs_type(lhs) => {
                    match $b {
                        $rhs_type(rhs) => {
                            $stack.push($result_type($f(lhs, rhs)));
                        }
                        v => panic!("can't perform operation {} with LHS of {:?} and RHS of {:?}", $op_name, lhs, v)
                    }
                }
            )+
            v => panic!("can't perform operation {} with LHS of {:?}", $op_name, v)
        }
    });
);

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
                check_bin_op!(a, b, "+", stack, [
                    Value::Int, Value::Int => |x, y| {x + y} => Value::Int,
                    Value::Float, Value::Float => |x, y| {x + y} => Value::Float,
                    Value::String, Value::String => |x: String, y: String| {x.clone() + &y[..]} => Value::String
                ])
            }
            Op::Sub => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                check_bin_op!(a, b, "-", stack, [
                    Value::Int, Value::Int => |x, y| {x - y} => Value::Int,
                    Value::Float, Value::Float => |x, y| {x - y} => Value::Float
                ])
            }
            Op::Mul => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                check_bin_op!(a, b, "*", stack, [
                    Value::Int, Value::Int => |x, y| {x * y} => Value::Int,
                    Value::Float, Value::Float => |x, y| {x * y} => Value::Float
                ])
            }
            Op::Div => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                check_bin_op!(a, b, "/", stack, [
                    Value::Int, Value::Int => |x, y| {x / y} => Value::Int,
                    Value::Float, Value::Float => |x, y| {x / y} => Value::Float
                ])
            }
            Op::Mod => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                check_bin_op!(a, b, "%", stack, [
                    Value::Int, Value::Int => |x, y| {x % y} => Value::Int,
                    Value::Float, Value::Float => |x, y| {x % y} => Value::Float
                ])
            }
            Op::In => {
                panic!("not implemented: 'in' operation");
            }
            Op::Lt => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                check_bin_op!(a, b, "<", stack, [
                    Value::Int, Value::Int => |x, y| {x < y} => Value::Bool,
                    Value::Float, Value::Float => |x, y| {x < y} => Value::Bool
                ])
            }
            Op::Lte => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                check_bin_op!(a, b, "<=", stack, [
                    Value::Int, Value::Int => |x, y| {x <= y} => Value::Bool,
                    Value::Float, Value::Float => |x, y| {x <= y} => Value::Bool
                ])
            }
            Op::Gt => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                check_bin_op!(a, b, ">", stack, [
                    Value::Int, Value::Int => |x, y| {x > y} => Value::Bool,
                    Value::Float, Value::Float => |x, y| {x > y} => Value::Bool
                ])
            }
            Op::Gte => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                check_bin_op!(a, b, ">=", stack, [
                    Value::Int, Value::Int => |x, y| {x >= y} => Value::Bool,
                    Value::Float, Value::Float => |x, y| {x >= y} => Value::Bool
                ])
            }
            Op::Eq => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(Value::Bool(a == b));
            }
            Op::Neq => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(Value::Bool(a != b));
            }
            Op::And => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                check_bin_op!(a, b, "&&", stack, [
                    Value::Bool, Value::Bool => |x, y| {x && y} => Value::Bool
                ])
            }
            Op::Or => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                check_bin_op!(a, b, "||", stack, [
                    Value::Bool, Value::Bool => |x, y| {x || y} => Value::Bool
                ])
            }
            _ => panic!("not implemented yet")
        }
        pc += 1;
    }
    println!("{:?}", stack);
}
