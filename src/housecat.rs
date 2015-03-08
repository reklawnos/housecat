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


fn main() {
    let command_args : Vec<String> = env::args().collect();
    //TODO: do_repl();
    if command_args.len() <= 1 {
        println!("No .hcat file provided!");
    } else {
        let path = Path::new(&command_args[1][..]);
        let result = do_file_parse(&path);
        match result {
            Err(s) => {
                println!("{}", s);
            }
            Ok(toks) => {
                for t in toks.iter() {
                    println!("{:?}: {},{}", t.token, t.line + 1, t.col + 1);
                }
                match parser::parse_expr(&toks[..]) {
                    (exp, _) => println!("{:?}", exp)
                }
            }
        }
    }
}

fn do_file_parse(path: &Path) -> Result<Vec<token::Tok>, String> {
    let file = match File::open(path) {
        Err(err) => {
            return Err(format!("couldn't open {}: {}", path.display(), err));
        },
        Ok(file) => file,
    };
    let br = BufReader::new(file);

    let mut result: Vec<token::Tok> = Vec::new();
    for (line_index, l) in br.lines().enumerate() {
        let unwrapped_line = &l.unwrap();
        let res = lexer::parse_line(unwrapped_line, line_index, & mut result);
        match res {
            Ok(()) => {},
            Err(col) => {
                let mut caret_string = String::with_capacity(col + 2);
                for _ in 0..col {
                    caret_string.push(' ');
                }
                caret_string.push('^');
                return Err(
                    format!(
                        "ERROR at {},{}: Lexing failure: syntax error, invalid character {}\n{}\n{}",
                        line_index + 1,
                        col + 1,
                        unwrapped_line[..].char_at(col),
                        unwrapped_line,
                        caret_string
                    )
                );
            }
        }
    }
    Ok(result)
}
