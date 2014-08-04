#![feature(phase)]
#![feature(globs)]

extern crate regex;

#[phase(plugin)]
extern crate regex_macros;

use std::io::BufferedReader;
use std::io::File;
use std::os;


mod token;
mod ast;
mod parser;
mod lexer;

fn main() {
    let args = os::args();
    if args.len() <= 1 {
        //TODO: do_repl();
    } else {
        let path = Path::new(args[1].as_slice());
        let result = do_file_parse(&path);
        match result {
            Err(s) => println!("{}", s),
            Ok(toks) => {
                for t in toks.iter() {
                    match t {
                        &(ref token, line_no, col_no) => {
                            let tok_string = token.to_string();
                            println!("{}, {}:{}", tok_string, line_no + 1, col_no + 1);
                        }
                    }
                }
            }
        }
    }
    let test_toks = &[
        token::OpenParen,
        token::Ident(box "a".to_string()),
        token::Add,
        token::Ident(box "b".to_string()),
        token::CloseParen,
        token::Mul,
        token::Ident(box "c".to_string()),
        token::Sub,
        token::Ident(box "d".to_string())
    ];
    match parser::parse_expr(test_toks) {
        (exp, _) => parser::print_expr(&(box exp), 0)
    }    
}

fn do_file_parse(path: &Path) -> Result<Vec<(token::Token, uint, uint)>, String> {
    let mut file = BufferedReader::new(File::open(path));
    let mut result: Vec<(token::Token, uint, uint)> = Vec::new();
    for (line_index, l) in file.lines().enumerate() {
        let unwrapped_line = &l.unwrap();
        let res = lexer::parse_line(unwrapped_line, line_index, & mut result);
        match res {
            Ok(()) => {},
            Err(col) => {
                return Err(
                    format!(
                        "Lexing failure: unrecognized symbol at line {}, column {}: '{}'",
                        line_index + 1,
                        col + 1,
                        unwrapped_line.as_slice().char_at(0)
                    )
                );
            }
        }
    }
    Ok(result)
}
