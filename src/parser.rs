use token;
use ast::test_ast;


pub fn parse_expr(tokens: &[token::Tok]) -> (test_ast::Expr, &[token::Tok]) {
    // <Term> ...
    let (parsed_term, tokens_after_term) = parse_term(tokens);
    match tokens_after_term {
        [ref next_tok, ..rest] => {
            match next_tok.token {
                // ... + <Expr>
                token::Add => {
                    let (parsed_expr, tokens_after_expr) = parse_expr(rest);
                    (
                        test_ast::BinOpExpr(
                            test_ast::BinAdd,
                            box parsed_term,
                            box parsed_expr
                        ),
                        tokens_after_expr
                    )
                },
                // ... - <Expr>
                token::Sub => {
                    let (parsed_expr, tokens_after_expr) = parse_expr(rest);
                    (
                        test_ast::BinOpExpr(
                            test_ast::BinSub,
                            box parsed_term,
                            box parsed_expr
                        ),
                        tokens_after_expr
                    )
                },
                // <Term>
                _ => (parsed_term, tokens_after_term),
            }
        }
        // <Term>
        _ => (parsed_term, tokens_after_term),
    }
}


fn parse_term(tokens: &[token::Tok]) -> (test_ast::Expr, &[token::Tok]) {
    // <Factor> ...
    let (parsed_factor, tokens_after_factor) = parse_factor(tokens);
    match tokens_after_factor {
        [ref next_tok, ..rest] => {
            match next_tok.token {
                // ... * <Term>
                token::Mul => {
                    let (parsed_term, tokens_after_term) = parse_term(rest);
                    (
                        test_ast::BinOpExpr(
                            test_ast::BinMul,
                            box parsed_factor,
                            box parsed_term
                        ),
                        tokens_after_term
                    )
                },
                // ... / <Term>
                token::Div => {
                    let (parsed_term, tokens_after_term) = parse_term(rest);
                    (
                        test_ast::BinOpExpr(
                            test_ast::BinDiv,
                            box parsed_factor,
                            box parsed_term
                        ),
                        tokens_after_term
                    )
                },
                // <Factor>
                _ => (parsed_factor, tokens_after_factor),
            }
        }
        // <Factor>
        _ => (parsed_factor, tokens_after_factor),
    }
}

fn parse_factor(tokens: &[token::Tok]) -> (test_ast::Expr, &[token::Tok]) {
    match tokens {
        [ref first_tok, ..rest] => {
            match first_tok.token {
                // <Id>
                token::Ident(ref id) => (test_ast::IdExpr(id.clone()), rest),
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


pub fn print_expr(e: &Box<test_ast::Expr>, ind: uint) {
    let mut indent = String::from_str("");
    indent.grow(ind, ' ');
    match **e {
        test_ast::BinOpExpr(op, ref lhs, ref rhs) => {
            let opstring = match op {
                test_ast::BinAdd => "+",
                test_ast::BinSub => "-",
                test_ast::BinMul => "*",
                test_ast::BinDiv => "/"
            };
            print!("{}BinOpExpr(\n", indent);
            print_expr(&*lhs, ind + 4);
            println!("    {}{}", indent, opstring);
            print_expr(&*rhs, ind + 4);
            print!("{})\n", indent);
        },
        test_ast::IdExpr(ref s) => {
            print!("{}IdExpr({})\n", indent, s);
        }
    }
}
