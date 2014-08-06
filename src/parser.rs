use token;
use ast::Ast;


fn parse_primary_expr(tokens: &[token::Tok]) -> (Ast::Expr, &[token::Tok]) {
    match tokens {
        [ref first_tok, ..rest] => {
            match first_tok.token {
                // <Id>
                token::Ident(ref id) => (Ast::ExprIdent(id.clone()), rest),
                // ( <Expr> ...
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
                                "Reached end of file, but the paren at {},{} is unmatched",
                                first_tok.line + 1,
                                first_tok.col + 1
                            )
                    }
                },
                _ => fail!(
                        "ERROR at {},{}: Found {} but Expected Ident or '('",
                        first_tok.line + 1,
                        first_tok.col + 1,
                        first_tok.token
                    )
            }
        }
        _ => fail!("Expected Ident or (Expression)")
    }
}

fn parse_postfix_expr(tokens: &[token::Tok]) -> (Ast::Expr, &[token::Tok]) {
    //TODO: add postfix expression parsing
    parse_primary_expr(tokens)
}

fn parse_unary_expr(tokens: &[token::Tok]) -> (Ast::Expr, &[token::Tok]) {
    //TODO: add unary expression parsing
    parse_postfix_expr(tokens)
}

fn parse_exponential_expr(tokens: &[token::Tok]) -> (Ast::Expr, &[token::Tok]) {
    //TODO: add exponential expression parsing
    parse_unary_expr(tokens)
}

fn parse_multiplicative_expr(tokens: &[token::Tok]) -> (Ast::Expr, &[token::Tok]) {
    // <Factor> ...
    let (parsed_factor, tokens_after_factor) = parse_exponential_expr(tokens);
    match tokens_after_factor {
        [ref next_tok, ..rest] => {
            match next_tok.token {
                // ... * <Term>
                token::Mul => {
                    let (parsed_term, tokens_after_term) = parse_multiplicative_expr(rest);
                    (build_bin_op(Ast::BinMul, parsed_factor, parsed_term), tokens_after_term)
                },
                // ... / <Term>
                token::Div => {
                    let (parsed_term, tokens_after_term) = parse_multiplicative_expr(rest);
                    (build_bin_op(Ast::BinDiv, parsed_factor, parsed_term), tokens_after_term)
                },
                // <Factor>
                _ => (parsed_factor, tokens_after_factor),
            }
        }
        // <Factor>
        _ => (parsed_factor, tokens_after_factor),
    }
}

fn parse_additive_expr(tokens: &[token::Tok]) -> (Ast::Expr, &[token::Tok]) {
    // <Term> ...
    let (parsed_term, tokens_after_term) = parse_multiplicative_expr(tokens);
    match tokens_after_term {
        [ref next_tok, ..rest] => {
            match next_tok.token {
                // ... + <Expr>
                token::Add => {
                    let (parsed_expr, tokens_after_expr) = parse_additive_expr(rest);
                    (build_bin_op(Ast::BinAdd, parsed_term, parsed_expr), tokens_after_expr)
                },
                // ... - <Expr>
                token::Sub => {
                    let (parsed_expr, tokens_after_expr) = parse_additive_expr(rest);
                    (build_bin_op(Ast::BinSub, parsed_term, parsed_expr), tokens_after_expr)
                },
                // <Term>
                _ => (parsed_term, tokens_after_term),
            }
        }
        // <Term>
        _ => (parsed_term, tokens_after_term),
    }
}

fn parse_relational_expr(tokens: &[token::Tok]) -> (Ast::Expr, &[token::Tok]) {
    //TODO: add relational expression parsing
    parse_additive_expr(tokens)
}

fn parse_equality_expr(tokens: &[token::Tok]) -> (Ast::Expr, &[token::Tok]) {
    //TODO: add equality expression parsing
    parse_relational_expr(tokens)
}

fn parse_and_expr(tokens: &[token::Tok]) -> (Ast::Expr, &[token::Tok]) {
    //TODO: add 'and' expression parsing
    parse_equality_expr(tokens)
}

fn parse_or_expr(tokens: &[token::Tok]) -> (Ast::Expr, &[token::Tok]) {
    //TODO: add 'or' expression parsing
    parse_and_expr(tokens)
}

pub fn parse_expr(tokens: &[token::Tok]) -> (Ast::Expr, &[token::Tok]) {
    //TODO: add steps before additivex
    parse_or_expr(tokens)
}

fn build_bin_op(op: Ast::BinOp, lhs: Ast::Expr, rhs: Ast::Expr) -> Ast::Expr {
    Ast::ExprBinOp(op, box lhs, box rhs)
}
