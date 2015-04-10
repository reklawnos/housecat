#![feature(plugin)]
#![plugin(regex_macros)]
#![feature(collections)]
#![feature(slice_patterns)]
#![feature(test)]
#![feature(core)]

extern crate regex;
extern crate test;

use std::io::prelude::*;
use std::fs::File;
use std::env;
use std::path::Path;
use evaluator::Evaluator;
use lexer::Lexer;

mod token;
mod ast;
mod parser;
mod lexer;
mod utils;
mod evaluator;
mod libhc;

static DEBUG: bool = false;

pub struct FileRunner<'a> {
    statements: Vec<ast::Stmt<'a>>,
    lexer: Lexer<'a>,
    pub evaluator: evaluator::ast_evaluator::AstEvaluator<'a>,
    params: Vec<&'a str>,
    returns: Vec<&'a str>
}

impl<'a> FileRunner<'a> {
    pub fn new() -> FileRunner<'a> {
        FileRunner {
            statements: Vec::new(),
            lexer: Lexer::new(),
            evaluator: evaluator::ast_evaluator::AstEvaluator::new(),
            params: Vec::new(),
            returns: Vec::new()
        }
    }

    pub fn run(&'a mut self, file_path: &Path) {
        let mut file = match File::open(file_path) {
            Err(err) => {
                 panic!("couldn't open {}: {}", file_path.display(), err);
            },
            Ok(file) => file,
        };
        let mut file_string = String::new();
        match file.read_to_string(&mut file_string) {
            Err(err) => panic!("couldn't read {}: {}", file_path.display(), err),
            Ok(_) => {}
        }
        let result = self.lexer.lex(file_string);
        match result {
            Err(s) => {
                println!("{}", s);
            }
            Ok(toks) => {
                if DEBUG {
                    println!("Parsed tokens:");
                    for t in toks.iter() {
                        println!("{:?}: {},{}", t.token, t.line + 1, t.col + 1);
                    }
                }
                let parse_result = parser::parse_tokens(&toks[..], &mut self.statements);
                match parse_result {
                    Ok(statement_vec) => {
                        if DEBUG {
                            println!("Parsed AST:");
                            for st in statement_vec.iter() {
                                println!("{:?}", st);
                            }
                        }
                        match self.evaluator.eval_file_stmts(&statement_vec, &self.params, &self.returns) {
                            //TODO: we get a clip back, we can use this for stuff.
                            Ok(_) => (),
                            Err(e) => println!("{}", e)
                        }
                        
                    }
                    Err(s) => println!("{}", s)
                }
            }
        }
    }
}

#[allow(dead_code)]
fn main() {
    let command_args: Vec<String> = env::args().collect();
    evaluator::stack_evaluator::test_stack();
    if command_args.len() <= 1 {
        println!("No .hcat file provided!");
    } else {
        let mut runner = FileRunner::new();
        libhc::open_libs(&mut runner.evaluator);
        let path = &Path::new(&command_args[1][..]);
        runner.run(path);
    }
}
