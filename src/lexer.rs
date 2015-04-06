use token::{Token, Tok};
use regex::Regex;
use utils::get_caret_string;

static COMMENT_REGEX: Regex = regex!(r"^#.*");
static WHITESPACE_REGEX: Regex = regex!(r"^\s");

static SYMBOL_SPECS: &'static [(Regex, Token<'static>)] = &[
    //Symbols
    (regex!(r"^:"), Token::Assign),
    (regex!(r"^\."), Token::Dot),
    (regex!(r"^\{"), Token::OpenCurly),
    (regex!(r"^\}"), Token::CloseCurly),
    (regex!(r"^\["), Token::OpenBrac),
    (regex!(r"^\]"), Token::CloseBrac),
    (regex!(r"^\("), Token::OpenParen),
    (regex!(r"^\)"), Token::CloseParen),
    (regex!(r"^,"), Token::Comma),
    (regex!(r"^->"), Token::Ret),
    //Binary Operators
    (regex!(r"^\*"), Token::Mul),
    (regex!(r"^/"), Token::Div),
    (regex!(r"^%"), Token::Mod),
    (regex!(r"^\+"), Token::Add),
    (regex!(r"^-"), Token::Sub),
    (regex!(r"^<="), Token::Lte),
    (regex!(r"^<"), Token::Lt),
    (regex!(r"^>="), Token::Gte),
    (regex!(r"^>"), Token::Gt),
    (regex!(r"^=="), Token::Same),
    (regex!(r"^="), Token::Eq),
    (regex!(r"^!=="), Token::Nsame),
    (regex!(r"^!="), Token::Neq),
    (regex!(r"^&&"), Token::And),
    (regex!(r"^\|\|"), Token::Or),
    //Unary Operators
    (regex!(r"^!"), Token::Not),
    (regex!(r"^\$"), Token::Get),
];

pub struct Lexer<'a> {
    input: String,
    toks: Vec<Tok<'a>>,
    lit_regexes: [(Regex, Box<Fn(&'a str) -> Token<'a> + 'static>); 4]
}

impl<'a> Lexer<'a> {

    pub fn new() -> Lexer<'a> {
        Lexer {
            input: String::new(),
            toks: Vec::new(),
            //TODO: There's gotta be a better way to do this
            lit_regexes: [
                (regex!(r"^[0-9]*\.[0-9]+(?:e[-+]?[0-9]+)?"), Box::new(|s: &'a str| Token::Float(s.parse().unwrap()))),
                (regex!(r"^[0-9]+"), Box::new(|s: &'a str| Token::Int(s.parse().unwrap()))),
                (regex!(r"^([A-Za-z_][0-9A-Za-z_]*)"), Box::new(|s: &'a str| {
                    //Match keywords and idents
                    match s {
                        "def" => Token::Def,
                        "var" => Token::Var,
                        "nil" => Token::Nil,
                        "fn" => Token::Fn,
                        "return" => Token::Return,
                        "in" => Token::In,
                        "if" => Token::If,
                        "else" => Token::Else,
                        "elif" => Token::Elif,
                        "while" => Token::While,
                        "end" => Token::End,
                        "true" => Token::Bool(true),
                        "false" => Token::Bool(false),
                        s => Token::Ident(s)
                    }
                })),
                (regex!(r#"^"(?:[^"\\]|\\.)*""#), Box::new(|s: &'a str|{
                    let trimmed_slice = &s[1..s.len() - 1];
                    let escaped = trimmed_slice.replace(r#"\""#, "\"").replace(r"\\", r"\");
                    Token::String(escaped)
                }))
            ]
        }
    }

    pub fn lex(&'a mut self, s: String) -> Result<&Vec<Tok<'a>>, String> {
        let mut char_index = 0usize;
        self.input = s;
        for (line_index, l) in self.input.lines().enumerate() {
            let res = Lexer::lex_line(l, line_index, &mut char_index, &mut self.toks, &self.lit_regexes);
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

    fn lex_line(line: &'a str, line_no: usize, char_index: &mut usize, toks: &mut Vec<Tok<'a>>, lit_regexes: &[(Regex, Box<Fn(&'a str) -> Token<'a> + 'static>)]) -> Result<(), usize> {
        let mut line_slice = line;
        let mut col = 0usize;
        let mut match_end = 0usize;
        while line_slice.len() > 0 {
            let mut found_token = false;
            //Return for this line once a comment is reached
            match COMMENT_REGEX.find(line_slice) {
                Some((_, end)) => {
                    match_end = end;
                    found_token = true;
                }
                None => ()
            }
            //Skip whitespace
            match WHITESPACE_REGEX.find(line_slice) {
                Some((_, end)) => {
                    match_end = end;
                    found_token = true;
                }
                None => ()
            };
            if !found_token {
                for &(ref re, ref tok_type) in SYMBOL_SPECS.iter() {
                    let (_,end) = match re.find(line_slice) {
                        Some(range) => range,
                        None => continue
                    };
                    match_end = end;
                    let new_token = (*tok_type).clone();
                    toks.push(Tok{token: new_token, line: line_no, col: col, line_string: line, char_index: *char_index});
                    found_token = true;
                    break;
                }
            }
            if !found_token {
                for &(ref re, ref tok_func) in lit_regexes.iter() {
                    let (start,end) = match re.find(line_slice) {
                        Some(range) => range,
                        None => continue
                    };
                    match_end = end;
                    let token_slice = &line_slice[start..end];
                    let new_token = tok_func(token_slice);
                    toks.push(Tok{token: new_token, line: line_no, col: col, line_string: line, char_index: *char_index});
                    found_token = true;
                    break;
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
    use test::Bencher;

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
            "def var nil return",
            vec![Token::Def, Token::Var, Token::Nil, Token::Return]
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
                Token::Eq,
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
        match lexer.lex("this is @ invalid".to_string()){
            Err(s) => assert_eq!("LEXING FAILURE at 1,9: invalid character @\nthis is @ invalid\n        ^".to_string(), s),
            _ => assert!(false)
        }
    }

    #[test]
    fn test_char_data() {
        let mut lexer = Lexer::new();
        match lexer.lex("if 3 > 2\n    # do some stuff\n    stuff()\nend".to_string()){
            Err(s) => assert!(false),
            Ok(v) => {
                assert_eq!("    stuff()", v[5].line_string);
                assert_eq!(9, v[5].col);
                assert_eq!(2, v[5].line);
                assert_eq!(38, v[5].char_index);
            }
        }
    }

    #[bench]
    fn bench_symbols(b: &mut Bencher) {
        let num_copies = 1000;
        let symbol_string = ":.{}()[],->*/%+-<=<>=>===!==!=&&||!$";
        let mut test_string = String::with_capacity(symbol_string.len() * num_copies);
        for i in 1..num_copies {
            test_string.push_str(symbol_string);
        }
        b.iter(|| {
            let mut lexer = Lexer::new();
            lexer.lex(test_string.clone());
        });
    }
}
