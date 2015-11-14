use std::collections::HashMap;
use std::cell::{RefCell};
use std::rc::Rc;
use std::fmt::Display;

use super::ops::{Op, ClipParts};
use super::values::{Value, FloatWrap};
use super::environment::Environment;
use super::standard_clip::StdClip;
use super::clip::ClipHolder;

macro_rules! check_bin_op(
    ($a:expr, $b:expr, $op_name:expr, $stack:expr, $pc:expr, [ $($lhs_type:path, $rhs_type:path => $f:expr => $result_type:path),+ ]) => ({
        match $a {
            $(
                $lhs_type(lhs) => {
                    match $b {
                        $rhs_type(rhs) => {
                            $stack.push($result_type($f(lhs, rhs)));
                        }
                        v => {return exec_failure($pc, format!("can't perform operation {} with LHS of {:?} and RHS of {:?}", $op_name, lhs, v));}
                    }
                }
            )+
            Value::Tuple(lhs_vals) => {
                match $b {
                    Value::Tuple(rhs_vals) => {
                        let mut result_vec = Vec::new();
                        for (lhs, rhs) in lhs_vals.into_iter().zip(rhs_vals.into_iter()) {
                            match (lhs, rhs) {
                                $(
                                    ($lhs_type(lhs), $rhs_type(rhs)) => {
                                        result_vec.push($result_type($f(lhs, rhs)));
                                    }
                                )+
                                (a, b) => {return exec_failure($pc, format!("can't perform operation {} with LHS of {:?} and RHS of {:?}", $op_name, a, b));}
                            }
                        }
                        $stack.push(Value::Tuple(result_vec));
                    }
                    _ => {return exec_failure($pc, format!("can't perform operation {} with a tuple and a non-tuple", $op_name));}
                }
            }
            v => {return exec_failure($pc, format!("can't perform operation {} with LHS of {:?}", $op_name, v));}
        }
    });
);

fn exec_failure<T, D: Display>(pc: usize, message: D) -> Result<T, String> {
    Err(format!("EXECUTION FAILURE at PC {}: {}", pc, message))
}

pub fn execute(ops: *const Vec<Op>, stack: &mut Vec<Value>,
                   vars: &mut Environment,
                   defs: *mut HashMap<Value, Value>) -> Result<(), String> {
    let mut pc: usize = 0;
    let len = unsafe {(*ops).len()};
    let mut iterators = Vec::new();
    while pc < len {
        match *unsafe {&(*ops)[pc]} {
            Op::Push(ref v) => {stack.push((**v).clone());},
            Op::PushClip(ref clip) => {
                stack.push(Value::Clip(ClipHolder::new(Box::new(StdClip::new(
                    clip.params.clone(),
                    clip.returns.clone(),
                    clip.ops.clone()
                )))));
            }
            Op::MakeTuple(arity) => {
                let mut tuple_vec = Vec::new();
                for _ in 0..arity {
                    tuple_vec.push(stack.pop().unwrap());
                }
                stack.push(Value::Tuple(tuple_vec));
            }
            Op::Jump(i) => {pc = i;},
            Op::JumpIfFalse(i) => {
                let cond = stack.pop().unwrap();
                match cond {
                    Value::Bool(b) => {
                        if !b {
                            pc = i;
                        }
                    }
                    _ => {return exec_failure(pc, "need boolean for if");}
                }
            }
            Op::JumpTarget => (),
            Op::Return => {return Ok(());},
            Op::PushIterator => {
                let a = stack.pop().unwrap();
                iterators.push(a);
            }
            Op::PopIterator => {
                iterators.pop();
            }
            Op::RetrieveIterator => {
                let idx = iterators.len() - 1;
                stack.push(iterators[idx].clone());
            }
            Op::PushScope => vars.push_frame(),
            Op::PopScope => vars.pop_frame(),
            Op::Load(ref s) => {
                match vars.get_var(s) {
                    Some(v) => {
                        stack.push(v);
                    }
                    None => {
                        return exec_failure(pc, format!("could not find `{}` in any scope", s));
                    }
                }
            }
            Op::DeclareAndStore(ref s) => {
                let a = stack.pop().unwrap();
                vars.declare_var(s.clone(), a);
            }
            Op::Store(ref s) => {
                let value = stack.pop().unwrap();
                vars.set_var(s.clone(), value);
            }
            Op::Def(ref key) => {
                match stack.pop().unwrap() {
                    Value::Clip(ref mut c) => {
                        let mut clip = c.borrow_mut();
                        let value = stack.pop().unwrap();
                        clip.set((**key).clone(), value);
                    }
                    _ => {return exec_failure(pc, "can't def on a non-clip");}
                };
            }
            Op::DefPop => {
                let key = stack.pop().unwrap();
                let value = stack.pop().unwrap();
                unsafe {(*defs).insert(key, value);}
            }
            Op::DefSelf(ref key) => {
                let value = stack.pop().unwrap();
                unsafe {(*defs).insert((**key).clone(), value);}
            }
            Op::GetAndAccess => {
                let b = stack.pop().unwrap();
                match stack.pop().unwrap() {
                    Value::Clip(ref mut c) => {
                        let clip = c.borrow_mut();
                        let new_val = clip.get(&b);
                        stack.push(new_val.clone());
                    }
                    _ => {return exec_failure(pc, "can't access a non-clip");}
                };
            }
            Op::Access(ref b) => {
                let idx = stack.len() - 1;
                let new_val = match stack[idx] {
                    Value::Clip(ref mut c) => {
                        let clip = c.borrow_mut();
                        clip.get(b)
                    }
                    _ => {return exec_failure(pc, "can't access a non-clip");}
                };
                stack.push(new_val);
            }
            Op::AccessPop(ref b) => {
                match stack.pop().unwrap() {
                    Value::Clip(ref mut c) => {
                        let clip = c.borrow_mut();
                        let new_val = clip.get(b);
                        stack.push(new_val.clone());
                    }
                    _ => {return exec_failure(pc, "can't access a non-clip");}
                };
            }
            Op::Play(n) => {
                let mut params = Vec::new();
                for _ in 0..n {
                    params.push(stack.pop().unwrap());
                }
                match stack.pop().unwrap() {
                    Value::Clip(ref mut c) => {
                        let mut clip = c.borrow_mut();
                        vars.push_frame();
                        let result = try!(clip.play(params, vars));
                        vars.pop_frame();
                        stack.push(result);
                    }
                    _ => {return exec_failure(pc, "can't run a non-clip");}
                }
            }
            Op::PlaySelf(n) => {
                let mut params = Vec::new();
                for _ in 0..n {
                    params.push(stack.pop().unwrap());
                }
                match stack.pop().unwrap() {
                    Value::Clip(ref mut c) => {
                        let mut clip = c.borrow_mut();
                        params.insert(0, stack.pop().unwrap());
                        vars.push_frame();
                        let result = try!(clip.play(params, vars));
                        vars.pop_frame();
                        stack.push(result);
                    }
                    _ => {return exec_failure(pc, "can't run a non-clip");}
                }
            }
            //Unary ops
            Op::Get => {
                match stack.pop().unwrap() {
                    Value::Clip(ref mut c) => {
                        {
                            let mut clip = c.borrow_mut();
                            vars.push_frame();
                            try!(clip.play(Vec::new(), vars));
                            vars.pop_frame();
                        }
                        stack.push(Value::Clip(c.clone()));
                    }
                    _ => {return exec_failure(pc, "can't use the get operator on a non-clip");}
                }
            }
            Op::Neg => {
                let a = stack.pop().unwrap();
                match a {
                    Value::Int(i) => stack.push(Value::Int(-i)),
                    Value::Float(f) => stack.push(Value::Float(FloatWrap::new(-f.get()))),
                    _ => {return exec_failure(pc, "cannot negate a non-numeric value");}
                }
            }
            Op::Not => {
                let a = stack.pop().unwrap();
                match a {
                    Value::Bool(b) => stack.push(Value::Bool(!b)),
                    _ => {return exec_failure(pc, "cannot apply ! to a non-boolean value");}
                }
            }
            //Binary ops
            Op::Add => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                check_bin_op!(a, b, "+", stack, pc, [
                    Value::Int, Value::Int => |x, y| {x + y} => Value::Int,
                    Value::Float, Value::Float => |x: FloatWrap, y: FloatWrap| {FloatWrap::new(x.get() + y.get())} => Value::Float,
                    Value::String, Value::String => |x: String, y: String| {x.clone() + &y[..]} => Value::String
                ])
            }
            Op::Sub => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                check_bin_op!(a, b, "-", stack, pc, [
                    Value::Int, Value::Int => |x, y| {x - y} => Value::Int,
                    Value::Float, Value::Float => |x: FloatWrap, y: FloatWrap| {FloatWrap::new(x.get() - y.get())} => Value::Float
                ])
            }
            Op::Mul => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                check_bin_op!(a, b, "*", stack, pc, [
                    Value::Int, Value::Int => |x, y| {x * y} => Value::Int,
                    Value::Float, Value::Float => |x: FloatWrap, y: FloatWrap| {FloatWrap::new(x.get() * y.get())} => Value::Float
                ])
            }
            Op::Div => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                check_bin_op!(a, b, "/", stack, pc, [
                    Value::Int, Value::Int => |x, y| {x / y} => Value::Int,
                    Value::Float, Value::Float => |x: FloatWrap, y: FloatWrap| {FloatWrap::new(x.get() / y.get())} => Value::Float
                ])
            }
            Op::Mod => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                check_bin_op!(a, b, "%", stack, pc, [
                    Value::Int, Value::Int => |x, y| {x % y} => Value::Int,
                    Value::Float, Value::Float => |x: FloatWrap, y: FloatWrap| {FloatWrap::new(x.get() % y.get())} => Value::Float
                ])
            }
            Op::In => {
                // let b = stack.pop().unwrap();
                // let a = stack.pop().unwrap();
                // match b {

                // }
                panic!("not implemented: binary in")
            }
            Op::Lt => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                check_bin_op!(a, b, "<", stack, pc, [
                    Value::Int, Value::Int => |x, y| {x < y} => Value::Bool,
                    Value::Float, Value::Float => |x: FloatWrap, y: FloatWrap| {x.get() < y.get()} => Value::Bool
                ])
            }
            Op::Lte => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                check_bin_op!(a, b, "<=", stack, pc, [
                    Value::Int, Value::Int => |x, y| {x <= y} => Value::Bool,
                    Value::Float, Value::Float => |x: FloatWrap, y: FloatWrap| {x.get() <= y.get()} => Value::Bool
                ])
            }
            Op::Gt => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                check_bin_op!(a, b, ">", stack, pc, [
                    Value::Int, Value::Int => |x, y| {x > y} => Value::Bool,
                    Value::Float, Value::Float => |x: FloatWrap, y: FloatWrap| {x.get() > y.get()} => Value::Bool
                ])
            }
            Op::Gte => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                check_bin_op!(a, b, ">=", stack, pc, [
                    Value::Int, Value::Int => |x, y| {x >= y} => Value::Bool,
                    Value::Float, Value::Float => |x: FloatWrap, y: FloatWrap| {x.get() >= y.get()} => Value::Bool
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
                check_bin_op!(a, b, "&&", stack, pc, [
                    Value::Bool, Value::Bool => |x, y| {x && y} => Value::Bool
                ])
            }
            Op::Or => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                check_bin_op!(a, b, "||", stack, pc, [
                    Value::Bool, Value::Bool => |x, y| {x || y} => Value::Bool
                ])
            }
        }
        // println!("{}: {:?}", pc, stack);
        pc += 1;
    }
    Ok(())
    //println!("{:?}", stack);
}
