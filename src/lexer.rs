use token::{Token, Tok};
use regex::Regex;
use utils::get_caret_string;

static COMMENT_REGEX: Regex = regex!(r"^#");
static WHITESPACE_REGEX: Regex = regex!(r"^\s");

static SYMBOL_SPECS: &'static [(Regex, Token<'static>)] = &[
    //Keywords
    (regex!(r"^def"), Token::Def),
    (regex!(r"^var"), Token::Var),
    (regex!(r"^nil"), Token::Nil),
    (regex!(r"^fn"), Token::Fn),
    (regex!(r"^return"), Token::Return),
    (regex!(r"^in"), Token::In),
    (regex!(r"^if"), Token::If),
    (regex!(r"^else"), Token::Else),
    (regex!(r"^elif"), Token::Elif),
    (regex!(r"^while"), Token::While),
    (regex!(r"^end"), Token::End),
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
    (regex!(r"^<"), Token::Lt),
    (regex!(r"^<="), Token::Lte),
    (regex!(r"^>"), Token::Gt),
    (regex!(r"^>="), Token::Gte),
    (regex!(r"^="), Token::Eq),
    (regex!(r"^!="), Token::Neq),
    (regex!(r"^=="), Token::Same),
    (regex!(r"^!=="), Token::Nsame),
    (regex!(r"^&&"), Token::And),
    (regex!(r"^\|\|"), Token::Or),
    //Unary Operators
    (regex!(r"^!"), Token::Not),
    (regex!(r"^\$"), Token::Get),
];

pub struct Lexer<'a> {
    input: String,
    toks: Vec<Tok<'a>>,
    lit_regexes: [(Regex, Box<Fn(&'a str) -> Token<'a> + 'static>); 5]
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
                (regex!(r"^(?:true|false)"), Box::new(|s: &'a str| Token::Bool(s.parse().unwrap()))),
                (regex!(r"^([A-Za-z_][0-9A-Za-z_]*)"), Box::new(|s: &'a str| Token::Ident(s))),
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
                Ok(()) => {},
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

    fn lex_line(line: &'a str, line_no: usize, char_index: &mut usize, toks: &mut Vec<Tok<'a>>, lit_regexes: &[(Regex, Box<Fn(&'a str) -> Token<'a> + 'static>); 5]) -> Result<(), usize> {
        let mut line_slice = line;
        let mut col = 0usize;
        let mut match_end = 0usize;
        while line_slice.len() > 0 {
            let mut found_token = false;
            //Return for this line once a comment is reached
            if COMMENT_REGEX.is_match(line_slice) {
                break;
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
