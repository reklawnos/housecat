use token;

pub enum ParseType {
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

pub enum LineResult {
    Ok,
    Err(uint)
}

pub fn parse_line(line: &String, line_no: uint, token_vec: & mut Vec<(token::Token, uint, uint)>) -> LineResult {
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
    let mut line = line.clone();
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
                    token_vec.push((new_token, line_no, col));
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
            return Err(col);
        }
    }
    Ok
}

fn decide_token(parse_type: ParseType, section: &str) -> token::Token {
    match parse_type {
        PtName => {
            match section {
                "def" => token::Def,
                s => {
                    token::Ident(box s.to_string())
                }
            }
        }
        PtBool => token::Bool(from_str(section).unwrap()),
        PtFloat => token::Float(from_str(section).unwrap()),
        PtInt => token::Int(from_str(section).unwrap()),
        //TODO: add support for escape characters
        PtString => {
            let mut ns = section.to_string().clone();
            //remove start and end character
            ns.pop_char();
            ns.shift_char();
            token::String(box ns)
        },
        PtColon => token::Colon,
        PtDot => token::Dot,
        PtOpenBrac => token::OpenBrac,
        PtCloseBrac => token::CloseBrac,
        PtOpenParen => token::OpenParen,
        PtCloseParen => token::CloseParen,
        PtBinOp => {
            match section {
                "+" => token::Add,
                "-" => token::Sub,
                "*" => token::Mul,
                "/" => token::Div,
                _ => fail!("Unknown binary op")
            }
        }
        _ => fail!("not implemented")
    }
}
