use std::collections::HashMap;
use std::cell::{RefCell};
use std::rc::Rc;

use super::ops::{Op, ClipParts};
use super::values::{Value, FloatWrap, ClipStruct, ClipHolder};

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
                                (a, b) => panic!("can't perform operation {} with LHS of {:?} and RHS of {:?}", $op_name, a, b)
                            }
                        }
                        $stack.push(Value::Tuple(result_vec));           
                    }
                    _ => panic!("can't perform operation {} with a tuple and a non-tuple")
                }
                
            }
            v => panic!("can't perform operation {} with LHS of {:?}", $op_name, v)
        }
    });
);

pub fn execute<'a>(ops: *const Vec<Op<'a>>, stack: &mut Vec<Value<'a>>,
                   vars: &mut Vec<HashMap<&'a str, Value<'a>>>,
                   defs: *mut HashMap<Value<'a>, Value<'a>>) {
    let mut pc: usize = 0;
    let len = unsafe {(*ops).len()};
    while pc < len {
        match *unsafe {&(*ops)[pc]} {
            Op::Push(ref v) => {stack.push((**v).clone());},
            Op::PushClip(ref parts) => {
                match **parts {
                    ClipParts{ref params, ref returns, ref ops} => {
                        let new_clip = ClipStruct {
                            params: params.clone(),
                            returns: returns.clone(),
                            statements: (*ops).clone(),
                            defs: HashMap::new()
                        };
                        stack.push(Value::Clip(ClipHolder(Rc::new(RefCell::new(new_clip)))))
                    }
                }
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
                    _ => panic!("{}: need boolean for if", pc)
                }
            }
            Op::JumpTarget => (),
            Op::PushScope => vars.push(HashMap::new()),
            Op::PopScope => {vars.pop();},
            Op::Load(ref s) => {
                let mut found_var = false;
                match unsafe{(*defs).get(&Value::String(s.to_string()))} {
                    Some(v) => {
                        stack.push(v.clone());
                        found_var = true;
                    }
                    None => ()
                }
                if !found_var {
                    for scope in vars.iter().rev() {
                        match scope.get(s) {
                            Some(v) => {
                                stack.push(v.clone());
                                found_var = true;
                                break;
                            }
                            None => continue
                        }
                    }
                }
                if !found_var {
                    panic!("{}: could not find {} in any scope", pc, s);
                }
            }
            Op::Store(ref s) => {
                let a = stack.pop().unwrap();
                let top_idx = vars.len() - 1;
                vars[top_idx].insert(s, a);
            }
            Op::Def(ref key) => {
                match stack.pop().unwrap() {
                    Value::Clip(ref c) => {
                        let mut clip = c.0.borrow_mut();
                        let value = stack.pop().unwrap();
                        clip.defs.insert((**key).clone(), value);
                    }
                    _ => panic!("can't def on a non-clip")
                };
                
            }
            Op::DefSelf(ref key) => {
                let mut found_var = false;
                match **key {
                    Value::String(ref s) => {
                        for scope in vars.iter_mut().rev() {
                            if scope.contains_key(&s[..]) {
                                let value = stack.pop().unwrap();
                                scope.insert(&*s, value);
                                found_var = true;
                                break;
                            }
                        }
                    }
                    _ => ()
                }
                if !found_var {
                    let value = stack.pop().unwrap();
                    unsafe {(*defs).insert((**key).clone(), value)};
                }
            }
            Op::Access(ref b) => {
                match stack.pop().unwrap() {
                    Value::Clip(ref c) => {
                        let clip = c.0.borrow_mut();
                        let new_val = match clip.defs.get(b) {
                            Some(v) => v,
                            None => panic!("{}: no key {} in clip", pc, b)
                        };
                        stack.push(new_val.clone());
                    }
                    _ => panic!("can't access a non-clip")
                };
            }
            Op::Play(n) => {
                let mut params = Vec::new();
                for _ in 0..n {
                    params.push(stack.pop().unwrap());
                }
                match stack.pop().unwrap() {
                    Value::Clip(ref c) => {
                        let mut clip = c.0.borrow_mut();
                        let mut new_scope = HashMap::new();
                        for (ident, value) in clip.params.iter().zip(params.into_iter()) {
                            new_scope.insert(*ident, value);
                        }
                        for ident in clip.returns.iter() {
                            new_scope.insert(*ident, Value::Nil);
                        }
                        {
                            vars.push(new_scope);
                            let mut temp_stack = Vec::new();
                            execute(&mut clip.statements, &mut temp_stack, vars, &mut clip.defs);
                            let idx = vars.len() - 1;
                            if clip.returns.len() == 0 {
                                stack.push(Value::Nil);
                            } else if clip.returns.len() == 1 {
                                stack.push(vars[idx].remove(clip.returns[0]).unwrap());
                            } else {
                                let mut ret_vec = Vec::new();
                                for ret in clip.returns.iter() {
                                    ret_vec.push(vars[idx].remove(*ret).unwrap());
                                }
                                stack.push(Value::Tuple(ret_vec));
                            }
                            
                            vars.pop();
                        }
                    }
                    _ => panic!("can't run a non-clip")
                }
            }
            //Unary ops
            Op::Get => {
                match stack.pop().unwrap() {
                    Value::Clip(ref c) => {
                        let mut clip = c.0.borrow_mut();
                        let new_scope = HashMap::new();
                        {
                            vars.push(new_scope);
                            let mut temp_stack = Vec::new();
                            execute(&mut clip.statements, &mut temp_stack, vars, &mut clip.defs);
                            vars.pop();
                        }
                        stack.push(Value::Clip(c.clone()));
                    }
                    _ => panic!("can't get a non-clip")
                }
            }
            Op::Neg => {
                let a = stack.pop().unwrap();
                match a {
                    Value::Int(i) => stack.push(Value::Int(-i)),
                    Value::Float(f) => stack.push(Value::Float(FloatWrap::new(-f.get()))),
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
                    Value::Float, Value::Float => |x: FloatWrap, y: FloatWrap| {FloatWrap::new(x.get() + y.get())} => Value::Float,
                    Value::String, Value::String => |x: String, y: String| {x.clone() + &y[..]} => Value::String
                ])
            }
            Op::Sub => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                check_bin_op!(a, b, "-", stack, [
                    Value::Int, Value::Int => |x, y| {x - y} => Value::Int,
                    Value::Float, Value::Float => |x: FloatWrap, y: FloatWrap| {FloatWrap::new(x.get() - y.get())} => Value::Float
                ])
            }
            Op::Mul => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                check_bin_op!(a, b, "*", stack, [
                    Value::Int, Value::Int => |x, y| {x * y} => Value::Int,
                    Value::Float, Value::Float => |x: FloatWrap, y: FloatWrap| {FloatWrap::new(x.get() * y.get())} => Value::Float
                ])
            }
            Op::Div => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                check_bin_op!(a, b, "/", stack, [
                    Value::Int, Value::Int => |x, y| {x / y} => Value::Int,
                    Value::Float, Value::Float => |x: FloatWrap, y: FloatWrap| {FloatWrap::new(x.get() / y.get())} => Value::Float
                ])
            }
            Op::Mod => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                check_bin_op!(a, b, "%", stack, [
                    Value::Int, Value::Int => |x, y| {x % y} => Value::Int,
                    Value::Float, Value::Float => |x: FloatWrap, y: FloatWrap| {FloatWrap::new(x.get() % y.get())} => Value::Float
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
                    Value::Float, Value::Float => |x: FloatWrap, y: FloatWrap| {x.get() < y.get()} => Value::Bool
                ])
            }
            Op::Lte => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                check_bin_op!(a, b, "<=", stack, [
                    Value::Int, Value::Int => |x, y| {x <= y} => Value::Bool,
                    Value::Float, Value::Float => |x: FloatWrap, y: FloatWrap| {x.get() <= y.get()} => Value::Bool
                ])
            }
            Op::Gt => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                check_bin_op!(a, b, ">", stack, [
                    Value::Int, Value::Int => |x, y| {x > y} => Value::Bool,
                    Value::Float, Value::Float => |x: FloatWrap, y: FloatWrap| {x.get() > y.get()} => Value::Bool
                ])
            }
            Op::Gte => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                check_bin_op!(a, b, ">=", stack, [
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
        //println!("{}: {:?}", pc, stack);
        pc += 1;
    }
    println!("{:?}", stack);
}
