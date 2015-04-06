mod io;
mod core;

use libhc::io::open_io;
use libhc::core::open_core;
use evaluator::Evaluator;

#[allow(dead_code)]
pub fn open_libs<'a>(eval: &mut Evaluator<'a>) {
    open_io(eval);
    open_core(eval);
}
