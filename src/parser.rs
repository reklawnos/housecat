use token;
use ast::test_ast;

pub fn parse_expr(tokens: &[token::Tok]) -> (test_ast::Expr, &[token::Tok]) {
    match parse_term(tokens) {
        (parsed_term, tokens_after_term) => {
            match tokens_after_term {
                // ... + <Expr>
                [token::Tok{token: token::Add, line: _, col: _}, ..tokens_after_plus] => {
                    match parse_expr(tokens_after_plus) {
                        (parsed_expr, tokens_after_expr) =>
                            (test_ast::BinOpExpr(test_ast::BinAdd, box parsed_term, box parsed_expr), tokens_after_expr),
                    }
                },
                // ... - <Expr>
                [token::Tok{token: token::Sub, line: _, col: _}, ..tokens_after_minus] => {
                    match parse_expr(tokens_after_minus) {
                        (parsed_expr, tokens_after_expr) =>
                            (test_ast::BinOpExpr(test_ast::BinSub, box parsed_term, box parsed_expr), tokens_after_expr),
                    }
                },
                // <Term>
                _ => (parsed_term, tokens_after_term),
            }
        }
    }
}

fn parse_term(tokens: &[token::Tok]) -> (test_ast::Expr, &[token::Tok]) {
    match parse_factor(tokens) {
        (parsed_factor, tokens_after_factor) => {
            match tokens_after_factor {
                // ... * <Term>
                [token::Tok{token: token::Mul, line: _, col: _}, ..tokens_after_mul] => {
                    match parse_term(tokens_after_mul) {
                        (parsed_term, tokens_after_term) =>
                            (test_ast::BinOpExpr(test_ast::BinMul, box parsed_factor, box parsed_term), tokens_after_term),
                    }
                },
                // ... / <Term>
                [token::Tok{token: token::Div, line: _, col: _}, ..tokens_after_div] => {
                    match parse_term(tokens_after_div) {
                        (parsed_term, tokens_after_term) => {
                            (test_ast::BinOpExpr(test_ast::BinDiv, box parsed_factor, box parsed_term), tokens_after_term)
                        },
                    }
                },
                // <Factor>
                _ => (parsed_factor, tokens_after_factor),
            }
        }
    }
}

fn parse_factor(tokens: &[token::Tok]) -> (test_ast::Expr, &[token::Tok]) {
    match tokens {
        // <ident>
        [token::Tok{token: token::Ident(ref id), line: _, col: _}, ..tokens_after_ident] => {
            (test_ast::IdExpr(id.clone()), tokens_after_ident)
        },
        // ... ( ...
        [token::Tok{token: token::OpenParen, line: _, col: _}, ..tokens_after_openparen] => {
            match parse_expr(tokens_after_openparen) {
                (parsed_expr, tokens_after_expr) => {
                    match tokens_after_expr {
                        // ... )
                        [token::Tok{token: token::CloseParen, line: _, col: _}, ..tokens_after_closeparen] => {
                            (parsed_expr, tokens_after_closeparen)
                        },
                        _ => fail!("no matching paren")
                    }
                }
            }
        },
        _ => fail!("unrecognized symbol or something")
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
