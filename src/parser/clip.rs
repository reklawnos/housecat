use token::{Token, Tok};
use ast::*;
use utils::*;
use parser::stmt::{parse_clip_statements};
use parser::ParseResult;

// <ident-list>
fn parse_ident_list<'a>(tokens: &'a[Tok]) -> ParseResult<'a, Vec<&'a str>> {
    match tokens {
        [Tok{token: Token::Ident(id), ..}, rest..] => {
            match rest {
                // ... ")"
                [Tok{token: Token::CloseParen, ..}, next_rest..] => Ok((vec![id], next_rest)),
                // ... "," <ident-list>
                [Tok{token: Token::Comma, ..}, next_rest..] => {
                            let (mut parsed_list, toks_after_list) = {
                                try!(parse_ident_list( next_rest))
                            };
                            parsed_list.insert(0, id);
                            Ok((parsed_list, toks_after_list))
                }
                [Tok{ref token, line, col, line_string, ..}, ..] => Err(format!(
                    "PARSING FAILURE at {},{}: Expected ')' or ',' but found {:?}\n{}\n{}",
                    line + 1,
                    col + 1,
                    token,
                    line_string,
                    get_caret_string(col)
                )),
                [] => Err(format!("PARSING FAILURE: Reached end of file but expected ')' or ','"))
            }
        }
        [Tok{ref token, line, col, line_string, ..}, ..] => {
            Err(format!(
                "PARSING FAILURE at {},{}: Expected Ident but found {:?}\n{}\n{}",
                line + 1,
                col + 1,
                token,
                line_string,
                get_caret_string(col)
            ))
        }
        [] => Err(format!("PARSING FAILURE: Reached end of file but expected an Ident"))
    }
}

// <params>
fn parse_params<'a>(tokens: &'a[Tok]) -> ParseResult<'a, Vec<&'a str>> {
    match tokens {
        //  ")"
        [Tok{token: Token::CloseParen, ..}, rest..] => Ok((vec![], rest)),
        // <ident-list>
        [Tok{token: _, ..}, ..] => parse_ident_list(tokens),
        _ => Err(format!("PARSING FAILURE: Reached end of file but expected an Ident or ')'"))
    }
}

// <rets>
pub fn parse_rets<'a>(tokens: &'a[Tok]) -> ParseResult<'a, Vec<&'a str>> {
    match tokens {
        // "(" <ident-list>
        [Tok{token: Token::OpenParen, ..}, rest..] => parse_ident_list(rest),
        // <ident>
        [Tok{token: Token::Ident(id), ..}, rest..] => Ok((vec![id], rest)),
        [Tok{ref token, line, col, line_string, ..}, ..] => {
            Err(format!(
                "PARSING FAILURE at {},{}: Expected Ident or '(' but found {:?}\n{}\n{}",
                line + 1,
                col + 1,
                token,
                line_string,
                get_caret_string(col)
            ))
        }
        [] => Err(format!("PARSING FAILURE: Reached end of file but expected an Ident or '('"))
    }
}

// <clip-def>
pub fn parse_clip_def<'a>(tokens: &'a[Tok])
                          -> ParseResult<'a, (Vec<&'a str>, Vec<&'a str>, Vec<Stmt<'a>>)> {
    match tokens {
        // "(" <params> ...
        [Tok{token: Token::OpenParen, line, col, line_string, ..}, rest..] => {
            let (parsed_params, tokens_after_params) = try!(parse_params(rest));
            match tokens_after_params {
                // ... "{" <clip-statements>
                [Tok{token: Token::OpenCurly, ..}, next_rest..] => {
                    let (parsed_list, tokens_after_list) = try!(parse_clip_statements(next_rest));
                    Ok(((parsed_params, vec![], parsed_list), tokens_after_list))
                }
                // ... "->" ...
                [Tok{token: Token::Ret, ..}, next_rest..] => {
                    let (parsed_rets, tokens_after_rets) = try!(parse_rets(next_rest));
                    match tokens_after_rets {
                        // ... "{" <clip-statements>
                        [Tok{token: Token::OpenCurly, ..}, tok_rest..] => {
                            let (parsed_list, tokens_after_list) = {
                                try!(parse_clip_statements(tok_rest))
                            };
                            Ok(((parsed_params, parsed_rets, parsed_list), tokens_after_list))
                        }
                        [Tok{ref token, line, col, line_string, ..}, ..] => Err(format!(
                            "PARSING FAILURE at {},{}: Found {:?} but expected '{{'\n{}\n{}",
                            line + 1,
                            col + 1,
                            token,
                            line_string,
                            get_caret_string(col)
                        )),
                        [] => Err(format!("PARSING FAILURE: Reached end of file, but expected '{{'"))
                    }  
                }
                [Tok{ref token, line, col, line_string, ..}, ..] => Err(format!(
                    "PARSING FAILURE at {},{}: Found {:?} but expected '{{' or '->'\n{}\n{}",
                    line + 1,
                    col + 1,
                    token,
                    line_string,
                    get_caret_string(col)
                )),
                [] => Err(format!(
                    "PARSING FAILURE: Reached end of file, but the paren at {},{} is unmatched\n{}\n{}",
                    line + 1,
                    col + 1,
                    line_string,
                    get_caret_string(col)
                ))
            }
        },
        [Tok{ref token, line, col, line_string, ..}, ..] => Err(format!(
                "PARSING FAILURE at {},{}: Found {:?} but expected '('\n{}\n{}",
                line + 1,
                col + 1,
                token,
                line_string,
                get_caret_string(col)
            )),
        [] => Err(format!("PARSING FAILURE: Reached end of file, but expected '('"))
    }
}
