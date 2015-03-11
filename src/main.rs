#![feature(plugin)]
#![plugin(regex_macros)]
#![feature(io)] 
#![feature(fs)] 
#![feature(path)]

extern crate regex;

use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use std::env;
use std::path::Path;



mod token;
mod ast;
mod parser;
mod lexer;
mod utils;


fn main() {
    let command_args : Vec<String> = env::args().collect();
    //TODO: do_repl();
    if command_args.len() <= 1 {
        println!("No .hcat file provided!");
    } else {
        let path = &Path::new(&command_args[1][..]);
        let file = match File::open(path) {
            Err(err) => {
                 panic!("couldn't open {}: {}", path.display(), err);
            },
            Ok(file) => file,
        };
        let br = BufReader::new(file);
        let mut file_lines: Vec<String> = Vec::new();
        for line in br.lines() {
            file_lines.push(line.unwrap());
        }
        let mut toks: Vec<token::Tok> = Vec::new();
        let result = do_file_parse(&file_lines, & mut toks);
        match result {
            Err(s) => {
                println!("{}", s);
            }
            Ok(()) => {
                for t in toks.iter() {
                    println!("{:?}: {},{}", t.token, t.line + 1, t.col + 1);
                }
                match parser::parse_expr(&toks[..]) {
                    Ok((exp, _)) => println!("{:?}", exp),
                    Err(s) => println!("{}", s)
                }
            }
        }
    }
}


fn do_file_parse<'a>(lines: &'a Vec<String>, result_vec: & mut Vec<token::Tok<'a>>) -> Result<(), String> {
    for (line_index, l) in lines.iter().enumerate() {
        let res = lexer::parse_line(&l, line_index, result_vec);
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
