#[macro_use]
mod macros;
pub mod ast_evaluator;
pub mod values;
pub mod stack_evaluator;
mod scopestack;

use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

use ast::Stmt;

pub use self::values::{ClipStruct, RustClip, Value, VarType};
pub use self::scopestack::ScopeStack;

pub type RustClipFunc<'a> = Fn(&Vec<Value<'a>>, &mut Evaluator<'a>) -> Result<Value<'a>, String>;

pub trait Evaluator<'a> {
     fn add_rust_clip(&mut self,
                         name: &'a str,
                         func: Box<RustClipFunc<'a>>,
                         defs: HashMap<&'a str, VarType<'a>>);

    fn eval_file_stmts(&mut self,
                           stmt_list: &'a Vec<Stmt<'a>>,
                           params: &'a Vec<&'a str>,
                           returns: &'a Vec<&'a str>) -> Result<Rc<RefCell<ClipStruct<'a>>>, String>;
}
