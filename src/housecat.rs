#![feature(phase)]
extern crate regex;
#[phase(plugin)] extern crate regex_macros;

use regex::Regex;
use std::io::BufferedReader;
use std::io::File;
use std::os;

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
		let path = Path::new(args.get(1).as_slice());
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
}

enum Token {
	// Keywords
	Def,                // 'def'
	// Symbols
	Eof,                // End of file
	Colon,              // :
	Dot,                // .
	OpenBrac,           // {
	CloseBrac,          // }
	OpenParen,          // (
	CloseParen,         // )
	// User values
	Bool(bool),         // 'true' or 'false'
	Int(i64),           // ex. 1324, -43
	Float(f64),         // ex. 1.3, -34.432e-4
	Ident(Box<String>), // ex. foo, bar
	String(Box<String>),
	// Binary ops
	Add,                // '+'
	Sub,                // '-'
	Mul,                // '*'
	Div,                // '/'
}

mod Ast {
	enum Literal {
		LitBool(bool), // `true` or `false`
		LitInt(i64),   // integers
		LitFloat(f64), // floats
		LitString(Box<String>), // string literals
		LitNil, // `nil`
	}
	//
	//Expressions
	enum Exp {
		ExpMonoOp(MonoOp, Box<Exp>), // MonoOp Exp
		ExpBinaryOp(BinaryOp, Box<Exp>, Box<Exp>), // Exp BinaryOp Exp
		ExpLiteral(Literal), // Literal
		//ExpClosure(Dec), // { Dec }
		ExpIf(Box<Exp>, Box<Exp>, Box<Exp>) // `if` Exp Exp <`else` Exp> 
	}

	enum MonoOp {
		MonNeg, // `-` (number negation)
	}

	enum BinaryOp {
		BinAdd, // `+`
		BinSub, // `-`
		BinMul, // `*`
		BinDiv,  // `/`
	}
}


impl Token {
	fn to_string(&self) -> String {
		match self {
			&Def => format!("Def"),
			&Ident(ref s) => format!("Ident({})", s),
			&Bool(b) => format!("Bool({})", b),
			&Float(f) => format!("Float({})", f),
			&Int(i) => format!("Int({})", i),
			&String(ref s) => format!("String({})", s),
			&Colon => format!("Colon"),
			&Dot => format!("Dot"),
			&OpenBrac => format!("OpenBrac"),
			&CloseBrac => format!("CloseBrac"),
			&OpenParen => format!("OpenParen"),
			&CloseParen => format!("CloseParen"),
			&Add => format!("Add"),
			&Sub => format!("Sub"),
			&Div => format!("Div"),
			&Mul => format!("Mul"),
			_ => fail!("not implemented")
		}
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

fn decide_token(parse_type: ParseType, section: &str) -> Token {
	match parse_type {
		PtName => {
			match section {
				"def" => Def,
				s => {
					Ident(box s.to_string())
				}
			}
		}
		PtBool => Bool(from_str(section).unwrap()),
		PtFloat => Float(from_str(section).unwrap()),
		PtInt => Int(from_str(section).unwrap()),
		//TODO: add support for escape characters
		PtString => {
			let mut ns = section.to_string().clone();
			//remove start and end character
			ns.pop_char();
			ns.shift_char();
			String(box ns)
		},
		PtColon => Colon,
		PtDot => Dot,
		PtOpenBrac => OpenBrac,
		PtCloseBrac => CloseBrac,
		PtOpenParen => OpenParen,
		PtCloseParen => CloseParen,
		PtBinOp => {
			match section {
				"+" => Add,
				"-" => Sub,
				"*" => Mul,
				"/" => Div,
				_ => fail!("Unknown binary op")
			}
		}
		_ => fail!("not implemented")
	}
}

fn do_file_parse(path: &Path, token_specs: &Vec<(ParseType, Regex)>) -> Option<Box<Vec<(Token, uint, uint)>>> {
	let mut file = BufferedReader::new(File::open(path));
	let mut result : Box<Vec<(Token, uint, uint)>> = box Vec::new();

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
