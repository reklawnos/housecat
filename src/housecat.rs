#![feature(phase)]
extern crate regex;
#[phase(plugin)] extern crate regex_macros;

//use std::io;
use std::io::BufferedReader;
use std::io::File;
use std::os;

fn main() {
	let args = os::args();
	if args.len() <= 1 {
		//do_repl();
	} else {
		let path = Path::new(args.get(1).as_slice());
		do_file_parse(&path);
	}
}

enum Token {
	// Keywords
	Func,               // 'func'
	Var,                // 'var'
	// Symbols
	Eof,                // End of file
	Colon,              // :
	OpenBrac,           // {
	CloseBrac,          // }
	OpenParen,          // (
	CloseParen,         // )
	// User values
	Ident(Box<String>), // ex. foo, bar
	Int(int),           // ex. 1324, -43
	Float(f64),         // ex. 1.3, -34.432e-4
	String(Box<String>),
	// Binary ops
	Add,                // '+'
	Sub,                // '-'
	Mul,                // '*'
	Div,                // '/'
}

enum ParseType {
	PtName, //ident or keyword
	PtFloat,
	PtInt,
	PtString,
	PtSkip,
	PtColon,
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
				"func" => Func,
				"var" => Var,
				s => {
					Ident(box s.to_string())
				}
			}
		}
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

fn do_file_parse(path: &Path) {
	let token_specs = [
		(PtName, regex!(r"^[A-z]\w*")),
		(PtFloat, regex!(r"^-?[0-9]*\.[0-9]+(?:e[-+]?[0-9]+)?")),
		(PtInt, regex!(r"^-?[0-9]+")),
		(PtString, regex!("^\"(?:[^\"\\\\]|\\\\.)*\"")),
		(PtColon, regex!(r"^:")),
		(PtOpenBrac, regex!(r"^\{")),
		(PtCloseBrac, regex!(r"^\}")),
		(PtOpenParen, regex!(r"^\(")),
		(PtCloseParen, regex!(r"^\)")),
		(PtBinOp, regex!(r"^[+*/-]")),
		(PtSkip, regex!(r"^\s")),
		(PtComment, regex!(r"^#"))
	];

	let mut file = BufferedReader::new(File::open(path));
	let mut result : Vec<Token> = Vec::new();

	for (line_index, l) in file.lines().enumerate() {
		println!("------------lexing line {}", line_index);
		let mut line = l.unwrap();
		let mut col = 0u;
		while line.len() > 0 {
			let mut found_token = false;
			let mut found_comment = false;
			println!("----------this is one go through")
			for &(parse_type, ref re) in token_specs.iter() {
				let pos = match re.find(line.as_slice()) {
					Some(range) => range,
					None => continue
				};
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
				println!("The result at position {} is: '{}'", pos, res);
				//Skip over whitespace
				match parse_type {
					PtSkip => {},
					_ => {
						let new_token = decide_token(parse_type, res);
						result.push(new_token);
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
				//println!("line is {}", line);
				println!("Lexing failure: unrecognized symbol at line {}, column {}: '{}'",
					line_index,
					col + 1,
					line.as_slice().char_at(0));
				return;
			}
		}
	}

	for r in result.iter() {
		match r {
			&Func => println!("Func"),
			&Var => println!("Var"),
			&Ident(ref s) => println!("Ident({})", s),
			&Float(f) => println!("Float({})", f),
			&Int(i) => println!("Int({})", i),
			&String(ref s) => println!("String({})", s),
			&Colon => println!("Colon"),
			&OpenBrac => println!("OpenBrac"),
			&CloseBrac => println!("CloseBrac"),
			&OpenParen => println!("OpenParen"),
			&CloseParen => println!("CloseParen"),
			&Add => println!("Add"),
			&Sub => println!("Sub"),
			&Div => println!("Div"),
			&Mul => println!("Mul"),
			_ => fail!("not implemented")
		}
	}
}
