#![feature(plugin)]
#![plugin(regex_macros)]
#![feature(collections)]
#![feature(core)]

extern crate regex;

use std::io::prelude::*;
use std::fs::File;
use std::env;
use std::path::Path;

mod token;
mod ast;
mod values;
mod parser;
mod lexer;
mod utils;
mod evaluator;
mod eval_result;
mod libhc;

static DEBUG: bool = false;

pub struct FileRunner<'a> {
    toks: Vec<token::Tok<'a>>,
    statements: Vec<ast::Stmt<'a>>,
    file_string: String,
    pub evaluator: evaluator::Evaluator<'a>,
    params: Vec<&'a str>,
    returns: Vec<&'a str>
}

impl<'a> FileRunner<'a> {
    pub fn new() -> FileRunner<'a> {
        FileRunner {
            toks: Vec::new(),
            statements: Vec::new(),
            file_string: String::new(),
            evaluator: evaluator::Evaluator::new(),
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
        match file.read_to_string(&mut self.file_string) {
            Err(err) => panic!("couldn't read {}: {}", file_path.display(), err),
            Ok(_) => {}
        }
        let result = do_file_parse(&self.file_string, & mut self.toks);
        match result {
            Err(s) => {
                println!("{}", s);
            }
            Ok(()) => {
                if DEBUG {
                    println!("Parsed tokens:");
                    for t in self.toks.iter() {
                        println!("{:?}: {},{}", t.token, t.line + 1, t.col + 1);
                    }
                }
                let parse_result = parser::parse_tokens(&self.toks[..], &mut self.statements);
                match parse_result {
                    parser::Result::Ok(statement_vec, _) => {
                        if DEBUG {
                            println!("Parsed AST:");
                            for st in statement_vec.iter() {
                                println!("{:?}", st);
                            }
                        }
                        match self.evaluator.eval_file_stmts(&statement_vec, &self.params, &self.returns) {
                            //TODO: we get a clip back, we can use this for stuff.
                            eval_result::Result::Ok(_) => (),
                            eval_result::Result::Err(e) => println!("{}", e)
                        }
                        
                    }
                    parser::Result::Err(s) => println!("{}", s)
                }
            }
        }
    }
}

fn main() {
    let command_args: Vec<String> = env::args().collect();
    if command_args.len() <= 1 {
        println!("No .hcat file provided!");
    } else {
        let mut runner = FileRunner::new();
        libhc::open_libs(&mut runner.evaluator);
        let path = &Path::new(&command_args[1][..]);
        runner.run(path);
    }
}

fn do_file_parse<'a>(lines: &'a String, result_vec: & mut Vec<token::Tok<'a>>) -> Result<(), String> {
    let mut char_index = 0usize;
    for (line_index, l) in lines[..].split("\n").enumerate() {
        let res = lexer::lex_line(l, line_index, &mut char_index, result_vec);
        match res {
            Ok(()) => {},
            Err(col) => {
                return Err(
                    format!(
                        "LEXING FAILURE at {},{}: invalid character {}\n{}\n{}",
                        line_index + 1,
                        col + 1,
                        l.chars().nth(col).unwrap(),
                        l,
                        utils::get_caret_string(col)
                    )
                );
            }
        }
    }
    Ok(())
}
