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
	let test_toks = &[OpenParen, Ident(box "a".to_string()), Add, Ident(box "b".to_string()), Mul, Ident(box "c".to_string()), Sub, Ident(box "d".to_string())];
	match parse_expr(test_toks) {
		(exp, _) => print_expr(&(box exp), 0)
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
	pub enum Literal {
		LitBool(bool), // 'true' or 'false'
		LitInt(i64),   // integers
		LitFloat(f64), // floats
		LitString(Box<String>), // string literals
		LitNil, // 'nil'
	}

	//Expressions
	pub enum Exp {
		ExpMonoOp(MonoOp, Box<Exp>), // <MonoOp> <Exp>
		ExpBinaryOp(BinaryOp, Box<Exp>, Box<Exp>), // <Exp> <BinaryOp> <Exp>
		ExpLiteral(Literal), // Literal
		ExpIdent(Box<String>), // Ident
		ExpIf(Box<Exp>, Box<Exp>, Box<Exp>) // if <Exp> <Exp> <`else` Exp> 
	}

	//Statements
	pub enum Statement {
		StAssignment(Box<String>, Exp) // 
	}

	pub enum MonoOp {
		MonNeg, // '-'' (number negation)
		MonNot, // '!' (boolean not)
	}

	pub enum BinaryOp {
		BinAdd, // '+'
		BinSub, // '-'
		BinMul, // '*'
		BinDiv,  // '/'
	}
}

mod test_ast {
	pub enum expr {
		TermAsExpr(Box<term>),
		PlusExpr(Box<term>, Box<expr>),
		MinusExpr(Box<term>, Box<expr>),
	}

	pub enum term {
		FactorAsTerm(Box<factor>),
		MultTerm(Box<factor>, Box<term>),
		DivTerm(Box<factor>, Box<term>),
	}

	pub enum factor {
		Id(Box<String>),
		ParenthesizedExpr(Box<expr>)
	}
}

fn parse_expr(tokens: &[Token]) -> (test_ast::expr, &[Token]) {
	match parse_term(tokens) {
		(parsed_term, tokens_after_term) => {
			match tokens_after_term {
				// ... + <expr>
				[Add, ..tokens_after_plus] => {
					match parse_expr(tokens_after_plus) {
						(parsed_expr, tokens_after_expr) =>
							(test_ast::PlusExpr(box parsed_term, box parsed_expr), tokens_after_expr),
					}
				},
				// ... - <expr>
				[Sub, ..tokens_after_minus] => {
					match parse_expr(tokens_after_minus) {
						(parsed_expr, tokens_after_expr) =>
							(test_ast::MinusExpr(box parsed_term, box parsed_expr), tokens_after_expr),
					}
				},
				// <term>
				_ => (test_ast::TermAsExpr(box parsed_term), tokens_after_term),
			}
		}
	}
}

fn parse_term(tokens: &[Token]) -> (test_ast::term, &[Token]) {
	match parse_factor(tokens) {
		(parsed_factor, tokens_after_factor) => {
			match tokens_after_factor {
				// ... * <term>
				[Mul, ..tokens_after_mul] => {
					match parse_term(tokens_after_mul) {
						(parsed_term, tokens_after_term) =>
							(test_ast::MultTerm(box parsed_factor, box parsed_term), tokens_after_term),
					}
				},
				// ... / <term>
				[Div, ..tokens_after_div] => {
					match parse_term(tokens_after_div) {
						(parsed_term, tokens_after_term) => {
							(test_ast::DivTerm(box parsed_factor, box parsed_term), tokens_after_term)
						},
					}
				},
				// <factor>
				_ => (test_ast::FactorAsTerm(box parsed_factor), tokens_after_factor),
			}
		}
	}
}

fn parse_factor(tokens: &[Token]) -> (test_ast::factor, &[Token]) {
	match tokens {
		// <ident>
		[Ident(ref id), ..tokens_after_ident] => {
			(test_ast::Id(id.clone()), tokens_after_ident)
		},
		// ... ( ...
		[OpenParen, ..tokens_after_openparen] => {
			match parse_expr(tokens_after_openparen) {
				(parsed_expr, tokens_after_expr) => {
					match tokens_after_expr {
						// ... )
						[CloseParen, ..tokens_after_closeparen] => {
							(test_ast::ParenthesizedExpr(box parsed_expr), tokens_after_closeparen)
						},
						_ => fail!("no matching paren")
					}
				}
			}
		},
		_ => fail!("unrecognized symbol or something")
	}
}

fn print_expr(e: &Box<test_ast::expr>, ind: uint) {
	let mut indent = String::from_str("");
	indent.grow(ind, ' ');
	match **e {
		test_ast::TermAsExpr(ref term) => {
			print!("{}TermAsExpr(\n", indent);
			print_term(term, ind + 4);
			print!("{})\n", indent);
		},
		test_ast::PlusExpr(ref term, ref expr) => {
			print!("{}PlusExpr(\n", indent);
			print_term(&*term, ind + 4);
			println!("    {}+", indent);
			print_expr(&*expr, ind + 4);
			print!("{})\n", indent);
		},
		test_ast::MinusExpr(ref term, ref expr) => {
			print!("{}MinusExpr(\n", indent);
			print_term(&*term, ind + 4);
			println!("    {}-", indent);
			print_expr(&*expr, ind + 4);
			print!("{})\n", indent);
		}
	}
}

fn print_term(e: &Box<test_ast::term>, ind: uint) {
	let mut indent = String::from_str("");
	indent.grow(ind, ' ');
	match **e {
		test_ast::FactorAsTerm(ref factor) => {
			print!("{}FactorAsTerm(\n", indent);
			print_factor(factor, ind + 4);
			print!("{})\n", indent);
		},
		test_ast::MultTerm(ref factor, ref term) => {
			print!("{}MultTerm(\n", indent);
			print_factor(factor, ind + 4);
			println!("    {}*", indent);
			print_term(term, ind + 4);
			print!("{})\n", indent);
		},
		test_ast::DivTerm(ref factor, ref term) => {
			print!("{}DivTerm(\n", indent);
			print_factor(&*factor, ind + 4);
			println!("    {}/", indent);
			print_term(&*term, ind + 4);
			print!("{})\n", indent);
		}
	}
}

fn print_factor(e: &Box<test_ast::factor>, ind: uint) {
	let mut indent = String::from_str("");
	indent.grow(ind, ' ');
	match **e {
		test_ast::Id(ref s) => {
			print!("{}Id({})\n", indent, s);
		},
		test_ast::ParenthesizedExpr(ref expr) => {
			print!("{}ParenthesizedExpr(\n", indent);
			print_expr(expr, ind + 4);
			print!("{})\n", indent);
		}
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
