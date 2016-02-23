#[macro_use]
mod macros;
mod stmt;
pub mod expr;
mod clip;

use token::{Tok, Token};
use ast::*;
use parser::stmt::parse_base_statements;
use std::fmt;
use utils::get_caret_string;

enum ParserErrorType<'a> {
    ExpectedTokens {
        expected: Vec<Token<'a>>
    },
    ExpectedMatchingToken {
        expected: Token<'a>,
        start_tok: Tok<'a>
    },
    ExpectedExpression,
    ExpectedStatement
}

struct ParserError<'a> {
    actual: Tok<'a>,
    error_type: ParserErrorType<'a>,
    hint: Option<&'static str>
}

impl<'a> fmt::Display for ParserError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "PARSING FAILURE at {}:{} ", self.actual.line + 1, self.actual.col + 1));
        match self.error_type {
            ParserErrorType::ExpectedTokens {ref expected} => {
                let len = expected.len();
                let mut sum_string = "".to_string();
                for (i, t) in expected.iter().enumerate() {
                    if len == 2 && i == 1 {
                        sum_string.push_str(" or ");
                    } else if len > 2 && i < len - 1 && i > 0 {
                        sum_string.push_str(", ");
                    } else if len > 2 && i == len - 1 {
                        sum_string.push_str(", or ");
                    }
                    sum_string = sum_string + &format!("`{}`", t);
                }
                try!(write!(f, "expected {} but found `{}`", sum_string, self.actual.token));
            }
            ParserErrorType::ExpectedMatchingToken {ref expected, ref start_tok} => {
                try!(write!(
                    f,
                    "must match `{}` at {}:{} with `{}` but found `{}`",
                    start_tok.token,
                    start_tok.line + 1,
                    start_tok.col + 1,
                    expected,
                    self.actual.token
                ));
            }
            ParserErrorType::ExpectedExpression => {
                try!(write!(f, "expected expression but found `{}`", self.actual.token));
            }
            ParserErrorType::ExpectedStatement => {
                try!(write!(f, "expected statement but found `{}`", self.actual.token));
            }
        }
        let line_as_string = (self.actual.line + 1).to_string();
        try!(write!(f, "\n{}: {}", line_as_string, self.actual.line_string));
        let ret = write!(f, "\n{}", get_caret_string(self.actual.col + line_as_string.len() + 2));
        if let Some(hint) = self.hint {
            return write!(f, "\n\nHint: {}\n", hint);
        }
        return ret;
    }
}

pub type ParseResult<'a, T> = Result<(T, &'a[Tok<'a>]), ParserError<'a>>;

#[allow(dead_code)]
fn print_toks<'a>(func: &str, tokens: &'a[Tok]) {
    print!("{}: ", func);
    for t in tokens.iter() {
        print!("{:?}, ", t.token);
    }
    println!("");
}

pub fn parse_tokens<'a>(tokens: &'a[Tok], cur_statements: &'a mut Vec<Stmt<'a>>)
                        -> Result<&'a Vec<Stmt<'a>>, String> {
    match parse_base_statements(tokens, cur_statements) {
        Ok((v, _)) => Ok(v),
        Err(s) => Err(s.to_string())
    }
}

#[cfg(test)]
mod test {
    use super::{ParserError, ParserErrorType};
    use token::{Token, Tok};

    #[test]
    fn test_display_expected_tokens_error() {
        let example = format!("{}", ParserError {
            actual: Tok{
                token: Token::Ident("bagelman"),
                line: 0,
                col: 9,
                line_string: "what's a bagelman?",
                char_index: 9
            },
            error_type: ParserErrorType::ExpectedTokens {
                expected: vec!(Token::Sub),
            }
        });
        assert_eq!(example, "PARSING FAILURE at 1:10 expected `-` but found `bagelman`".to_string());
        let example = format!("{}", ParserError {
            actual: Tok{
                token: Token::Ident("bagelman"),
                line: 0,
                col: 9,
                line_string: "what's a bagelman?",
                char_index: 9
            },
            error_type: ParserErrorType::ExpectedTokens {
                expected: vec!(Token::Sub, Token::Add)
            }
        });
        assert_eq!(example, "PARSING FAILURE at 1:10 expected `-` or `+` but found `bagelman`".to_string());
        let example = format!("{}", ParserError {
            actual: Tok{
                token: Token::Ident("bagelman"),
                line: 0,
                col: 9,
                line_string: "what's a bagelman?",
                char_index: 9
            },
            error_type: ParserErrorType::ExpectedTokens {
                expected: vec!(Token::Sub, Token::Add, Token::Mul),
            }
        });
        assert_eq!(example, "PARSING FAILURE at 1:10 expected `-`, `+`, or `*` but found `bagelman`".to_string());
    }

    #[test]
    fn test_display_expected_matching_token_error() {
        let example = format!("{}", ParserError {
            actual: Tok{
                token: Token::Ident("bagelman"),
                line: 2,
                col: 9,
                line_string: "what's a bagelman?",
                char_index: 9
            },
            error_type: ParserErrorType::ExpectedMatchingToken {
                expected: Token::CloseBrac,
                start_tok: Tok{
                    token: Token::OpenBrac,
                    line: 1,
                    col: 10,
                    line_string: "open brac [",
                    char_index: 10
                }
            }
        });
        assert_eq!(example, "PARSING FAILURE at 3:10 expected `]` to match `[` at 1:10 but found `bagelman`".to_string());
    }
}
