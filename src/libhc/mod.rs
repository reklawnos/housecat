mod io;
//mod core;

use libhc::io::open_io;
//use libhc::core::open_core;
use evaluator::stack_evaluator::values::RustClip;
use std::collections::HashMap;

#[allow(dead_code)]
pub fn open_libs<'a>() -> HashMap<&'a str, Box<RustClip<'a>>> {
    // let mut result = HashMap::new();
    // open_io(&mut result);
    // //open_core(&mut result);
    // result
    open_io()
}
