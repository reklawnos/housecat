use token::{Token, Tok};
use ast::*;
use utils::*;
use parser::stmt::{parse_clip_statements};
use parser::clip::{parse_clip_def};
use parser::Result;

// <primary-expr>
fn parse_primary_expr<'a>(tokens: &'a[Tok]) -> Result<'a, Expr<'a>> {
    match tokens {
        [ref first_tok, rest..] => {
            match first_tok.token {
                // <ident>
                Token::Ident(id) => Result::Ok(Expr::Ident{name: id, data: AstData{line: first_tok.line}}, rest),
                // "(" <expr> ...
                Token::OpenParen => {
                    let (parsed_expr, tokens_after_expr) = get_parsed!(parse_expr(rest));
                    match tokens_after_expr {
                        [ref next_tok, next_rest..] => {
                            match next_tok.token {
                                // ... ")"
                                Token::CloseParen => Result::Ok(parsed_expr, next_rest),
                                // ... "," <expr-list>
                                Token::Comma => {
                                    let (mut parsed_list, tokens_after_list) = get_parsed!(parse_expr_list(next_rest));
                                    parsed_list.insert(0, parsed_expr);
                                    Result::Ok(Expr::Tuple{values: parsed_list, data: AstData{line: first_tok.line}}, tokens_after_list)
                                }
                                _ => Result::Err(format!(
                                        "PARSING FAILURE at {},{}: Found {:?} but expected ')' to match '(' at {},{}\n{}\n{}",
                                        next_tok.line + 1,
                                        next_tok.col + 1,
                                        next_tok.token,
                                        first_tok.line + 1,
                                        first_tok.col + 1,
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
                // "{" <clip-statements>
                Token::OpenCurly => {
                    let (parsed_list, tokens_after_list) = get_parsed!(parse_clip_statements(rest));
                    Result::Ok(
                        Expr::Literal{
                            value: Literal::Clip{
                                params:vec![],
                                returns:vec![],
                                statements:parsed_list
                            },
                            data:AstData{line: first_tok.line}
                        }, tokens_after_list)
                }
                // "fn" <clip-def>
                Token::Fn => {
                    let ((parsed_params, parsed_returns, parsed_statements), tokens_after_list) = get_parsed!(parse_clip_def(rest));
                    Result::Ok(
                        Expr::Literal{
                            value: Literal::Clip{
                                params: parsed_params,
                                returns: parsed_returns,
                                statements: parsed_statements
                            },
                            data: AstData{line: first_tok.line}
                        }, tokens_after_list)
                }
                // <bool>
                Token::Bool(b) => Result::Ok(Expr::Literal{value: Literal::Bool(b), data: AstData{line: first_tok.line}}, rest),
                // <int>
                Token::Int(i) => Result::Ok(Expr::Literal{value: Literal::Int(i), data: AstData{line: first_tok.line}}, rest),
                // <float>
                Token::Float(f) => Result::Ok(Expr::Literal{value: Literal::Float(f), data: AstData{line: first_tok.line}}, rest),
                // <string>
                Token::String(ref s) => Result::Ok(Expr::Literal{value: Literal::String(&s[..]), data: AstData{line: first_tok.line}}, rest),
                // "nil"
                Token::Nil => Result::Ok(Expr::Literal{value: Literal::Nil, data: AstData{line: first_tok.line}}, rest),
                _ => Result::Err(format!(
                        "PARSING FAILURE at {},{}: Found {:?} but expected Ident, Literal or '('\n{}\n{}",
                        first_tok.line + 1,
                        first_tok.col + 1,
                        first_tok.token,
                        first_tok.line_string,
                        get_caret_string(first_tok.col)
                    ))
            }
        }
        _ => Result::Err(format!("PARSING FAILURE: Reached end of file, but expected Ident or (Expression)"))
    }
}

// <postfix-expr>
fn parse_postfix_expr<'a>(tokens: &'a[Tok]) -> Result<'a, Expr<'a>> {
    // <primary-expr> ...
    let (parsed_expr, tokens_after_expr) = get_parsed!(parse_primary_expr(tokens));
    match tokens_after_expr {
        [ref first_tok, ..] => {
            match first_tok.token {
                Token::OpenParen | Token::Dot | Token::OpenBrac => {
                    let (parsed_postfixes, tokens_after_postfix) = get_parsed!(parse_postfix_continuation(tokens_after_expr));
                    Result::Ok(Expr::Postfix{expr: Box::new(parsed_expr), postfixes: parsed_postfixes, data: AstData{line: first_tok.line}}, tokens_after_postfix)
                },
                _ => Result::Ok(parsed_expr, tokens_after_expr)
            }
        },
        _ => Result::Ok(parsed_expr, tokens_after_expr)
    }
    
    //parse_primary_expr(tokens)
}

fn parse_postfix_continuation<'a>(tokens: &'a[Tok]) -> Result<'a, Vec<Postfix<'a>>> {
    match tokens {
        [ref first_tok, rest..] => {
            match first_tok.token {
                // ... "(" ...
                Token::OpenParen => {
                    let (parsed_params, tokens_after_params) = get_parsed!(parse_expr_list(rest));
                    let (mut postfix_list, tokens_after_postfix) = get_parsed!(parse_postfix_continuation(tokens_after_params));
                    postfix_list.insert(0, Postfix::Play(parsed_params));
                    Result::Ok(postfix_list, tokens_after_postfix)
                },
                // ... "[" ...
                Token::OpenBrac => {
                    let (parsed_expr, tokens_after_expr) = get_parsed!(parse_expr(rest));
                    match tokens_after_expr {
                        [ref next_tok, next_rest..] => {
                            match next_tok.token {
                                // ... "]"
                                Token::CloseBrac => {
                                    let (mut postfix_list, tokens_after_next) = get_parsed!(parse_postfix_continuation(next_rest));
                                    postfix_list.insert(0, Postfix::Index(Box::new(parsed_expr)));
                                    Result::Ok(postfix_list, tokens_after_next)
                                },
                                _ => Result::Err(format!(
                                    "PARSING FAILURE at {},{}: Found {:?} but expected ']' to match '[' at {},{}\n{}\n{}",
                                    next_tok.line + 1,
                                    next_tok.col + 1,
                                    next_tok.token,
                                    first_tok.line + 1,
                                    first_tok.col + 1,
                                    next_tok.line_string,
                                    get_caret_string(next_tok.col)
                                ))
                            }
                        }
                        _ => Result::Err(format!(
                            "PARSING FAILURE: Reached end of file, but '[' at {},{} is unmatched\n{}\n{}",
                            first_tok.line + 1,
                            first_tok.col + 1,
                            first_tok.line_string,
                            get_caret_string(first_tok.col)
                        ))
                    }
                },
                // ... . ...
                Token::Dot => {
                    match rest {
                        [ref next_tok, next_rest..] => {
                            match next_tok.token {
                                // <ident>
                                Token::Ident(i) => {
                                    let (mut postfix_list, tokens_after_next) = get_parsed!(parse_postfix_continuation(next_rest));
                                    postfix_list.insert(0, Postfix::Access(i));
                                    Result::Ok(postfix_list, tokens_after_next)
                                },
                                _ => Result::Err(format!(
                                    "PARSING FAILURE at {},{}: Found {:?} but expected an Ident\n{}\n{}",
                                    next_tok.line + 1,
                                    next_tok.col + 1,
                                    next_tok.token,
                                    next_tok.line_string,
                                    get_caret_string(next_tok.col)
                                ))
                            }
                        },
                        _ => Result::Err(format!("PARSING FAILURE: Reached end of file but expected an Ident"))
                    }
                },
                // EPS 
                _ => Result::Ok(vec![], tokens)
            }
        }
        // EOF
        _ => Result::Ok(vec![], tokens)
    }
}

// <params>
fn parse_expr_list<'a>(tokens: &'a[Tok]) -> Result<'a, Vec<Expr<'a>>> {
    match tokens {
        [ref first_tok, rest..] => {
            match first_tok.token {
                // )
                Token::CloseParen => {
                    Result::Ok(vec![], rest)
                }
                // <expr> ...
                _ => {
                    let (parsed_expr, tokens_after_expr) = get_parsed!(parse_expr(tokens));
                    match tokens_after_expr {
                        [ref next_tok, rest..] => {
                            match next_tok.token {
                                // ... ")"
                                Token::CloseParen => {
                                    Result::Ok(vec![parsed_expr], rest)
                                },
                                // ... "," ...
                                Token::Comma => {
                                    let (mut parsed_list, tokens_after_params) = get_parsed!(parse_expr_list(rest));
                                    parsed_list.insert(0, parsed_expr);
                                    Result::Ok(parsed_list, tokens_after_params)
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
                        },
                        _ => Result::Err(format!("PARSING FAILURE: Reached end of file but expected another expression or ')'"))
                    }
                }
            }
        },
        _ => Result::Err(format!("PARSING FAILURE: Reached end of file but expected another expression or ')'"))
    }
}

// <unary-expr>
fn parse_unary_expr<'a>(tokens: &'a[Tok]) -> Result<'a, Expr<'a>> {
    match tokens {
        [ref first_tok, rest..] => {
            match first_tok.token {
                // "-" ...
                Token::Sub => {
                    let (parsed_expr, tokens_after_expr) = get_parsed!(parse_unary_expr(rest));
                    Result::Ok(Expr::UnOp{op: UnOp::Neg, expr: Box::new(parsed_expr), data: AstData{line: first_tok.line}}, tokens_after_expr)
                },
                // "!" ...
                Token::Not => {
                    let (parsed_expr, tokens_after_expr) = get_parsed!(parse_unary_expr(rest));
                    Result::Ok(Expr::UnOp{op: UnOp::Not, expr: Box::new(parsed_expr), data: AstData{line: first_tok.line}}, tokens_after_expr)
                },
                // <postfix-expr>
                _ => parse_postfix_expr(tokens)
            }
        }
        _ => Result::Err(format!("Expected expression"))
    }
}

// <exponential-expr>
fn parse_exponential_expr<'a>(tokens: &'a[Tok]) -> Result<'a, Expr<'a>> {
    parse_expr_binary_op!(
        tokens,
        parse_unary_expr,
        parse_exponential_expr,
        [
            Token::Exp => BinOp::Exp
        ]
    )
}

// <multiplicative-expr>
fn parse_multiplicative_expr<'a>(tokens: &'a[Tok]) -> Result<'a, Expr<'a>> {
    parse_expr_binary_op!(
        tokens,
        parse_exponential_expr,
        parse_multiplicative_expr,
        [
            Token::Mul => BinOp::Mul,
            Token::Div => BinOp::Div,
            Token::Mod => BinOp::Mod
        ]
    )
}

// <additive-expr>
fn parse_additive_expr<'a>(tokens: &'a[Tok]) -> Result<'a, Expr<'a>> {
    parse_expr_binary_op!(
        tokens,
        parse_multiplicative_expr,
        parse_additive_expr,
        [
            Token::Add => BinOp::Add,
            Token::Sub => BinOp::Sub
        ]
    )
}

// <in-expr>
fn parse_in_expr<'a>(tokens: &'a[Tok]) -> Result<'a, Expr<'a>> {
    parse_expr_binary_op!(
        tokens,
        parse_additive_expr,
        parse_in_expr,
        [
            Token::In => BinOp::In
        ]
    )
}

// <relational-expr>
fn parse_relational_expr<'a>(tokens: &'a[Tok]) -> Result<'a, Expr<'a>> {
    parse_expr_binary_op!(
        tokens,
        parse_in_expr,
        parse_relational_expr,
        [
            Token::Lt => BinOp::Lt,
            Token::Lte => BinOp::Lte,
            Token::Gt => BinOp::Gt,
            Token::Gte => BinOp::Gte
        ]
    )
}

// <equality-expr>
fn parse_equality_expr<'a>(tokens: &'a[Tok]) -> Result<'a, Expr<'a>> {
    parse_expr_binary_op!(
        tokens,
        parse_relational_expr,
        parse_equality_expr,
        [
            Token::Eq => BinOp::Eq,
            Token::Neq => BinOp::Neq,
            Token::Same => BinOp::Same,
            Token::Nsame => BinOp::Nsame
        ]
    )
}

// <and-expr>
fn parse_and_expr<'a>(tokens: &'a[Tok]) -> Result<'a, Expr<'a>> {
    parse_expr_binary_op!(
        tokens,
        parse_equality_expr,
        parse_and_expr,
        [
            Token::And => BinOp::And
        ]
    )
}

// <or-expr>
fn parse_or_expr<'a>(tokens: &'a[Tok]) -> Result<'a, Expr<'a>> {
    parse_expr_binary_op!(
        tokens,
        parse_and_expr,
        parse_or_expr,
        [
            Token::Or => BinOp::Or
        ]
    )
}

// <expr>
pub fn parse_expr<'a>(tokens: &'a[Tok]) -> Result<'a, Expr<'a>> {
    parse_or_expr(tokens)
}
