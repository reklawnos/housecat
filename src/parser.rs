macro_rules! get_parsed(
    ($parsed:expr) => ({
        match $parsed {
            Result::Ok(p, toks) => (p, toks),
            Result::Err(e) => {return Result::Err(e);}
        }
    });
);

macro_rules! parse_expr_binary_op(
    ($tokens:ident, $parse_lhs:ident, $parse_rhs:ident, [ $($tok:pat => $op:expr),+ ]) => ({
        // <LHS> ...
        let (parsed_lhs, tokens_after_lhs) = get_parsed!($parse_lhs($tokens));
        match tokens_after_lhs {
            [ref next_tok, rest..] => {
                match next_tok.token {
                    $(
                        // ... <op> <RHS>
                        $tok => {
                            let (parsed_rhs, tokens_after_term) = get_parsed!($parse_rhs(rest));
                            Result::Ok(
                                Expr::BinOp(
                                    $op,
                                    Box::new(parsed_lhs),
                                    Box::new(parsed_rhs)
                                ),
                                tokens_after_term
                            )
                        },
                    )+
                    // <LHS>
                    _ => Result::Ok(parsed_lhs, tokens_after_lhs),
                }
            }
            // <LHS>
            _ => Result::Ok(parsed_lhs, tokens_after_lhs),
        }
    });
);

use token::{Token, Tok};
use ast::*;
use utils::*;

pub enum Result<'a, T> {
    Ok(T, &'a[Tok<'a>]),
    Err(String)
}

#[allow(dead_code)]
fn print_toks<'a>(func: &str, tokens: &'a[Tok]) {
    print!("{}: ", func);
    for t in tokens.iter() {
        print!("{:?}, ", t.token);
    }
    println!("");
}

// <primary-expr>
fn parse_primary_expr<'a>(tokens: &'a[Tok]) -> Result<'a, Expr> {
    match tokens {
        [ref first_tok, rest..] => {
            match first_tok.token {
                // <ident>
                Token::Ident(ref id) => Result::Ok(Expr::Ident(id.clone()), rest),
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
                                    Result::Ok(Expr::Tuple(Box::new(parsed_list)), tokens_after_list)
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
                    Result::Ok(Expr::Literal(Literal::Clip(vec![], vec![], parsed_list)), tokens_after_list)
                }
                // "fn" <clip-def>
                Token::Fn => {
                    let ((parsed_params, parsed_returns, parsed_statements), tokens_after_list) = get_parsed!(parse_clip_def(rest));
                    Result::Ok(Expr::Literal(Literal::Clip(parsed_params, parsed_returns, parsed_statements)), tokens_after_list)
                }
                // <bool>
                Token::Bool(b) => Result::Ok(Expr::Literal(Literal::Bool(b)), rest),
                // <int>
                Token::Int(i) => Result::Ok(Expr::Literal(Literal::Int(i)), rest),
                // <float>
                Token::Float(f) => Result::Ok(Expr::Literal(Literal::Float(f)), rest),
                // <string>
                Token::String(ref s) => Result::Ok(Expr::Literal(Literal::String(s.clone())), rest),
                // "nil"
                Token::Nil => Result::Ok(Expr::Literal(Literal::Nil), rest),
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
fn parse_postfix_expr<'a>(tokens: &'a[Tok]) -> Result<'a, Expr> {
    // <primary-expr> ...
    let (parsed_expr, tokens_after_expr) = get_parsed!(parse_primary_expr(tokens));
    match tokens_after_expr {
        [ref first_tok, ..] => {
            match first_tok.token {
                Token::OpenParen | Token::Dot | Token::OpenBrac => {
                    let (parsed_postfix, tokens_after_postfix) = get_parsed!(parse_postfix_continuation(tokens_after_expr));
                    Result::Ok(Expr::Postfix(Box::new(parsed_expr), parsed_postfix), tokens_after_postfix)
                },
                _ => Result::Ok(parsed_expr, tokens_after_expr)
            }
        },
        _ => Result::Ok(parsed_expr, tokens_after_expr)
    }
    
    //parse_primary_expr(tokens)
}

fn parse_postfix_continuation<'a>(tokens: &'a[Tok]) -> Result<'a, Vec<Postfix>> {
    match tokens {
        [ref first_tok, rest..] => {
            match first_tok.token {
                // ... "(" ...
                Token::OpenParen => {
                    let (parsed_args, tokens_after_args) = get_parsed!(parse_expr_list(rest));
                    let (mut postfix_list, tokens_after_postfix) = get_parsed!(parse_postfix_continuation(tokens_after_args));
                    postfix_list.insert(0, Postfix::Play(parsed_args));
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
                                Token::Ident(ref i) => {
                                    let (mut postfix_list, tokens_after_next) = get_parsed!(parse_postfix_continuation(next_rest));
                                    postfix_list.insert(0, Postfix::Access(i.clone()));
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

// <args>
fn parse_expr_list<'a>(tokens: &'a[Tok]) -> Result<'a, Vec<Expr>> {
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
                                    let (mut parsed_list, tokens_after_arg) = get_parsed!(parse_expr_list(rest));
                                    parsed_list.insert(0, parsed_expr);
                                    Result::Ok(parsed_list, tokens_after_arg)
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
fn parse_unary_expr<'a>(tokens: &'a[Tok]) -> Result<'a, Expr> {
    match tokens {
        [ref first_tok, rest..] => {
            match first_tok.token {
                // "-" ...
                Token::Sub => {
                    let (parsed_expr, tokens_after_expr) = get_parsed!(parse_unary_expr(rest));
                    Result::Ok(Expr::UnOp(UnOp::Neg, Box::new(parsed_expr)), tokens_after_expr)
                },
                // "!" ...
                Token::Not => {
                    let (parsed_expr, tokens_after_expr) = get_parsed!(parse_unary_expr(rest));
                    Result::Ok(Expr::UnOp(UnOp::Not, Box::new(parsed_expr)), tokens_after_expr)
                },
                // <postfix-expr>
                _ => parse_postfix_expr(tokens)
            }
        }
        _ => Result::Err(format!("Expected expression"))
    }
}

// <exponential-expr>
fn parse_exponential_expr<'a>(tokens: &'a[Tok]) -> Result<'a, Expr> {
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
fn parse_multiplicative_expr<'a>(tokens: &'a[Tok]) -> Result<'a, Expr> {
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
fn parse_additive_expr<'a>(tokens: &'a[Tok]) -> Result<'a, Expr> {
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
fn parse_in_expr<'a>(tokens: &'a[Tok]) -> Result<'a, Expr> {
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
fn parse_relational_expr<'a>(tokens: &'a[Tok]) -> Result<'a, Expr> {
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
fn parse_equality_expr<'a>(tokens: &'a[Tok]) -> Result<'a, Expr> {
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
fn parse_and_expr<'a>(tokens: &'a[Tok]) -> Result<'a, Expr> {
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
fn parse_or_expr<'a>(tokens: &'a[Tok]) -> Result<'a, Expr> {
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
fn parse_expr<'a>(tokens: &'a[Tok]) -> Result<'a, Expr> {
    parse_or_expr(tokens)
}



// <item>
fn parse_item<'a>(tokens: &'a[Tok]) -> Result<'a, StmtItem> {
    match tokens {
        [ref first_tok, rest..] => {
            match first_tok.token {
                // "var" <ident>
                Token::Var => {
                    match rest {
                        [ref next_tok, rest..] => {
                            match next_tok.token{
                                Token::Ident(ref id) => {
                                    Result::Ok(StmtItem::Var(id.clone()), rest)
                                }
                                _ => Result::Err(format!(
                                    "PARSING FAILURE at {},{}: Expected Ident but found {:?}\n{}\n{}",
                                    next_tok.line + 1,
                                    next_tok.col + 1,
                                    next_tok.token,
                                    next_tok.line_string,
                                    get_caret_string(next_tok.col)
                                ))
                            }
                        }
                        _ => Result::Err(format!("PARSING FAILURE: Reached end of file but expected an ident"))
                    }
                }
                // "def" <expr>
                Token::Def => {
                    let (parsed_expr, tokens_after_expr) = get_parsed!(parse_expr(rest));
                    Result::Ok(StmtItem::Def(Box::new(parsed_expr)), tokens_after_expr)
                }
                // <expr>
                _ => {
                    let (parsed_expr, tokens_after_expr) = get_parsed!(parse_expr(tokens));
                    Result::Ok(StmtItem::Bare(Box::new(parsed_expr)), tokens_after_expr)
                }
            }
        }
        _ => Result::Err(format!("PARSING FAILURE: Reached end of file but expected a statement"))
    }
}

// <item-list>
fn parse_item_list<'a>(tokens: &'a[Tok]) -> Result<'a, Vec<StmtItem>> {
    let (parsed_item, tokens_after_item) = get_parsed!(parse_item(tokens));
    match tokens_after_item {
        [ref first_tok, rest..] => {
            match first_tok.token {
                // ... "," <item-list>
                Token::Comma => {
                    let (mut parsed_list, tokens_after_list) = get_parsed!(parse_item_list(rest));
                    parsed_list.insert(0, parsed_item);
                    Result::Ok(parsed_list, tokens_after_list)
                }
                // EPS
                _ => Result::Ok(vec![parsed_item], tokens_after_item)
            }
        }
        _ => Result::Ok(vec![parsed_item], tokens_after_item)
    }
}

// <stmt-items>
fn parse_stmt_items<'a>(tokens: &'a[Tok]) -> Result<'a, Stmt> {
    let (parsed_items, tokens_after_items) = get_parsed!(parse_item_list(tokens));
    match tokens_after_items {
        [ref first_tok, rest..] => {
            match first_tok.token {
                // ... ":" <expr>
                Token::Assign => {
                    let (parsed_expr, tokens_after_expr) = get_parsed!(parse_expr(rest));
                    Result::Ok(Stmt::Assignment(parsed_items, Box::new(parsed_expr)), tokens_after_expr)
                }
                // EPS
                _ => Result::Ok(Stmt::Bare(parsed_items), tokens_after_items)
            }
        }
        _ => Result::Ok(Stmt::Bare(parsed_items), tokens_after_items)
    }
}

// <stmt>
fn parse_stmt<'a>(tokens: &'a[Tok]) -> Result<'a, Stmt> {
    match tokens {
        [ref first_tok, rest..] => {
            match first_tok.token {
                // "if" <expr> <if-statements>
                Token::If => {
                    let (parsed_expr, tokens_after_expr) = get_parsed!(parse_expr(rest));
                    let ((if_list, else_list), tokens_after_if) = get_parsed!(parse_if_statements(tokens_after_expr));
                    Result::Ok(Stmt::If(Box::new(parsed_expr), if_list, else_list), tokens_after_if)
                }
                // "while" <expr> <block-statements>
                Token::While => {
                    let (parsed_expr, tokens_after_expr) = get_parsed!(parse_expr(rest));
                    let (stmt_list, tokens_after_list) = get_parsed!(parse_block_statements(tokens_after_expr));
                    Result::Ok(Stmt::While(Box::new(parsed_expr), stmt_list), tokens_after_list)

                }
                // "return"
                Token::Return => Result::Ok(Stmt::Return, rest),
                // <stmt-items>
                _ => parse_stmt_items(tokens)
            }
        }
        _ => Result::Err(format!("PARSING FAILURE: Reached end of file but expected a statement"))
    }
}

// <ident-list>
fn parse_ident_list<'a>(tokens: &'a[Tok]) -> Result<'a, Vec<Box<String>>> {
    match tokens {
        [ref first_tok, rest..] => {
            match first_tok.token {
                // <ident> ...
                Token::Ident(ref id) => {
                    match rest {
                        [ref next_tok, next_rest..] => {
                            match next_tok.token {
                                // ... ")"
                                Token::CloseParen => Result::Ok(vec![id.clone()], next_rest),
                                // ... "," <ident-list>
                                Token::Comma => {
                                    let (mut parsed_list, tokens_after_list) = get_parsed!(parse_ident_list(next_rest));
                                    parsed_list.insert(0, id.clone());
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
fn parse_params<'a>(tokens: &'a[Tok]) -> Result<'a, Vec<Box<String>>> {
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
fn parse_rets<'a>(tokens: &'a[Tok]) -> Result<'a, Vec<Box<String>>> {
    match tokens {
        [ref first_tok, rest..] => {
            match first_tok.token {
                // "(" <ident-list>
                Token::OpenParen => parse_ident_list(rest),
                // <ident>
                Token::Ident(ref id) => Result::Ok(vec![id.clone()], rest),
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
fn parse_clip_def<'a>(tokens: &'a[Tok]) -> Result<'a, (Vec<Box<String>>, Vec<Box<String>>, Vec<Stmt>)> {
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

// <if-statements>
fn parse_if_statements<'a>(tokens: &'a[Tok]) -> Result<'a, (Vec<Stmt>, Vec<Stmt>)> {
    match tokens {
        [ref first_tok, rest..] => {
            match first_tok.token {
                // "else" <block-statements>
                Token::Else => {
                    let (parsed_list, tokens_after_list) = get_parsed!(parse_block_statements(rest));
                    Result::Ok((vec![], parsed_list), tokens_after_list)
                }
                // "end"
                Token::End => Result::Ok((vec![], vec![]), rest),
                // <stmt> <if-statements>
                _ => {
                    let (parsed_stmt, tokens_after_stmt) = get_parsed!(parse_stmt(tokens));
                    let ((mut if_list, else_list), tokens_after_if) = get_parsed!(parse_if_statements(tokens_after_stmt));
                    if_list.insert(0, parsed_stmt);
                    Result::Ok((if_list, else_list), tokens_after_if)
                }
            }
        }
        _ => Result::Err(format!("PARSING FAILURE: Reached end of file but expected 'end'"))
    }
}

// <block-statements>
fn parse_block_statements<'a>(tokens: &'a[Tok]) -> Result<'a, Vec<Stmt>> {
    match tokens {
        [ref first_tok, rest..] => {
            match first_tok.token {
                // "end"
                Token::End => Result::Ok(vec![], rest),
                // <stmt> <block-statements>
                _ => {
                    let (parsed_stmt, tokens_after_stmt) = get_parsed!(parse_stmt(tokens));
                    let (mut parsed_list, tokens_after_list) = get_parsed!(parse_block_statements(tokens_after_stmt));
                    parsed_list.insert(0, parsed_stmt);
                    Result::Ok(parsed_list, tokens_after_list)
                }
            }
        }
        _ => Result::Err(format!("PARSING FAILURE: Reached end of file but expected 'end'"))
    }
}

// <clip-statements>
fn parse_clip_statements<'a>(tokens: &'a[Tok]) -> Result<'a, Vec<Stmt>> {
    match tokens {
        [ref first_tok, rest..] => {
            match first_tok.token {
                // "}"
                Token::CloseCurly => Result::Ok(vec![], rest),
                // <stmt> <clip-statements>
                _ => {
                    let (parsed_stmt, tokens_after_stmt) = get_parsed!(parse_stmt(tokens));
                    let (mut parsed_list, tokens_after_list) = get_parsed!(parse_clip_statements(tokens_after_stmt));
                    parsed_list.insert(0, parsed_stmt);
                    Result::Ok(parsed_list, tokens_after_list)
                }
            }
        }
        _ => Result::Err(format!("PARSING FAILURE: Reached end of file but expected '}}'"))
    }
}

// <base-statements>
pub fn parse_base_statements<'a>(tokens: &'a[Tok]) -> Result<'a, Vec<Stmt>> {
    match tokens {
        // <stmt> <base-statements>
        [_, _..] => {
            let (parsed_stmt, tokens_after_stmt) = get_parsed!(parse_stmt(tokens));
            let (mut parsed_list, tokens_after_list) = get_parsed!(parse_base_statements(tokens_after_stmt));
            parsed_list.insert(0, parsed_stmt);
            Result::Ok(parsed_list, tokens_after_list)
        }
        // ""
        _ => Result::Ok(vec![], tokens)
    }
}
