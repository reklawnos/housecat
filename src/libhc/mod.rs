mod io;

use libhc::io::open_io;
use evaluator::Evaluator;

pub fn open_libs<'a>(eval: &mut Evaluator<'a>) {
    open_io(eval);
}
