extern crate regex;
use regex::Regex;
use std::io;
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

	// User values
	Ident(Box<String>), // ex. foo, bar
	Int(int),           // ex. 1324, -43
	Float(f64),         // ex. 1.3, -34.432e-4
	// Binary ops
	Add,                // '+'
	Sub,                // '-'
	Mul,                // '*'
	Div,                // '/'
}

fn do_file_parse(path: &Path) {
	let token_specs = [
		("Name", r"[A-z]\w*"),
		("Float", r"-?[0-9]*\.?[0-9]+(?:e[-+]?[0-9]+)?"),
		("Int", r"-?[0-9]+"),
		("Skip", r"\s"),
	];

	let mut re_string = "".to_string();

	//Assemble a regex from our token specs
	for spec in token_specs.iter() {
		match spec {
			&(name, re) => {
				re_string.push_str(format!("(?P<{}>{})|", name, re).as_slice());
			}
		}
	}

	//Get rid of the last '|'
	re_string.pop_char();

	//Generate a new regex
	let re = match Regex::new(re_string.as_slice()) {
		Ok(re) => re,
		Err(err) => fail!("{}", err),
	};

	let mut file = BufferedReader::new(File::open(path));
	let mut result : Vec<Token> = Vec::new();

	for (line_index, l) in file.lines().enumerate() {
		println!("------------lexing line {}", line_index);
		let mut line = l.unwrap();
		let mut next_line = line.clone();
		let mut col = 0u;
		let mut line_offset = 0u;
		while next_line.len() > 0 {
			//TODO: figure out this Rust pointer mumbo jumbo
			line = next_line.clone();
			let line_clone = line.as_slice().clone();
			let caps = re.captures(line_clone).unwrap();
			let mut found_token = false;
			//Both of these iterators are ahead of the specs by 1, due to the 0th general capture
			let mut caps_iter = caps.iter();
			caps_iter.next();
			let mut pos_iter = caps.iter_pos();
			pos_iter.next();
			let mut it = token_specs.iter().zip(caps_iter.zip(pos_iter));
			//Iterate through token types and check for matches
			println!("----------this is one go through")
			for (&(name, _), (res, pos)) in it {
				println!("The result with name '{}' at position {} is: '{}'", name, pos, res);
				if res != "" {
					println!("something was matched! :D");
					let (start,end) = pos.unwrap();
					println!("(start, end): {} {}", start, end);

					//The very first character should be the start of a valid token
					if start != 0 {
						fail!("Lexing failure: unrecognized symbol at line {}, column {}: {}",
							line_index + 1,
							col + 1,
							line.as_slice().char_at(col));
					}
					//Push the column index to the end of what we just read
					col += end;
					next_line = line.as_slice().slice_from(end).to_string();
					found_token = true;
					break;
				}
			}
			if !found_token {
				println!("no token found :(");
				break;
			}
		}
	}
}
