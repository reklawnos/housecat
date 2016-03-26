use token::{Token, Tok};
use ast::*;
use parser::stmt::{parse_clip_statements};
use parser::clip::{parse_clip_def};
use parser::{ParseResult, ParserError, ParserErrorType};

// <primary-expr>
fn parse_primary_expr<'a>(tokens: &'a[Tok]) -> ParseResult<'a, Expr<'a>> {
    match tokens {
        // <ident>
        [Tok{token: Token::Ident(id), line, ..}, rest..] => {
            Ok((Expr{expr: ExprType::Ident{name: id}, data: AstData{line: line}}, rest))
        }
        // "(" <expr> ...
        [Tok{token: Token::OpenParen, line, ..}, rest..] => {
            let (parsed_list, tokens_after_list) = try!(parse_expr_list(rest, Token::CloseParen));
            // if parsed_list.len() == 1 {
            //     Ok((parsed_list.remove(0), rest))
            // } else {
                Ok((Expr{expr: ExprType::Tuple{values: parsed_list}, data: AstData{line: line}},
                    tokens_after_list))
            //}
        },
        // "{" <clip-statements>
        [Tok{token: Token::OpenCurly, line, ..}, rest..] => {
            let (parsed_list, tokens_after_list) = try!(parse_clip_statements(rest));
            Ok((
                Expr{
                    expr: ExprType::Literal{
                        value: Literal::Clip{
                            params:vec![],
                            returns:vec![],
                            statements:parsed_list
                        }
                    },
                    data:AstData{line: line}
                }, tokens_after_list))
        }
        // "fn" <clip-def>
        [Tok{token: Token::Fn, line, ..}, rest..] => {
            let ((parsed_params, parsed_returns, parsed_statements), tokens_after_list) = {
                try!(parse_clip_def(rest))
            };
            Ok((
                Expr{
                    expr: ExprType::Literal{
                        value: Literal::Clip{
                            params: parsed_params,
                            returns: parsed_returns,
                            statements: parsed_statements
                        },
                    },
                    data: AstData{line: line}
                }, tokens_after_list))
        }
        // <bool>
        [Tok{token: Token::Bool(b), line, ..}, rest..] => {
            Ok((Expr{expr: ExprType::Literal{value: Literal::Bool(b)},
                     data: AstData{line: line}}, rest))
        }
        // <int>
        [Tok{token: Token::Int(i), line, ..}, rest..] => {
            Ok((Expr{expr: ExprType::Literal{value: Literal::Int(i)},
                     data: AstData{line: line}}, rest))
        }
        // <float>
        [Tok{token: Token::Float(f), line, ..}, rest..] => {
            Ok((Expr{expr: ExprType::Literal{value: Literal::Float(f)},
                     data: AstData{line: line}}, rest))
        }
        // <string>
        [Tok{token: Token::String(ref s), line, ..}, rest..] => {
            Ok((Expr{expr: ExprType::Literal{value: Literal::String(&s[..])},
                     data: AstData{line: line}}, rest))
        }
        // "nil"
        [Tok{token: Token::Nil, line, ..}, rest..] => {
            Ok((Expr{expr: ExprType::Literal{value: Literal::Nil},
                     data: AstData{line: line}}, rest))
        }
        [ref tok, ..] => Err(ParserError{
            actual: tok.clone(),
            error_type: ParserErrorType::ExpectedBaseExpression,
            hint: None
        }),
        [] => panic!("Missing EOF")
    }
}

// <postfix-expr>
fn parse_postfix_expr<'a>(tokens: &'a[Tok]) -> ParseResult<'a, Expr<'a>> {
    // <primary-expr> ...
    let (parsed_expr, tokens_after_expr) = try!(parse_primary_expr(tokens));
    match tokens_after_expr {
        [ref first_tok, ..] => {
            match first_tok.token {
                //TODO: fix this - ends up checking the type of this token twice if it's a postfix
                Token::OpenParen | Token::Access | Token::AccessSelf | Token::OpenBrac => {
                    let (parsed_postfixes, tokens_after_postfix) = {
                        try!(parse_postfix_continuation(tokens_after_expr))
                    };
                    Ok((Expr{expr: ExprType::Postfix{expr: Box::new(parsed_expr),
                                               postfixes: parsed_postfixes},
                             data: AstData{line: first_tok.line}}, tokens_after_postfix))
                },
                _ => Ok((parsed_expr, tokens_after_expr))
            }
        },
        [] => Ok((parsed_expr, tokens_after_expr))
    }
}

// <params>
fn parse_params<'a>(tokens: &'a[Tok]) -> ParseResult<'a, Vec<Expr<'a>>> {
     match tokens {
        // ... ")" ...
        [Tok{token: Token::CloseParen, ..}, rest..] => {
            Ok((vec![], rest))
        }
        [_, ..] => {
            parse_expr_list(tokens, Token::CloseParen)
        }
        [] => panic!("Missing EOF")
    }
}

// <postfix-continuation>
fn parse_postfix_continuation<'a>(tokens: &'a[Tok]) -> ParseResult<'a, Vec<Postfix<'a>>> {
    match tokens {
        // ... "(" ...
        [Tok{token: Token::OpenParen, ..}, rest..] => {
            let (parsed_params, tokens_after_params) = try!(parse_params(rest));
            let (mut postfix_list, tokens_after_postfix) = {
                try!(parse_postfix_continuation(tokens_after_params))
            };
            postfix_list.insert(0, Postfix::Play(parsed_params));
            Ok((postfix_list, tokens_after_postfix))
        }
        // ... "[" ...
        [ref start_tok, rest..] if start_tok.token == Token::OpenBrac => {
            let (parsed_expr, tokens_after_expr) = try!(parse_expr(rest));
            match tokens_after_expr {
                // ... "]"
                [Tok{token: Token::CloseBrac, ..}, rest..] => {
                    let (mut postfix_list, tokens_after_next) = {
                        try!(parse_postfix_continuation(rest))
                    };
                    postfix_list.insert(0, Postfix::Index(Box::new(parsed_expr)));
                    Ok((postfix_list, tokens_after_next))
                },
                [ref tok, ..] => Err(ParserError{
                    actual: tok.clone(),
                    error_type: ParserErrorType::ExpectedMatchingToken{
                        expected: Token::CloseBrac,
                        start_tok: start_tok.clone()
                    },
                    hint: None
                }),
                [] => panic!("Missing EOF")
            }
        }
        // ... "." ...
        [Tok{token: Token::Access, ..}, rest..] => {
            match rest {
                // <ident>
                [Tok{token: Token::Ident(i), ..}, rest..] => {
                    let (mut postfix_list, tokens_after_next) = {
                        try!(parse_postfix_continuation(rest))
                    };
                    postfix_list.insert(0, Postfix::Access(i));
                    Ok((postfix_list, tokens_after_next))
                },
                [ref tok, ..] => Err(ParserError{
                    actual: tok.clone(),
                    error_type: ParserErrorType::ExpectedIdent,
                    hint: Some("`.` must be followed by an ident with the name of a property to access")
                }),
                [] => panic!("Missing EOF")
            }
        }
        // ... "|" <ident> "(" ...
        [Tok{token: Token::AccessSelf, ..}, rest..] => {
            match rest {
                // <ident> "(" <params>
                [Tok{token: Token::Ident(ident), ..}, rest..] => {
                    match rest {
                        [Tok{token: Token::OpenParen, ..}, rest..] => {
                            let (params_list, tokens_after_params) = try!(parse_params(rest));
                            let (mut postfix_list, tokens_after_next) = {
                                try!(parse_postfix_continuation(tokens_after_params))
                            };
                            postfix_list.insert(0, Postfix::PlaySelf(ident, params_list));
                            Ok((postfix_list, tokens_after_next))
                        }
                        [ref tok, ..] => Err(ParserError{
                            actual: tok.clone(),
                            error_type: ParserErrorType::ExpectedTokens{
                                expected: vec!(Token::OpenParen)
                            },
                            hint: None
                        }),
                        [] => panic!("Missing EOF")
                    }
                },
                [ref tok, ..] => Err(ParserError{
                    actual: tok.clone(),
                    error_type: ParserErrorType::ExpectedIdent,
                    hint: Some("`|` must be followed by an ident with the name of a property to access")
                }),
                [] => panic!("Missing EOF")
            }
        }
        // EPS
        _ => Ok((vec![], tokens))
    }
}

// <expr-list>
fn parse_expr_list<'a>(tokens: &'a[Tok], delimiter_tok: Token<'a>) -> ParseResult<'a, Vec<Expr<'a>>> {
    match tokens {
        // <expr> ...
        [Tok{token: _, ..}, ..] => {
            let (parsed_expr, tokens_after_expr) = try!(parse_expr(tokens));
            match tokens_after_expr {
                // ... <delimiter_tok>
                [Tok{ref token, ..}, rest..] if token == &delimiter_tok => {
                    Ok((vec![parsed_expr], rest))
                },
                // ... "," ...
                [Tok{token: Token::Comma, ..}, rest..] => {
                    let (mut parsed_list, tokens_after_params) = try!(parse_expr_list(rest, delimiter_tok));
                    parsed_list.insert(0, parsed_expr);
                    Ok((parsed_list, tokens_after_params))
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
        [] => panic!("Missing EOF")
    }
}

// <unary-expr>
fn parse_unary_expr<'a>(tokens: &'a[Tok]) -> ParseResult<'a, Expr<'a>> {
    match tokens {
        // "-" ...
        [Tok{token: Token::Sub, line, ..}, rest..] => {
            let (parsed_expr, tokens_after_expr) = try!(parse_unary_expr(rest));
            Ok((Expr{expr: ExprType::UnOp{op: UnOp::Neg, expr: Box::new(parsed_expr)},
                     data: AstData{line: line}},
                tokens_after_expr))
        }
        // "!" ...
        [Tok{token: Token::Not, line, ..}, rest..] => {
            let (parsed_expr, tokens_after_expr) = try!(parse_unary_expr(rest));
            Ok((Expr{expr: ExprType::UnOp{op: UnOp::Not, expr: Box::new(parsed_expr)},
                     data: AstData{line: line}},
                tokens_after_expr))
        }
        // "$" ...
        [Tok{token: Token::Get, line, ..}, rest..] => {
            let (parsed_expr, tokens_after_expr) = try!(parse_unary_expr(rest));
            Ok((Expr{expr: ExprType::UnOp{op: UnOp::Get, expr: Box::new(parsed_expr)},
                     data: AstData{line: line}},
                tokens_after_expr))
        }
        // <postfix-expr>
        _ => parse_postfix_expr(tokens)
    }
}

// <exponential-expr>
fn parse_exponential_expr<'a>(tokens: &'a[Tok]) -> ParseResult<'a, Expr<'a>> {
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
fn parse_multiplicative_expr<'a>(tokens: &'a[Tok]) -> ParseResult<'a, Expr<'a>> {
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
fn parse_additive_expr<'a>(tokens: &'a[Tok]) -> ParseResult<'a, Expr<'a>> {
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
fn parse_in_expr<'a>(tokens: &'a[Tok]) -> ParseResult<'a, Expr<'a>> {
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
fn parse_relational_expr<'a>(tokens: &'a[Tok]) -> ParseResult<'a, Expr<'a>> {
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
fn parse_equality_expr<'a>(tokens: &'a[Tok]) -> ParseResult<'a, Expr<'a>> {
    parse_expr_binary_op!(
        tokens,
        parse_relational_expr,
        parse_equality_expr,
        [
            Token::Eq => BinOp::Eq,
            Token::Neq => BinOp::Neq
        ]
    )
}

// <and-expr>
fn parse_and_expr<'a>(tokens: &'a[Tok]) -> ParseResult<'a, Expr<'a>> {
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
fn parse_or_expr<'a>(tokens: &'a[Tok]) -> ParseResult<'a, Expr<'a>> {
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
pub fn parse_expr<'a>(tokens: &'a[Tok]) -> ParseResult<'a, Expr<'a>> {
    parse_or_expr(tokens)
}
