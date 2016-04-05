#![feature(slice_patterns)]
#![feature(as_unsafe_cell)]


extern crate regex;
extern crate num;
mod token;
mod ast;
mod parser;
mod lexer;
mod utils;
mod evaluator;
mod interpreter;
mod libhc;

pub use interpreter::Interpreter;
