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
    PtOperator,
    PtRet
}

static TOKEN_SPECS: &'static [(ParseType, regex::Regex)] = &[
    (ParseType::PtBool, regex!(r"^(?:true|false)")),
    (ParseType::PtName, regex!(r"^[:alpha:][:word:]*")),
    (ParseType::PtRet, regex!(r"^->")),
    (ParseType::PtFloat, regex!(r"^-?[0-9]*\.[0-9]+(?:e[-+]?[0-9]+)?")),
    (ParseType::PtInt, regex!(r"^-?[0-9]+")),
    (ParseType::PtString, regex!(r#"^"(?:[^"\\]|\\.)*""#)),
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

pub fn parse_line<'a>(line: &'a String, line_no: usize, token_vec: & mut Vec<Tok<'a>>) -> Result<(), usize> {
    let mut line_slice = &line[..];
    let mut col = 0usize;
    while line_slice.len() > 0 {
        let mut found_token = false;
        let mut found_comment = false;
        for &(ref parse_type, ref re) in TOKEN_SPECS.iter() {
            let pos = match re.find(line_slice) {
                Some(range) => range,
                None => continue
            };
            //Skip the rest of the line_slice if we found a comment
            match *parse_type {
                ParseType::PtComment => {
                    found_comment = true;
                    break;
                },
                _ => {}
            }
            let (start,end) = pos;
            let res = &line_slice[start..end];
            //Skip over whitespace
            match *parse_type {
                ParseType::PtSkip => {},
                _ => {
                    let new_token = decide_token(parse_type, res);
                    token_vec.push(Tok{token: new_token, line: line_no, col: col, line_string: line});
                }
            }
            //Push the column index to the end of what we just read
            col += end;
            line_slice = &line_slice[end..];
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

fn decide_token(parse_type: &ParseType, tok_string: &str) -> Token {
    match *parse_type {
        //Capture keywords and idents
        ParseType::PtName => {
            match tok_string {
                "def" => Token::Def,
                "var" => Token::Var,
                "nil" => Token::Nil,
                "fn" => Token::Fn,
                "in" => Token::In,
                "return" => Token::Return,
                s => {
                    Token::Ident(Box::new(s.to_string()))
                }
            }
        }
        ParseType::PtBool => Token::Bool(tok_string.parse().unwrap()),
        ParseType::PtFloat => Token::Float(tok_string.parse().unwrap()),
        ParseType::PtInt => Token::Int(tok_string.parse().unwrap()),
        //TODO: add support for escape characters (should have error when there's an invalid char)
        ParseType::PtString => {
            let trimmed_slice = &tok_string[1..tok_string.len() - 1];
            let escaped = trimmed_slice.replace(r#"\""#, "\"").replace(r"\\", r"\");
            Token::String(Box::new(escaped))
        },
        ParseType::PtColon => Token::Assign,
        ParseType::PtDot => Token::Dot,
        ParseType::PtComma => Token::Comma,
        ParseType::PtOpenBrac => Token::OpenBrac,
        ParseType::PtCloseBrac => Token::CloseBrac,
        ParseType::PtOpenCurly => Token::OpenCurly,
        ParseType::PtCloseCurly => Token::CloseCurly,
        ParseType::PtOpenParen => Token::OpenParen,
        ParseType::PtCloseParen => Token::CloseParen,
        ParseType::PtRet => Token::Ret,
        ParseType::PtOperator => {
            match tok_string {
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
