use token::{Token, Tok};
use ast::ast::*;

macro_rules! parse_expr_binary_op(
    ($tokens:ident, $parse_lhs:ident, $parse_rhs:ident, [ $($tok:pat => $op:expr),+ ]) => ({
        // <LHS> ...
        let (parsed_lhs, tokens_after_lhs) = $parse_lhs($tokens);
        match tokens_after_lhs {
            [ref next_tok, rest..] => {
                match next_tok.token {
                    $(
                        // ... <op> <RHS>
                        $tok => {
                            let (parsed_rhs, tokens_after_term) = $parse_rhs(rest);
                            (
                                Expr::ExprBinOp(
                                    $op,
                                    Box::new(parsed_lhs),
                                    Box::new(parsed_rhs)
                                ),
                                tokens_after_term
                            )
                        },
                    )+
                    // <LHS>
                    _ => (parsed_lhs, tokens_after_lhs),
                }
            }
            // <LHS>
            _ => (parsed_lhs, tokens_after_lhs),
        }
    });
);

// <primary-expr>
fn parse_primary_expr(tokens: &[Tok]) -> (Expr, &[Tok]) {
    match tokens {
        [ref first_tok, rest..] => {
            match first_tok.token {
                // <ident>
                Token::Ident(ref id) => (Expr::ExprIdent(id.clone()), rest),
                // ( <expr> ...
                Token::OpenParen => {
                    let (parsed_expr, tokens_after_expr) = parse_expr(rest);
                    match tokens_after_expr {
                        [ref next_tok, next_rest..] => {
                            match next_tok.token {
                                // ... )
                                Token::CloseParen => (parsed_expr, next_rest),
                                _ => panic!(
                                        "ERROR at {},{}: Found {:?} but expected ')' to match '(' at {},{}",
                                        next_tok.line + 1,
                                        next_tok.col + 1,
                                        next_tok.token,
                                        first_tok.line + 1,
                                        first_tok.col + 1,
                                    )
                            }
                        }
                        _ => panic!(
                            "ERROR: Reached end of file, but the paren at {},{} is unmatched",
                            first_tok.line + 1,
                            first_tok.col + 1
                        )
                    }
                },
                // <bool>
                Token::Bool(b) => (Expr::ExprLiteral(Literal::LitBool(b)), rest),
                // <int>
                Token::Int(i) => (Expr::ExprLiteral(Literal::LitInt(i)), rest),
                // <float>
                Token::Float(f) => (Expr::ExprLiteral(Literal::LitFloat(f)), rest),
                // <string>
                Token::String(ref s) => (Expr::ExprLiteral(Literal::LitString(s.clone())), rest),
                // nil
                Token::Nil => (Expr::ExprLiteral(Literal::LitNil), rest),
                _ => panic!(
                        "ERROR at {},{}: Found {:?} but Expected Ident or '('",
                        first_tok.line + 1,
                        first_tok.col + 1,
                        first_tok.token
                    )
            }
        }
        _ => panic!("ERROR: Reached end of file, but Expected Ident or (Expression)")
    }
}

// <postfix-expr>
fn parse_postfix_expr(tokens: &[Tok]) -> (Expr, &[Tok]) {
    // <primary-expr> ...
    let (parsed_expr, tokens_after_expr) = parse_primary_expr(tokens);
    match tokens_after_expr {
        [ref first_tok, ..] => {
            match first_tok.token {
                Token::OpenParen | Token::Dot | Token::OpenBrac => {
                    let (parsed_postfix, tokens_after_postfix) = parse_postfix_continuation(tokens_after_expr);
                    (Expr::ExprPostfix(Box::new(parsed_expr), Box::new(parsed_postfix)), tokens_after_postfix)
                },
                _ => (parsed_expr, tokens_after_expr)
            }
        },
        _ => (parsed_expr, tokens_after_expr)
    }
    
    //parse_primary_expr(tokens)
}

fn parse_postfix_continuation(tokens: &[Tok]) -> (Postfix, &[Tok]) {
    match tokens {
        [ref first_tok, rest..] => {
            match first_tok.token {
                // ... ( ...
                Token::OpenParen => {
                    let (parsed_args, tokens_after_args) = parse_args(rest);
                    let (next_postfix, tokens_after_postfix) = parse_postfix_continuation(tokens_after_args);
                    (Postfix::PostfixPlay(Box::new(parsed_args), Box::new(next_postfix)), tokens_after_postfix)
                },
                // ... [ ...
                Token::OpenBrac => {
                    let (parsed_expr, tokens_after_expr) = parse_expr(rest);
                    match tokens_after_expr {
                        [ref next_tok, next_rest..] => {
                            match next_tok.token {
                                // ... ]
                                Token::CloseBrac => {
                                    let (next_postfix, tokens_after_next) = parse_postfix_continuation(next_rest);
                                    (Postfix::PostfixIndex(Box::new(parsed_expr), Box::new(next_postfix)), tokens_after_next)
                                },
                                _ => panic!(
                                    "ERROR at {},{}: Found {:?} but expected ']' to match '[' at {},{}",
                                    next_tok.line + 1,
                                    next_tok.col + 1,
                                    next_tok.token,
                                    first_tok.line + 1,
                                    first_tok.col + 1,
                                )
                            }
                        }
                        _ => panic!(
                            "ERROR: Reached end of file, but the bracket at {},{} is unmatched",
                            first_tok.line + 1,
                            first_tok.col + 1
                        )
                    }
                },
                // ... . ...
                Token::Dot => {
                    match rest {
                        [ref next_tok, next_rest..] => {
                            match next_tok.token {
                                // <ident>
                                Token::Ident(ref i) => {
                                    let (next_postfix, tokens_after_next) = parse_postfix_continuation(next_rest);
                                    (Postfix::PostfixAccess(i.clone(), Box::new(next_postfix)), tokens_after_next)
                                },
                                _ => panic!(
                                    "ERROR at {},{}: Found {:?} but expected an ident",
                                    next_tok.line + 1,
                                    next_tok.col + 1,
                                    next_tok.token,
                                )
                            }
                        },
                        _ => panic!("ERROR: Reached end of file but expected an ident")
                    }
                },
                // ... 
                _ => panic!(
                    "ERROR at {},{}: Unexpected Token {:?}",
                    first_tok.line + 1,
                    first_tok.col + 1,
                    first_tok.token,
                )
            }
        }
        // EPS
        _ => (Postfix::PostfixNone, tokens),
    }
}

// <args>
fn parse_args(tokens: &[Tok]) -> (Args, &[Tok]) {
    match tokens {
        [ref first_tok, rest..] => {
            match first_tok.token {
                // )
                Token::CloseParen => {
                    (Args::ArgsNone, rest)
                }
                // <expr> ...
                _ => {
                    let (parsed_expr, tokens_after_expr) = parse_expr(tokens);
                    match tokens_after_expr {
                        [ref next_tok, rest..] => {
                            match next_tok.token {
                                // ... )
                                Token::CloseParen => {
                                    (Args::ArgsItem(Box::new(parsed_expr), Box::new(Args::ArgsNone)), rest)
                                },
                                // ... , ...
                                Token::Comma => {
                                    let (parsed_arg, tokens_after_arg) = parse_args(rest);
                                    (Args::ArgsItem(Box::new(parsed_expr), Box::new(parsed_arg)), tokens_after_arg)
                                }
                                _ => panic!(
                                    "ERROR at {},{}: Expected ')' or ',' but found {:?}",
                                    next_tok.line + 1,
                                    next_tok.col + 1,
                                    next_tok.token
                                )
                            }
                        },
                        _ => panic!("ERROR: Reached end of file but expected more arguments or ')'")
                    }
                }
            }
        },
        _ => panic!("ERROR: Reached end of file but expected arguments or ')'")
    }
}

// <unary-expr>
fn parse_unary_expr(tokens: &[Tok]) -> (Expr, &[Tok]) {
    match tokens {
        [ref first_tok, rest..] => {
            match first_tok.token {
                // - ...
                Token::Sub => {
                    let (parsed_expr, tokens_after_expr) = parse_unary_expr(rest);
                    (Expr::ExprUnOp(UnOp::UnNeg, Box::new(parsed_expr)), tokens_after_expr)
                },
                // ! ...
                Token::Not => {
                    let (parsed_expr, tokens_after_expr) = parse_unary_expr(rest);
                    (Expr::ExprUnOp(UnOp::UnNot, Box::new(parsed_expr)), tokens_after_expr)
                },
                // <primary-expr>
                _ => parse_postfix_expr(tokens)
            }
        }
        _ => panic!("Expected expression")
    }
}

// <exponential-expr>
fn parse_exponential_expr(tokens: &[Tok]) -> (Expr, &[Tok]) {
    parse_expr_binary_op!(
        tokens,
        parse_unary_expr,
        parse_exponential_expr,
        [
            Token::Exp => BinOp::BinExp
        ]
    )
}

// <multiplicative-expr>
fn parse_multiplicative_expr(tokens: &[Tok]) -> (Expr, &[Tok]) {
    parse_expr_binary_op!(
        tokens,
        parse_exponential_expr,
        parse_multiplicative_expr,
        [
            Token::Mul => BinOp::BinMul,
            Token::Div => BinOp::BinDiv,
            Token::Mod => BinOp::BinMod
        ]
    )
}

// <additive-expr>
fn parse_additive_expr(tokens: &[Tok]) -> (Expr, &[Tok]) {
    parse_expr_binary_op!(
        tokens,
        parse_multiplicative_expr,
        parse_additive_expr,
        [
            Token::Add => BinOp::BinAdd,
            Token::Sub => BinOp::BinSub
        ]
    )
}

// <relational-expr>
fn parse_relational_expr(tokens: &[Tok]) -> (Expr, &[Tok]) {
    parse_expr_binary_op!(
        tokens,
        parse_additive_expr,
        parse_relational_expr,
        [
            Token::Lt => BinOp::BinLt,
            Token::Lte => BinOp::BinLte,
            Token::Gt => BinOp::BinGt,
            Token::Gte => BinOp::BinGte
        ]
    )
}

// <equality-expr>
fn parse_equality_expr(tokens: &[Tok]) -> (Expr, &[Tok]) {
    parse_expr_binary_op!(
        tokens,
        parse_relational_expr,
        parse_equality_expr,
        [
            Token::Eq => BinOp::BinEq,
            Token::Neq => BinOp::BinNeq,
            Token::Same => BinOp::BinSame,
            Token::Nsame => BinOp::BinNsame
        ]
    )
}

// <and-expr>
fn parse_and_expr(tokens: &[Tok]) -> (Expr, &[Tok]) {
    parse_expr_binary_op!(
        tokens,
        parse_equality_expr,
        parse_and_expr,
        [
            Token::And => BinOp::BinAnd
        ]
    )
}

// <or-expr>
fn parse_or_expr(tokens: &[Tok]) -> (Expr, &[Tok]) {
    parse_expr_binary_op!(
        tokens,
        parse_and_expr,
        parse_or_expr,
        [
            Token::Or => BinOp::BinOr
        ]
    )
}

// <expr>
pub fn parse_expr(tokens: &[Tok]) -> (Expr, &[Tok]) {
    parse_or_expr(tokens)
}
