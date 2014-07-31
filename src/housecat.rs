#![feature(phase)]
#![feature(globs)]
extern crate regex;
#[phase(plugin)] extern crate regex_macros;

use regex::Regex;
use std::io::BufferedReader;
use std::io::File;
use std::os;


mod token;
mod ast;
mod parser;

fn main() {
    let args = os::args();
    let token_specs = vec!(
        (PtBool, regex!(r"^(?:true|false)")),
        (PtName, regex!(r"^[A-z]\w*")),
        (PtFloat, regex!(r"^-?[0-9]*\.[0-9]+(?:e[-+]?[0-9]+)?")),
        (PtInt, regex!(r"^-?[0-9]+")),
        (PtString, regex!("^\"(?:[^\"\\\\]|\\\\.)*\"")),
        (PtColon, regex!(r"^:")),
        (PtDot, regex!(r"^\.")),
        (PtOpenBrac, regex!(r"^\{")),
        (PtCloseBrac, regex!(r"^\}")),
        (PtOpenParen, regex!(r"^\(")),
        (PtCloseParen, regex!(r"^\)")),
        (PtBinOp, regex!(r"^[+*/-]")),
        (PtSkip, regex!(r"^\s")),
        (PtComment, regex!(r"^#"))
    );
    if args.len() <= 1 {
        //do_repl();
    } else {
        let path = Path::new(args[1].as_slice());
        let result = do_file_parse(&path, &token_specs);
        for r in result.unwrap().iter() {
            match r {
                &(ref token, line_no, col_no) => {
                    let tok_string = token.to_string();
                    println!("{}, {}:{}", tok_string, line_no + 1, col_no + 1);
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

enum ParseType {
    PtName, //ident or keyword
    PtBool,
    PtFloat,
    PtInt,
    PtString,
    PtSkip,
    PtColon,
    PtDot,
    PtOpenBrac,
    PtCloseBrac,
    PtOpenParen,
    PtCloseParen,
    PtComment,
    PtBinOp
}

fn decide_token(parse_type: ParseType, section: &str) -> token::Token {
    match parse_type {
        PtName => {
            match section {
                "def" => token::Def,
                s => {
                    token::Ident(box s.to_string())
                }
            }
        }
        PtBool => token::Bool(from_str(section).unwrap()),
        PtFloat => token::Float(from_str(section).unwrap()),
        PtInt => token::Int(from_str(section).unwrap()),
        //TODO: add support for escape characters
        PtString => {
            let mut ns = section.to_string().clone();
            //remove start and end character
            ns.pop_char();
            ns.shift_char();
            token::String(box ns)
        },
        PtColon => token::Colon,
        PtDot => token::Dot,
        PtOpenBrac => token::OpenBrac,
        PtCloseBrac => token::CloseBrac,
        PtOpenParen => token::OpenParen,
        PtCloseParen => token::CloseParen,
        PtBinOp => {
            match section {
                "+" => token::Add,
                "-" => token::Sub,
                "*" => token::Mul,
                "/" => token::Div,
                _ => fail!("Unknown binary op")
            }
        }
        _ => fail!("not implemented")
    }
}

fn do_file_parse(path: &Path, token_specs: &Vec<(ParseType, Regex)>) -> Option<Box<Vec<(token::Token, uint, uint)>>> {
    let mut file = BufferedReader::new(File::open(path));
    let mut result : Box<Vec<(token::Token, uint, uint)>> = box Vec::new();

    for (line_index, l) in file.lines().enumerate() {
        let mut line = l.unwrap();
        let mut col = 0u;
        while line.len() > 0 {
            let mut found_token = false;
            let mut found_comment = false;
            for &(parse_type, ref re) in token_specs.iter() {
                let pos = match re.find(line.as_slice()) {
                    Some(range) => range,
                    None => continue
                };
                //Skip the rest of the line if we found a comment
                match parse_type {
                    PtComment => {
                        found_comment = true;
                        break;
                    },
                    _ => {}
                }
                let (start,end) = pos;
                let cl = line.clone();
                let res = cl.as_slice().slice(start, end);
                //Skip over whitespace
                match parse_type {
                    PtSkip => {},
                    _ => {
                        let new_token = decide_token(parse_type, res);
                        result.push((new_token, line_index, col));
                    }
                }
                //Push the column index to the end of what we just read
                col += end;
                line = line.as_slice().slice_from(end).to_string();
                found_token = true;
                break;
            }

            if found_comment {
                break;
            }
            //No token was found, which means that something was invalid
            if !found_token {
                println!("Lexing failure: unrecognized symbol at line {}, column {}: '{}'",
                    line_index,
                    col + 1,
                    line.as_slice().char_at(0));
                return None;
            }
        }
    }
    Some(result)
}
