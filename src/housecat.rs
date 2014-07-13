#![feature(phase)]
extern crate regex;
#[phase(plugin)] extern crate regex_macros;
extern crate regex;
use std::io;
use std::io::BufferedReader;
use std::io::File;
use std::os;

fn main() {
	let args = os::args();
	if args.len() <= 1 {
		do_repl();
	} else {
		let path = Path::new(args.get(1).as_slice());
		do_file_parse(&path);
	}
}

fn do_repl() {
	print!("> ");
	for line in io::stdin().lines() {
		let input = line.unwrap();
		let s = clean_input(&input);
		parse_line(&s);
		print!("> ");
	}
}

fn do_file_parse(path: &Path) {
	let mut file = BufferedReader::new(File::open(path));
	for line in file.lines() {
	    let input = line.unwrap();
		let s = clean_input(&input);
		parse_line(&s);
	}
}

fn clean_input(s: &String) -> String {
	return s.as_slice().trim().to_string();
}

enum Token {
	Eof,
	Func,
	Ident(Box<String>),
	Int(int),
	Float(f64),
}

enum LexerState {
	Root,
	Name,
	NumberPreDecimal,
}

fn parse_line(s: &String) {
	let mut result : Vec<Token> = Vec::new();
	let mut lexer_stack : Vec<LexerState> = Vec::new();
	lexer_stack.push(Root);
	for w in s.as_slice().words() {
		println!("Parsing word: {}", w);
		match to_number(w) {
			Some(t) => {result.push(t); continue},
			_ => {}
		}
		match to_name(w) {
			Some(t) => {result.push(t); continue},
			_ => {}
		}
	}
	print!("\n----------Line done! Results are:\n");
	for t in result.iter() {
		match t {
			&Func => println!("Func"),
			&Ident(ref i) => println!("Ident({})", i),
			&Float(n) => println!("Float({})", n),
			_ => fail!("not sure what else could happen")
		}
	}
}

//Parses number literals
fn to_number(s: &str) -> Option<Token> {
	if regex!(r"^-?[0-9]*\.?[0-9]+(e[-+]?[0-9]+)?$").is_match(s) {
		let new_number : Option<f64> = from_str(s);
		match new_number {
			Some(n) => Some(Float(n)),
			_ => fail!("Could not parse number.")
		}
	} else {
		None
	}
}

//Parses identifiers and keywords used by the language
fn to_name(s: &str) -> Option<Token> {
	if regex!(r"^[A-z]\w*$").is_match(s) {
		match s {
			"func" => Some(Func),
			_ => Some(Ident(box s.to_string()))
		}
	} else {
		None
	}
}
