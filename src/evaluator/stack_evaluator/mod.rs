mod ops;
pub mod values;

use ast::*;
use std::collections::HashMap;
use std::num::{Int, Float};
use super::*;
use lexer::Lexer;
use parser;
mod codegen;

use self::codegen::gen_stmt_list;
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

fn print_ops(ops: &Vec<Op>) {
    for (idx, op) in ops.iter().enumerate() {
        println!("{}: {:?}", idx, op);
    }
}

pub fn test_stack(){
    println!("testing stack eval...");
    let mut lexer = Lexer::new();
    let file_string =
    "\
    var x: 1\
    while x < 10
        \"go\"
        x
        x: x + 1
    end
    if false
        1
        2
        3
    end".to_string();
    let result = lexer.lex(file_string);
    let mut statements = Vec::new();
    let ast = match result {
        Err(_) => {
            panic!("failed to lex");
        }
        Ok(toks) => {
            let parse_result = parser::parse_tokens(&toks[..], &mut statements);
            match parse_result {
                Ok(v) => v,
                Err(s) => panic!("failed to parse: {}", s)
            }
        }
    };
    let mut stack = Vec::new();
    let mut vars: HashMap<&str, Value> = HashMap::new();
    //let ops = vec![Op::Push(Value::String("hello ".to_string())), Op::Push(Value::String("world!".to_string())), Op::Add];
    let mut ops = Vec::new();
    gen_stmt_list(&ast, &mut ops);
    //println!("ops are: {:?}", ops);
    print_ops(&ops);
    let mut pc: usize = 0;
    let len = ops.len();
    while pc < len {
        match ops[pc] {
            Op::Push(ref v) => {stack.push(v.clone());},
            Op::Pop => {stack.pop();},
            Op::MakeTuple(arity) => {
                let mut tuple_vec = Vec::new();
                for _ in 0..arity {
                    tuple_vec.push(stack.pop().unwrap());
                }
                stack.push(Value::Tuple(tuple_vec));
            }
            Op::Jump(i) => {pc = i;},
            Op::JumpIfTrue(i) => {
                let cond = stack.pop().unwrap();
                match cond {
                    Value::Bool(b) => {
                        if b {
                            pc = i;
                        }
                    }
                    _ => panic!("{}: need boolean for if", pc)
                }
            }
            Op::JumpIfFalse(i) => {
                let cond = stack.pop().unwrap();
                match cond {
                    Value::Bool(b) => {
                        if !b {
                            pc = i;
                        }
                    }
                    _ => panic!("{}: need boolean for if", pc)
                }
            }
            Op::JumpTarget => (),
            Op::PushScope => (),
            Op::PopScope => (),
            Op::Load(ref s) => {
                stack.push(vars.get(s).unwrap().clone());
            }
            Op::Store(ref s) => {
                let a = stack.pop().unwrap();
                vars.insert(s, a);
            }
            Op::Access(ref s) => {
                panic!("not implemented: access op")
            }
            //Unary ops
            Op::Get => {
                panic!("not implemented: eval get op"); 
            }
            Op::Neg => {
                let a = stack.pop().unwrap();
                match a {
                    Value::Int(i) => stack.push(Value::Int(-i)),
                    Value::Float(f) => stack.push(Value::Float(-f)),
                    _ => panic!("{}: cannot negate a non-bool value", pc)
                }
            }
            Op::Not => {
                let a = stack.pop().unwrap();
                match a {
                    Value::Bool(b) => stack.push(Value::Bool(!b)),
                    _ => panic!("{}: cannot apply ! to a non-bool value", pc)
                }
            }
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
        }
        pc += 1;
    }
    println!("{:?}", stack);
}
