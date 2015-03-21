use token::{Token, Tok};
use ast::*;
use utils::*;
use parser::stmt::{parse_clip_statements};
use parser::Result;

// <ident-list>
fn parse_ident_list<'a>(tokens: &'a[Tok]) -> Result<'a, Vec<&'a str>> {
    match tokens {
        [ref first_tok, rest..] => {
            match first_tok.token {
                // <ident> ...
                Token::Ident(id) => {
                    match rest {
                        [ref next_tok, next_rest..] => {
                            match next_tok.token {
                                // ... ")"
                                Token::CloseParen => Result::Ok(vec![id], next_rest),
                                // ... "," <ident-list>
                                Token::Comma => {
                                    let (mut parsed_list, tokens_after_list) = get_parsed!(parse_ident_list(next_rest));
                                    parsed_list.insert(0, id);
                                    Result::Ok(parsed_list, tokens_after_list)
                                }
                                _ => Result::Err(format!(
                                    "PARSING FAILURE at {},{}: Expected ')' or ',' but found {:?}\n{}\n{}",
                                    next_tok.line + 1,
                                    next_tok.col + 1,
                                    next_tok.token,
                                    next_tok.line_string,
                                    get_caret_string(next_tok.col)
                                ))
                            }
                        }
                        [] => Result::Err(format!("PARSING FAILURE: Reached end of file but expected ')' or ','"))
                    }
                }
                _ => Result::Err(format!(
                                    "PARSING FAILURE at {},{}: Expected Ident but found {:?}\n{}\n{}",
                                    first_tok.line + 1,
                                    first_tok.col + 1,
                                    first_tok.token,
                                    first_tok.line_string,
                                    get_caret_string(first_tok.col)
                                ))
            }
        }
        _ => Result::Err(format!("PARSING FAILURE: Reached end of file but expected an Ident"))
    }
}

// <params>
fn parse_params<'a>(tokens: &'a[Tok]) -> Result<'a, Vec<&'a str>> {
    match tokens {
        [ref first_tok, rest..] => {
            match first_tok.token {
                //  ")"
                Token::CloseParen => Result::Ok(vec![], rest),
                // <ident-list>
                _ => parse_ident_list(tokens)
            }
        }
        _ => Result::Err(format!("PARSING FAILURE: Reached end of file but expected an Ident or ')'"))
    }
}

// <rets>
fn parse_rets<'a>(tokens: &'a[Tok]) -> Result<'a, Vec<&'a str>> {
    match tokens {
        [ref first_tok, rest..] => {
            match first_tok.token {
                // "(" <ident-list>
                Token::OpenParen => parse_ident_list(rest),
                // <ident>
                Token::Ident(id) => Result::Ok(vec![id], rest),
                _ => Result::Err(format!(
                                    "PARSING FAILURE at {},{}: Expected Ident or '(' but found {:?}\n{}\n{}",
                                    first_tok.line + 1,
                                    first_tok.col + 1,
                                    first_tok.token,
                                    first_tok.line_string,
                                    get_caret_string(first_tok.col)
                                ))
            }
        }
        _ => Result::Err(format!("PARSING FAILURE: Reached end of file but expected an Ident or '('"))
    }
}

// <clip-def>
pub fn parse_clip_def<'a>(tokens: &'a[Tok]) -> Result<'a, (Vec<&'a str>, Vec<&'a str>, Vec<Stmt<'a>>)> {
    match tokens {
        [ref first_tok, rest..] => {
            match first_tok.token {
                // "(" <params> ...
                Token::OpenParen => {
                    let (parsed_params, tokens_after_params) = get_parsed!(parse_params(rest));
                    match tokens_after_params {
                        [ref next_tok, next_rest..] => {
                            match next_tok.token {
                                // ... "{" <clip-statements>
                                Token::OpenCurly => {
                                    let (parsed_list, tokens_after_list) = get_parsed!(parse_clip_statements(next_rest));
                                    Result::Ok((parsed_params, vec![], parsed_list), tokens_after_list)
                                }
                                // ... "->" ...
                                Token::Ret => {
                                    let (parsed_rets, tokens_after_rets) = get_parsed!(parse_rets(next_rest));
                                    match tokens_after_rets {
                                        [ref brac_tok, tok_rest..] => {
                                            match brac_tok.token {
                                                Token::OpenCurly => {
                                                    let (parsed_list, tokens_after_list) = get_parsed!(parse_clip_statements(tok_rest));
                                                    Result::Ok((parsed_params, parsed_rets, parsed_list), tokens_after_list)
                                                }
                                                _ => Result::Err(format!(
                                                                    "PARSING FAILURE at {},{}: Found {:?} but expected '{{'\n{}\n{}",
                                                                    brac_tok.line + 1,
                                                                    brac_tok.col + 1,
                                                                    brac_tok.token,
                                                                    brac_tok.line_string,
                                                                    get_caret_string(brac_tok.col)
                                                                ))  
                                            }
                                            
                                        }
                                        _ => Result::Err(format!("PARSING FAILURE: Reached end of file, but expected '{{'"))
                                    }  
                                }
                                _ => Result::Err(format!(
                                                    "PARSING FAILURE at {},{}: Found {:?} but expected '{{' or '->'\n{}\n{}",
                                                    next_tok.line + 1,
                                                    next_tok.col + 1,
                                                    next_tok.token,
                                                    next_tok.line_string,
                                                    get_caret_string(next_tok.col)
                                                ))
                            }
                        }
                        _ => Result::Err(format!(
                            "PARSING FAILURE: Reached end of file, but the paren at {},{} is unmatched\n{}\n{}",
                            first_tok.line + 1,
                            first_tok.col + 1,
                            first_tok.line_string,
                            get_caret_string(first_tok.col)
                        ))
                    }
                },
                _ => Result::Err(format!(
                        "PARSING FAILURE at {},{}: Found {:?} but expected '('\n{}\n{}",
                        first_tok.line + 1,
                        first_tok.col + 1,
                        first_tok.token,
                        first_tok.line_string,
                        get_caret_string(first_tok.col)
                    ))
            }
        }
        _ => Result::Err(format!("PARSING FAILURE: Reached end of file, but expected '('"))
    }
}
