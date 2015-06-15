#![feature(plugin)]
#![plugin(regex_macros)]
#![feature(collections)]
#![feature(slice_patterns)]
#![feature(test)]
#![feature(core)]

extern crate regex;
extern crate test;
extern crate core;
extern crate num;

use std::env;
use interpreter::Interpreter;

mod token;
mod ast;
mod parser;
mod lexer;
mod utils;
mod evaluator;
mod interpreter;
mod libhc;

#[allow(dead_code)]
fn main() {
    let command_args: Vec<String> = env::args().collect();
    if command_args.len() <= 1 {
        println!("No .hcat file provided!");
    } else {
        let mut interpreter = Interpreter::new();
        match interpreter.interpret_file(&command_args[1][..]) {
            Ok(_) => (),
            Err(s) => println!("{}", s)
        }
    }
}
