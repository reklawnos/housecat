use token::{Token, Tok};
use ast::*;
use parser::stmt::{parse_clip_statements};
use parser::{ParseResult, ParserError, ParserErrorType};

// <ident-list>
fn parse_ident_list<'a>(tokens: &'a[Tok<'a>]) -> ParseResult<'a, Vec<&'a str>> {
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
                [ref tok, ..] => Err(ParserError{
                    actual: tok.clone(),
                    error_type: ParserErrorType::ExpectedTokens{
                        expected: vec!(Token::CloseParen, Token::Comma)
                    },
                    hint: None
                }),
                [] => panic!("Missing EOF")
            }
        }
        [ref tok, ..] => Err(ParserError{
            actual: tok.clone(),
            error_type: ParserErrorType::ExpectedIdent,
            hint: None
        }),
        [] => panic!("Missing EOF")
    }
}

// <params>
fn parse_params<'a>(tokens: &'a[Tok]) -> ParseResult<'a, Vec<&'a str>> {
    match tokens {
        //  ")"
        [Tok{token: Token::CloseParen, ..}, rest..] => Ok((vec![], rest)),
        // <ident-list>
        [Tok{token: _, ..}, ..] => parse_ident_list(tokens),
        [] => panic!("Missing EOF")
    }
}

// <rets>
pub fn parse_rets<'a>(tokens: &'a[Tok]) -> ParseResult<'a, Vec<&'a str>> {
    match tokens {
        // "(" <ident-list>
        [Tok{token: Token::OpenParen, ..}, rest..] => parse_ident_list(rest),
        // <ident>
        [Tok{token: Token::Ident(id), ..}, rest..] => Ok((vec![id], rest)),
        [ref tok, ..] => Err(ParserError{
            actual: tok.clone(),
            error_type: ParserErrorType::ExpectedRets,
            hint: Some("`->` must be followed by a single ident or a list of idents in the form `(foo, bar, ...)`")
        }),
        [] => panic!("Missing EOF")
    }
}

// <clip-def>
pub fn parse_clip_def<'a>(tokens: &'a[Tok<'a>])
                          -> ParseResult<'a, (Vec<&'a str>, Vec<&'a str>, Vec<Stmt<'a>>)> {
    match tokens {
        // "(" <params> ...
        [Tok{token: Token::OpenParen, ..}, rest..] => {
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
                        [ref tok, ..] => Err(ParserError{
                            actual: tok.clone(),
                            error_type: ParserErrorType::ExpectedTokens{
                                expected: vec!(Token::OpenCurly)
                            },
                            hint: Some("return idents must be followed by a clip block in the form `{...}`")
                        }),
                        [] => panic!("Missing EOF")
                    }
                }
                [ref tok, ..] => Err(ParserError{
                    actual: tok.clone(),
                    error_type: ParserErrorType::ExpectedTokens{
                        expected: vec!(Token::OpenCurly, Token::Ret)
                    },
                    hint: Some("`fn(...)` expressions must include a clip block in the form `{...}`")
                }),
                [] => panic!("Missing EOF")
            }
        },
        [ref tok, ..] => Err(ParserError{
            actual: tok.clone(),
            error_type: ParserErrorType::ExpectedTokens{
                expected: vec!(Token::OpenParen)
            },
            hint: Some("`fn` expressions must include a list of parameters in the form `()` or `(foo, bar, ...)`")
        }),
        [] => panic!("Missing EOF")
    }
}
