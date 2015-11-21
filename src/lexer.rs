use token::{Token, Tok};
use regex::Regex;
use utils::get_caret_string;


static SYMBOL_SPECS: &'static [(&'static str, Token<'static>)] = &[
    //Symbols
    (r":", Token::Def),
    (r".", Token::Access),
    (r"|", Token::AccessSelf),
    (r"@", Token::ExprDef),
    (r"{", Token::OpenCurly),
    (r"}", Token::CloseCurly),
    (r"[", Token::OpenBrac),
    (r"]", Token::CloseBrac),
    (r"(", Token::OpenParen),
    (r")", Token::CloseParen),
    (r",", Token::Comma),
    (r"->", Token::Ret),
    //Binary Operators
    (r"^", Token::Exp),
    (r"*", Token::Mul),
    (r"/", Token::Div),
    (r"%", Token::Mod),
    (r"+", Token::Add),
    (r"-", Token::Sub),
    (r"<=", Token::Lte),
    (r"<", Token::Lt),
    (r">=", Token::Gte),
    (r">", Token::Gt),
    (r"==", Token::Eq),
    (r"!=", Token::Neq),
    (r"&&", Token::And),
    (r"||", Token::Or),
    //Unary Operators
    (r"!", Token::Not),
    (r"$", Token::Get),

    (r"=", Token::Assign)
];

pub struct Lexer<'a> {
    input: String,
    toks: Vec<Tok<'a>>,
}

fn match_float(line_slice: &str) -> Option<(Token, usize)> {
    let re = Regex::new(r"^[0-9]*\.[0-9]+(?:e[-+]?[0-9]+)?").unwrap();
    let (start,end) = match re.find(line_slice) {
        Some(range) => range,
        None => {return None}
    };
    let token_slice = &line_slice[start..end];
    return Some((Token::Float(token_slice.parse().unwrap()), end));
}

fn match_int(line_slice: &str) -> Option<(Token, usize)> {
    let re = Regex::new(r"^[0-9]+").unwrap();
    let (start,end) = match re.find(line_slice) {
        Some(range) => range,
        None => {return None}
    };
    let token_slice = &line_slice[start..end];
    return Some((Token::Int(token_slice.parse().unwrap()), end));
}

fn match_keyword(line_slice: &str) -> Option<(Token, usize)> {
    let re = Regex::new(r"^((\p{Alphabetic}|\p{M}|\p{Pc}|\p{Join_Control})\w*)").unwrap();
    let (start,end) = match re.find(line_slice) {
        Some(range) => range,
        None => {return None}
    };
    let token_slice = &line_slice[start..end];
    let tok = match token_slice {
        "var" => Token::Var,
        "let" => Token::Let,
        "nil" => Token::Nil,
        "fn" => Token::Fn,
        "return" => Token::Return,
        "in" => Token::In,
        "if" => Token::If,
        "else" => Token::Else,
        "elif" => Token::Elif,
        "while" => Token::While,
        "for" => Token::For,
        "end" => Token::End,
        "true" => Token::Bool(true),
        "false" => Token::Bool(false),
        "do" => Token::Do,
        s => Token::Ident(s)
    };
    return Some((tok, end));
}

fn match_string(line_slice: &str) -> Option<(Token, usize)> {
    let re = Regex::new(r#"^"(?:[^"\\]|\\.)*""#).unwrap();
    let (start,end) = match re.find(line_slice) {
        Some(range) => range,
        None => {return None}
    };
    let trimmed_slice = &line_slice[start + 1..end - 1];
    println!("trimmed slice: {:?}", trimmed_slice);
    let escaped = trimmed_slice.replace(r#"\""#, "\"").replace(r"\\", r"\");
    return Some((Token::String(escaped), end));
}

impl<'a> Lexer<'a> {
    pub fn new() -> Lexer<'a> {
        Lexer {
            input: String::new(),
            toks: Vec::new(),
        }
    }

    pub fn lex(&'a mut self, s: String) -> Result<&Vec<Tok<'a>>, String> {
        let mut char_index = 0usize;
        self.input = s;
        for (line_index, l) in self.input.lines().enumerate() {
            let res = Lexer::lex_line(l, line_index, &mut char_index, &mut self.toks);
            match res {
                Ok(()) => {char_index += 1;},
                Err(col) => {
                    return Err(
                        format!(
                            "LEXING FAILURE at {},{}: invalid character {}\n{}\n{}",
                            line_index + 1,
                            col + 1,
                            l.chars().nth(col).unwrap(),
                            l,
                            get_caret_string(col)
                        )
                    );
                }
            }
        }
        Ok(&self.toks)
    }

    fn lex_line(line: &'a str, line_no: usize, char_index: &mut usize, toks: &mut Vec<Tok<'a>>) -> Result<(), usize> {
        let mut line_slice = line;
        let mut col = 0usize;
        let mut match_end = 0usize;
        while line_slice.len() > 0 {
            let mut found_token = false;
            //Return for this line once a comment is reached
            let comment_regex = Regex::new(r"^#.*").unwrap();
            let whitespace_regex = Regex::new(r"^\s").unwrap();
            match comment_regex.find(line_slice) {
                Some((_, end)) => {
                    match_end = end;
                    found_token = true;
                }
                None => ()
            }
            //Skip whitespace
            match whitespace_regex.find(line_slice) {
                Some((_, end)) => {
                    match_end = end;
                    found_token = true;
                }
                None => ()
            };
            //Lex symbols
            if !found_token {
                for &(s, ref tok_type) in SYMBOL_SPECS.iter() {
                    let mut got_match = true;
                    if s.len() > line_slice.len() {
                        continue;
                    }
                    for (s_char, line_char) in s.chars().zip(line_slice.chars()) {
                        if s_char != line_char {
                            got_match = false;
                            break;
                        }
                    }
                    if got_match {
                        match_end = s.len();
                        let new_token = (*tok_type).clone();
                        toks.push(Tok{token: new_token, line: line_no, col: col, line_string: line, char_index: *char_index});
                        found_token = true;
                        break;
                    }
                }
            }
            //Lex regexes
            if !found_token {
                let mut funcs: Vec<Box<Fn(&str) -> Option<(Token, usize)>>> = vec![
                    Box::new(match_float),
                    Box::new(match_int),
                    Box::new(match_keyword),
                    Box::new(match_string)
                ];
                for f in funcs.iter_mut() {
                    match f(line_slice) {
                        Some((new_token, end)) => {
                            match_end = end;
                            toks.push(Tok{token: new_token, line: line_no, col: col, line_string: line, char_index: *char_index});
                            found_token = true;
                            break;
                        }
                        None => {}
                    }
                }
            }

            //No token was found, which means that something was invalid
            if !found_token {
                return Err(col);
            }

            //Push the column index to the end of what we just read
            col += match_end;
            *char_index += match_end;
            line_slice = &line_slice[match_end..];
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::Lexer;
    use token::Token;

    fn match_tokens(input: &str, output: Vec<Token>) {
        let mut lexer = Lexer::new();
        let results = lexer.lex(input.to_string()).ok().unwrap();
        for (res, test) in results.iter().zip(output.iter()) {
            assert_eq!(test, &res.token);
        }
    }

    #[test]
    fn test_keywords() {
        match_tokens(
            "while var nil return let",
            vec![Token::While, Token::Var, Token::Nil, Token::Return, Token::Let]
        );
    }

    #[test]
    fn test_ambiguous() {
        match_tokens(
            "-->> ->-> >== <<= !=",
            vec![
                Token::Sub,
                Token::Ret,
                Token::Gt,
                Token::Ret,
                Token::Ret,
                Token::Gte,
                Token::Assign,
                Token::Lt,
                Token::Lte,
                Token::Neq
            ]
        );
    }

    #[test]
    fn test_ident() {
        match_tokens(
            "else elser fn() nils dowhile returns defing tootrue falsehood",
            vec![
                Token::Else,
                Token::Ident("elser"),
                Token::Fn,
                Token::OpenParen,
                Token::CloseParen,
                Token::Ident("nils"),
                Token::Ident("dowhile"),
                Token::Ident("returns"),
                Token::Ident("defing"),
                Token::Ident("tootrue"),
                Token::Ident("falsehood")
            ]
        )
    }

    #[test]
    fn test_invalid_char() {
        let mut lexer = Lexer::new();
        match lexer.lex("this is & invalid".to_string()){
            Err(s) => assert_eq!("LEXING FAILURE at 1,9: invalid character &\nthis is & invalid\n        ^".to_string(), s),
            _ => assert!(false)
        }
    }

    #[test]
    fn test_char_data() {
        let mut lexer = Lexer::new();
        match lexer.lex("if 3 > 2\n    # do some stuff\n    stuff()\nend".to_string()){
            Err(_) => assert!(false),
            Ok(v) => {
                assert_eq!("    stuff()", v[5].line_string);
                assert_eq!(9, v[5].col);
                assert_eq!(2, v[5].line);
                assert_eq!(38, v[5].char_index);
            }
        }
    }

    #[test]
    fn test_unicode_space() {
        let mut lexer = Lexer::new();
        match lexer.lex("def x: ${  \n    def a: 1\n    def b: 2\n}".to_string()){
            Err(_) => assert!(false),
            Ok(v) => {
                assert_eq!(Token::Get, v[3].token);
                assert_eq!(Token::OpenCurly, v[4].token);
            }
        }
    }

    #[test]
    fn test_idents() {
        match_tokens(
            "_i1 i2_a ΔᎠβ_ⅠᏴγδⅡ",
            vec![
                Token::Ident("_i1"),
                Token::Ident("i2_a"),
                Token::Ident("ΔᎠβ_ⅠᏴγδⅡ")
            ]
        )
    }

    #[test]
    fn test_starting_with_number() {
        match_tokens(
            "3_i1 i2_a",
            vec![
                Token::Int(3),
                Token::Ident("_i1"),
                Token::Ident("i2_a"),
            ]
        )
    }

    #[test]
    fn test_comment() {
        let test_string = "if else #valid as is 3 + 3\n2 + 2 #comments #in #comments\n 5 + 5";
        match_tokens(
            test_string,
            vec![
                Token::If,
                Token::Else,
                Token::Int(2),
                Token::Add,
                Token::Int(2),
                Token::Int(5),
                Token::Add,
                Token::Int(5)
            ]
        )
    }

    #[test]
    fn test_strings() {
        match_tokens(
            "var a = \"hello there kind sir\"\nvar b = \"how are you \\\"doing\\\" today?\"",
            vec![
                Token::Var,
                Token::Ident("a"),
                Token::Assign,
                Token::String("hello there kind sir".to_string()),
                Token::Var,
                Token::Ident("b"),
                Token::Assign,
                Token::String("how are you \"doing\" today?".to_string())
            ]
        )
    }

    /*
    #[bench]
    fn bench_symbols(b: &mut Bencher) {
        let num_copies = 1000;
        let symbol_string = ":.{}()[],->*%/+-<=<>=>===!==!=&&||!$";
        let mut test_string = String::with_capacity(symbol_string.len() * num_copies);
        for _ in 1..num_copies {
            test_string.push_str(symbol_string);
        }
        b.iter(|| {
            let mut lexer = Lexer::new();
            match lexer.lex(test_string.clone()) {
                Err(_) => assert!(false),
                Ok(_) => ()
            }
        });
    }
    */
}
