use token;
use ast::test_ast;

pub fn parse_expr(tokens: &[token::Token]) -> (test_ast::Expr, &[token::Token]) {
    match parse_term(tokens) {
        (parsed_term, tokens_after_term) => {
            match tokens_after_term {
                // ... + <Expr>
                [token::Add, ..tokens_after_plus] => {
                    match parse_expr(tokens_after_plus) {
                        (parsed_expr, tokens_after_expr) =>
                            (test_ast::PlusExpr(box parsed_term, box parsed_expr), tokens_after_expr),
                    }
                },
                // ... - <Expr>
                [token::Sub, ..tokens_after_minus] => {
                    match parse_expr(tokens_after_minus) {
                        (parsed_expr, tokens_after_expr) =>
                            (test_ast::MinusExpr(box parsed_term, box parsed_expr), tokens_after_expr),
                    }
                },
                // <Term>
                _ => (test_ast::TermAsExpr(box parsed_term), tokens_after_term),
            }
        }
    }
}

fn parse_term(tokens: &[token::Token]) -> (test_ast::Term, &[token::Token]) {
    match parse_factor(tokens) {
        (parsed_factor, tokens_after_factor) => {
            match tokens_after_factor {
                // ... * <Term>
                [token::Mul, ..tokens_after_mul] => {
                    match parse_term(tokens_after_mul) {
                        (parsed_term, tokens_after_term) =>
                            (test_ast::MultTerm(box parsed_factor, box parsed_term), tokens_after_term),
                    }
                },
                // ... / <Term>
                [token::Div, ..tokens_after_div] => {
                    match parse_term(tokens_after_div) {
                        (parsed_term, tokens_after_term) => {
                            (test_ast::DivTerm(box parsed_factor, box parsed_term), tokens_after_term)
                        },
                    }
                },
                // <Factor>
                _ => (test_ast::FactorAsTerm(box parsed_factor), tokens_after_factor),
            }
        }
    }
}

fn parse_factor(tokens: &[token::Token]) -> (test_ast::Factor, &[token::Token]) {
    match tokens {
        // <ident>
        [token::Ident(ref id), ..tokens_after_ident] => {
            (test_ast::Id(id.clone()), tokens_after_ident)
        },
        // ... ( ...
        [token::OpenParen, ..tokens_after_openparen] => {
            match parse_expr(tokens_after_openparen) {
                (parsed_expr, tokens_after_expr) => {
                    match tokens_after_expr {
                        // ... )
                        [token::CloseParen, ..tokens_after_closeparen] => {
                            (test_ast::ParenthesizedExpr(box parsed_expr), tokens_after_closeparen)
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
        test_ast::TermAsExpr(ref term) => {
            print!("{}TermAsExpr(\n", indent);
            print_term(term, ind + 4);
            print!("{})\n", indent);
        },
        test_ast::PlusExpr(ref term, ref expr) => {
            print!("{}PlusExpr(\n", indent);
            print_term(&*term, ind + 4);
            println!("    {}+", indent);
            print_expr(&*expr, ind + 4);
            print!("{})\n", indent);
        },
        test_ast::MinusExpr(ref term, ref expr) => {
            print!("{}MinusExpr(\n", indent);
            print_term(&*term, ind + 4);
            println!("    {}-", indent);
            print_expr(&*expr, ind + 4);
            print!("{})\n", indent);
        }
    }
}

fn print_term(e: &Box<test_ast::Term>, ind: uint) {
    let mut indent = String::from_str("");
    indent.grow(ind, ' ');
    match **e {
        test_ast::FactorAsTerm(ref factor) => {
            print!("{}FactorAsTerm(\n", indent);
            print_factor(factor, ind + 4);
            print!("{})\n", indent);
        },
        test_ast::MultTerm(ref factor, ref term) => {
            print!("{}MultTerm(\n", indent);
            print_factor(factor, ind + 4);
            println!("    {}*", indent);
            print_term(term, ind + 4);
            print!("{})\n", indent);
        },
        test_ast::DivTerm(ref factor, ref term) => {
            print!("{}DivTerm(\n", indent);
            print_factor(&*factor, ind + 4);
            println!("    {}/", indent);
            print_term(&*term, ind + 4);
            print!("{})\n", indent);
        }
    }
}

fn print_factor(e: &Box<test_ast::Factor>, ind: uint) {
    let mut indent = String::from_str("");
    indent.grow(ind, ' ');
    match **e {
        test_ast::Id(ref s) => {
            print!("{}Id({})\n", indent, s);
        },
        test_ast::ParenthesizedExpr(ref expr) => {
            print!("{}ParenthesizedExpr(\n", indent);
            print_expr(expr, ind + 4);
            print!("{})\n", indent);
        }
    }
}
