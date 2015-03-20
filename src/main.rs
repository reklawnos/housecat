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

static DEBUG: bool = false;

fn main() {
    let command_args: Vec<String> = env::args().collect();
    //TODO: do_repl();
    if command_args.len() <= 1 {
        println!("No .hcat file provided!");
    } else {
        let path = &Path::new(&command_args[1][..]);
        let mut file = match File::open(path) {
            Err(err) => {
                 panic!("couldn't open {}: {}", path.display(), err);
            },
            Ok(file) => file,
        };
        let mut file_string: String = String::new();
        match file.read_to_string(&mut file_string) {
            Err(err) => panic!("couldn't read {}: {}", path.display(), err),
            Ok(_) => {}
        }
        let mut toks: Vec<token::Tok> = Vec::new();
        let mut statements: Vec<ast::Stmt> = Vec::new();
        let result = do_file_parse(&file_string, & mut toks);
        match result {
            Err(s) => {
                println!("{}", s);
            }
            Ok(()) => {
                if DEBUG {
                    println!("Parsed tokens:");
                    for t in toks.iter() {
                        println!("{:?}: {},{}", t.token, t.line + 1, t.col + 1);
                    }
                }
                let parse_result = parser::parse_base_statements(&toks[..], &mut statements);
                match parse_result {
                    parser::Result::Ok(vec, _) => {
                        if DEBUG {
                            println!("Parsed AST:");
                            for st in vec.iter() {
                                println!("{:?}", st);
                            }
                        }
                        match evaluator::eval_file_stmts(&vec) {
                            evaluator::Result::Ok(r) => println!("result: {:?}", r),
                            evaluator::Result::Err(e) => println!("{}", e)
                        }
                        
                    }
                    parser::Result::Err(s) => println!("{}", s)
                }
            }
        }
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
                        l[..].char_at(col),
                        l,
                        utils::get_caret_string(col)
                    )
                );
            }
        }
    }
    Ok(())
}
