pub mod ast_evaluator;
pub mod values;

use evaluator::values::{Value, VarType, ClipStruct};
use ast::Stmt;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

pub trait Evaluator<'a> {
     fn add_rust_clip(&mut self,
                         name: &'a str,
                         func: Box<Fn(&Vec<Value<'a>>, &mut Evaluator<'a>) -> Result<Value<'a>, String>>,
                         defs: HashMap<&'a str, VarType<'a>>);

    fn eval_file_stmts(&mut self,
                           stmt_list: &'a Vec<Stmt<'a>>,
                           params: &'a Vec<&'a str>,
                           returns: &'a Vec<&'a str>) -> Result<Rc<RefCell<ClipStruct<'a>>>, String>;
}
