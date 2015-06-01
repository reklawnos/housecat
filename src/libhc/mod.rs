mod io;
mod core;

use libhc::io::{Print};
use libhc::core::{Import};
use evaluator::stack_evaluator::values::RustClip;
use std::collections::HashMap;

#[allow(dead_code)]
pub fn open_libs<'a>() -> HashMap<&'static str, Box<RustClip<'a>>> {
    let mut result = HashMap::new();
    result.insert("print", Box::new(Print) as Box<RustClip>);
    result.insert("import", Box::new(Import) as Box<RustClip>);
    result
}
