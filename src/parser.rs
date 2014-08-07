use token;
use ast::Ast;

macro_rules! parse_expr_binary_op(
    ($tokens:ident, $parse_lhs:ident, $parse_rhs:ident, [ $($tok:pat -> $op:expr),+ ]) => ({
        // <LHS> ...
        let (parsed_lhs, tokens_after_LHS) = $parse_lhs($tokens);
        match tokens_after_LHS {
            [ref next_tok, ..rest] => {
                match next_tok.token {
                    $(
                        // ... <op> <RHS>
                        $tok => {
                            let (parsed_rhs, tokens_after_term) = $parse_rhs(rest);
                            (
                                Some(
                                    Ast::ExprBinOp(
                                        $op,
                                        box parsed_lhs.unwrap(),
                                        box parsed_rhs.unwrap()
                                    )
                                ),
                                tokens_after_term
                            )
                        },
                    )+
                    // <LHS>
                    _ => (parsed_lhs, tokens_after_LHS),
                }
            }
            // <LHS>
            _ => (parsed_lhs, tokens_after_LHS),
        }
    });
)

// <primary-expr>
fn parse_primary_expr(tokens: &[token::Tok]) -> (Option<Ast::Expr>, &[token::Tok]) {
    match tokens {
        [ref first_tok, ..rest] => {
            match first_tok.token {
                // <ident>
                token::Ident(ref id) => (Some(Ast::ExprIdent(id.clone())), rest),
                // ( <expr> ...
                token::OpenParen => {
                    let (parsed_expr, tokens_after_expr) = parse_expr(rest);
                    match tokens_after_expr {
                        [ref next_tok, ..next_rest] => {
                            match next_tok.token {
                                // ... )
                                token::CloseParen => (parsed_expr, next_rest),
                                _ => fail!(
                                        "ERROR at {},{}: Found {} but expected ')' to match '(' at {},{}",
                                        next_tok.line + 1,
                                        next_tok.col + 1,
                                        next_tok.token,
                                        first_tok.line + 1,
                                        first_tok.col + 1,
                                    )
                            }
                        }
                        _ => fail!(
                            "ERROR: Reached end of file, but the paren at {},{} is unmatched",
                            first_tok.line + 1,
                            first_tok.col + 1
                        )
                    }
                },
                // <bool>
                token::Bool(b) => (Some(Ast::ExprLiteral(Ast::LitBool(b))), rest),
                // <int>
                token::Int(i) => (Some(Ast::ExprLiteral(Ast::LitInt(i))), rest),
                // <float>
                token::Float(f) => (Some(Ast::ExprLiteral(Ast::LitFloat(f))), rest),
                // <string>
                token::String(ref s) => (Some(Ast::ExprLiteral(Ast::LitString(s.clone()))), rest),
                // nil
                token::Nil => (Some(Ast::ExprLiteral(Ast::LitNil)), rest),
                _ => fail!(
                        "ERROR at {},{}: Found {} but Expected Ident or '('",
                        first_tok.line + 1,
                        first_tok.col + 1,
                        first_tok.token
                    )
            }
        }
        _ => fail!("ERROR: Reached end of file, but Expected Ident or (Expression)")
    }
}

// <postfix-expr>
fn parse_postfix_expr(tokens: &[token::Tok]) -> (Option<Ast::Expr>, &[token::Tok]) {
    // <primary-expr> ...
    let (parsed_expr, tokens_after_expr) = parse_primary_expr(tokens);
    match tokens_after_expr {
        [ref first_tok, ..] => {
            match first_tok.token {
                token::OpenParen | token::Dot | token::OpenBrac => {
                    let (parsed_postfix, tokens_after_postfix) = parse_postfix_continuation(tokens_after_expr);
                    (Some(Ast::ExprPostfix(box parsed_expr.unwrap(), box parsed_postfix)), tokens_after_postfix)
                },
                _ => (parsed_expr, tokens_after_expr)
            }
        },
        _ => (parsed_expr, tokens_after_expr)
    }
    
    //parse_primary_expr(tokens)
}

fn parse_postfix_continuation(tokens: &[token::Tok]) -> (Ast::Postfix, &[token::Tok]) {
    match tokens {
        [ref first_tok, ..rest] => {
            match first_tok.token {
                // ... ( ...
                token::OpenParen => {
                    let (parsed_args, tokens_after_args) = parse_args(rest);
                    let (next_postfix, tokens_after_postfix) = parse_postfix_continuation(tokens_after_args);
                    (Ast::PostfixPlay(box parsed_args, box next_postfix), tokens_after_postfix)
                },
                // ... [ ...
                token::OpenBrac => {
                    let (parsed_expr, tokens_after_expr) = parse_expr(rest);
                    match tokens_after_expr {
                        [ref next_tok, ..next_rest] => {
                            match next_tok.token {
                                // ... ]
                                token::CloseBrac => {
                                    let (next_postfix, tokens_after_next) = parse_postfix_continuation(next_rest);
                                    (Ast::PostfixIndex(box parsed_expr.unwrap(), box next_postfix), tokens_after_next)
                                },
                                _ => fail!(
                                    "ERROR at {},{}: Found {} but expected ']' to match '[' at {},{}",
                                    next_tok.line + 1,
                                    next_tok.col + 1,
                                    next_tok.token,
                                    first_tok.line + 1,
                                    first_tok.col + 1,
                                )
                            }
                        }
                        _ => fail!(
                            "ERROR: Reached end of file, but the bracket at {},{} is unmatched",
                            first_tok.line + 1,
                            first_tok.col + 1
                        )
                    }
                },
                // ... . ...
                token::Dot => {
                    match rest {
                        [ref next_tok, ..next_rest] => {
                            match next_tok.token {
                                // <ident>
                                token::Ident(ref i) => {
                                    let (next_postfix, tokens_after_next) = parse_postfix_continuation(next_rest);
                                    (Ast::PostfixAccess(i.clone(), box next_postfix), tokens_after_next)
                                },
                                _ => fail!(
                                    "ERROR at {},{}: Found {} but expected an ident",
                                    next_tok.line + 1,
                                    next_tok.col + 1,
                                    next_tok.token,
                                )
                            }
                        },
                        _ => fail!("ERROR: Reached end of file but expected an ident")
                    }
                },
                // ... 
                _ => fail!(
                    "ERROR at {},{}: Unexpected token {}",
                    first_tok.line + 1,
                    first_tok.col + 1,
                    first_tok.token,
                )
            }
        }
        // EPS
        _ => (Ast::PostfixNone, tokens),
    }
}

// <args>
fn parse_args(tokens: &[token::Tok]) -> (Ast::Args, &[token::Tok]) {
    match tokens {
        [ref first_tok, ..rest] => {
            match first_tok.token {
                // )
                token::CloseParen => {
                    (Ast::ArgsNone, rest)
                }
                // <expr> ...
                _ => {
                    let (parsed_expr, tokens_after_expr) = parse_expr(tokens);
                    match tokens_after_expr {
                        [ref next_tok, ..rest] => {
                            match next_tok.token {
                                // ... )
                                token::CloseParen => {
                                    (Ast::ArgsItem(box parsed_expr.unwrap(), box Ast::ArgsNone), rest)
                                },
                                // ... , ...
                                token::Comma => {
                                    let (parsed_arg, tokens_after_arg) = parse_args(rest);
                                    (Ast::ArgsItem(box parsed_expr.unwrap(), box parsed_arg), tokens_after_arg)
                                }
                                _ => fail!(
                                    "ERROR at {},{}: Expected ')' or ',' but found {}",
                                    next_tok.line + 1,
                                    next_tok.col + 1,
                                    next_tok.token
                                )
                            }
                        },
                        _ => fail!("ERROR: Reached end of file but expected arguments or ')'")
                    }
                }
            }
        },
        _ => fail!("ERROR: Reached end of file but expected arguments or ')'")
    }
}

// <unary-expr>
fn parse_unary_expr(tokens: &[token::Tok]) -> (Option<Ast::Expr>, &[token::Tok]) {
    match tokens {
        [ref first_tok, ..rest] => {
            match first_tok.token {
                // - ...
                token::Sub => {
                    let (parsed_expr, tokens_after_expr) = parse_unary_expr(rest);
                    (Some(Ast::ExprUnOp(Ast::UnNeg, box parsed_expr.unwrap())), tokens_after_expr)
                },
                // ! ...
                token::Not => {
                    let (parsed_expr, tokens_after_expr) = parse_unary_expr(rest);
                    (Some(Ast::ExprUnOp(Ast::UnNot, box parsed_expr.unwrap())), tokens_after_expr)
                },
                // <primary-expr>
                _ => parse_postfix_expr(tokens)
            }
        }
        _ => fail!("Expected expression")
    }
}

// <exponential-expr>
fn parse_exponential_expr(tokens: &[token::Tok]) -> (Option<Ast::Expr>, &[token::Tok]) {
    parse_expr_binary_op!(
        tokens,
        parse_unary_expr,
        parse_exponential_expr,
        [
            token::Exp -> Ast::BinExp
        ]
    )
}

// <multiplicative-expr>
fn parse_multiplicative_expr(tokens: &[token::Tok]) -> (Option<Ast::Expr>, &[token::Tok]) {
    parse_expr_binary_op!(
        tokens,
        parse_exponential_expr,
        parse_multiplicative_expr,
        [
            token::Mul -> Ast::BinMul,
            token::Div -> Ast::BinDiv,
            token::Mod -> Ast::BinMod
        ]
    )
}

// <additive-expr>
fn parse_additive_expr(tokens: &[token::Tok]) -> (Option<Ast::Expr>, &[token::Tok]) {
    parse_expr_binary_op!(
        tokens,
        parse_multiplicative_expr,
        parse_additive_expr,
        [
            token::Add -> Ast::BinAdd,
            token::Sub -> Ast::BinSub
        ]
    )
}

// <relational-expr>
fn parse_relational_expr(tokens: &[token::Tok]) -> (Option<Ast::Expr>, &[token::Tok]) {
    parse_expr_binary_op!(
        tokens,
        parse_additive_expr,
        parse_relational_expr,
        [
            token::Lt -> Ast::BinLt,
            token::Lte -> Ast::BinLte,
            token::Gt -> Ast::BinGt,
            token::Gte -> Ast::BinGte
        ]
    )
}

// <equality-expr>
fn parse_equality_expr(tokens: &[token::Tok]) -> (Option<Ast::Expr>, &[token::Tok]) {
    parse_expr_binary_op!(
        tokens,
        parse_relational_expr,
        parse_equality_expr,
        [
            token::Eq -> Ast::BinEq,
            token::Neq -> Ast::BinNeq,
            token::Same -> Ast::BinSame,
            token::Nsame -> Ast::BinNsame
        ]
    )
}

// <and-expr>
fn parse_and_expr(tokens: &[token::Tok]) -> (Option<Ast::Expr>, &[token::Tok]) {
    parse_expr_binary_op!(
        tokens,
        parse_equality_expr,
        parse_and_expr,
        [
            token::And -> Ast::BinAnd
        ]
    )
}

// <or-expr>
fn parse_or_expr(tokens: &[token::Tok]) -> (Option<Ast::Expr>, &[token::Tok]) {
    parse_expr_binary_op!(
        tokens,
        parse_and_expr,
        parse_or_expr,
        [
            token::Or -> Ast::BinOr
        ]
    )
}

// <expr>
pub fn parse_expr(tokens: &[token::Tok]) -> (Option<Ast::Expr>, &[token::Tok]) {
    //TODO: add steps before additivex
    parse_or_expr(tokens)
}
