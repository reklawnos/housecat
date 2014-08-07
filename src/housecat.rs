#![feature(phase)]
#![feature(globs)]
#![feature(macro_rules)]

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
    //TODO: do_repl();
    if args.len() <= 1 {
        println!("No .hcat file provided!");
    } else {
        let path = Path::new(args[1].as_slice());
        let result = do_file_parse(&path);
        let toks = match result {
            Err(s) => fail!("{}", s),
            Ok(toks) => toks
        };
        for t in toks.iter() {
            println!("{}: {},{}", t.token, t.line + 1, t.col + 1);
        }
        match parser::parse_expr(toks.as_slice()) {
            (exp, _) => println!("{}", exp)
        }
    }
}

fn do_file_parse(path: &Path) -> Result<Vec<token::Tok>, String> {
    let mut file = BufferedReader::new(File::open(path));
    let mut result: Vec<token::Tok> = Vec::new();
    for (line_index, l) in file.lines().enumerate() {
        let unwrapped_line = &l.unwrap();
        let res = lexer::parse_line(unwrapped_line, line_index, & mut result);
        match res {
            Ok(()) => {},
            Err(col) => {
                return Err(
                    format!(
                        "ERROR at {},{}: Lexing failure: unrecognized symbol '{}'",
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
