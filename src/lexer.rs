use token::{Token, Tok};
use regex;

pub enum ParseType {
    PtName, //ident or keyword
    PtBool,
    PtFloat,
    PtInt,
    PtString,
    PtSkip,
    PtColon,
    PtDot,
    PtComma,
    PtOpenBrac,
    PtCloseBrac,
    PtOpenCurly,
    PtCloseCurly,
    PtOpenParen,
    PtCloseParen,
    PtComment,
    PtOperator
}

static TOKEN_SPECS: &'static [(ParseType, regex::Regex)] = &[
    (ParseType::PtBool, regex!(r"^(?:true|false)")),
    (ParseType::PtName, regex!(r"^[:alpha:][:word:]*")),
    (ParseType::PtFloat, regex!(r"^-?[0-9]*\.[0-9]+(?:e[-+]?[0-9]+)?")),
    (ParseType::PtInt, regex!(r"^-?[0-9]+")),
    (ParseType::PtString, regex!("^\"(?:[^\"\\\\]|\\\\.)*\"")),
    (ParseType::PtColon, regex!(r"^:")),
    (ParseType::PtDot, regex!(r"^\.")),
    (ParseType::PtComma, regex!(r"^,")),
    (ParseType::PtOpenBrac, regex!(r"^\[")),
    (ParseType::PtCloseBrac, regex!(r"^\]")),
    (ParseType::PtOpenCurly, regex!(r"^\{")),
    (ParseType::PtCloseCurly, regex!(r"^\}")),
    (ParseType::PtOpenParen, regex!(r"^\(")),
    (ParseType::PtCloseParen, regex!(r"^\)")),
    (ParseType::PtOperator, regex!(r"^(<=|>=|!=|==|!==|\|\||&&|[!^*%+-<>=/])")),
    (ParseType::PtSkip, regex!(r"^\s")),
    (ParseType::PtComment, regex!(r"^#"))
];

pub fn parse_line(line: &String, line_no: usize, token_vec: & mut Vec<Tok>) -> Result<(), usize> {
    let mut line = &line[..];
    let mut col = 0usize;
    while line.len() > 0 {
        let mut found_token = false;
        let mut found_comment = false;
        for &(ref parse_type, ref re) in TOKEN_SPECS.iter() {
            let pos = match re.find(line) {
                Some(range) => range,
                None => continue
            };
            //Skip the rest of the line if we found a comment
            match *parse_type {
                ParseType::PtComment => {
                    found_comment = true;
                    break;
                },
                _ => {}
            }
            let (start,end) = pos;
            let res = &line[start..end];
            //Skip over whitespace
            match *parse_type {
                ParseType::PtSkip => {},
                _ => {
                    let new_token = decide_token(parse_type, res);
                    token_vec.push(Tok{token: new_token, line: line_no, col: col});
                }
            }
            //Push the column index to the end of what we just read
            col += end;
            line = &line[end..];
            found_token = true;
            break;
        }

        if found_comment {
            break;
        }
        //No token was found, which means that something was invalid
        if !found_token {
            return Err(col);
        }
    }
    Ok(())
}

fn decide_token(parse_type: &ParseType, section: &str) -> Token {
    match *parse_type {
        //Capture keywords and idents
        ParseType::PtName => {
            match section {
                "def" => Token::Def,
                "nil" => Token::Nil,
                s => {
                    Token::Ident(Box::new(s.to_string()))
                }
            }
        }
        ParseType::PtBool => Token::Bool(section.parse().unwrap()),
        ParseType::PtFloat => Token::Float(section.parse().unwrap()),
        ParseType::PtInt => Token::Int(section.parse().unwrap()),
        //TODO: add support for escape characters (should have error when there's an invalid char)
        ParseType::PtString => {
            let trimmed_slice = &section[1..section.len() - 1];//section.slice_chars(1, section.len() - 1);
            let escaped = trimmed_slice.replace("\\\"", "\"").replace("\\\\", "\\");
            Token::String(Box::new(escaped))
        },
        ParseType::PtColon => Token::Colon,
        ParseType::PtDot => Token::Dot,
        ParseType::PtComma => Token::Comma,
        ParseType::PtOpenBrac => Token::OpenBrac,
        ParseType::PtCloseBrac => Token::CloseBrac,
        ParseType::PtOpenCurly => Token::OpenCurly,
        ParseType::PtCloseCurly => Token::CloseCurly,
        ParseType::PtOpenParen => Token::OpenParen,
        ParseType::PtCloseParen => Token::CloseParen,
        ParseType::PtOperator => {
            match section {
                "!" => Token::Not,
                "^" => Token::Exp,
                "*" => Token::Mul,
                "/" => Token::Div,
                "%" => Token::Mod,
                "+" => Token::Add,
                "-" => Token::Sub,
                "<" => Token::Lt,
                "<=" => Token::Lte,
                ">" => Token::Gt,
                ">=" => Token::Gte,
                "=" => Token::Eq,
                "!=" => Token::Neq,
                "==" => Token::Same,
                "!==" => Token::Nsame,
                "&&" => Token::And,
                "||" => Token::Or,
                _ => panic!("Unknown binary op")
            }
        }
        _ => panic!("not implemented")
    }
}
