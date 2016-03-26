use token::{Token, Tok};
use ast::*;
use parser::expr::parse_expr;
use parser::clip::parse_rets;
use parser::{ParseResult, ParserError, ParserErrorType};

// <item>
fn parse_item<'a>(tokens: &'a[Tok]) -> ParseResult<'a, StmtItem<'a>> {
    match tokens {
        // "var" <ident>
        [Tok{token: Token::Var, ..}, rest..] => {
            match rest {
                [Tok{token: Token::Ident(id), ..}, rest..]=> Ok((StmtItem::Var(id), rest)),
                [ref tok, ..] => Err(ParserError{
                    actual: tok.clone(),
                    error_type: ParserErrorType::ExpectedIdent,
                    hint: None
                }),
                [] => panic!("Missing EOF")
            }
        }
        // "var" <ident>
        [Tok{token: Token::Let, ..}, rest..] => {
            match rest {
                [Tok{token: Token::Ident(id), ..}, rest..]=> Ok((StmtItem::Let(id), rest)),
                [ref tok, ..] => Err(ParserError{
                    actual: tok.clone(),
                    error_type: ParserErrorType::ExpectedIdent,
                    hint: None
                }),
                [] => panic!("Missing EOF")
            }
        }
        // "@" <expr>
        [Tok{token: Token::ExprDef, ..}, rest..] => {
            let (parsed_expr, tokens_after_expr) = try!(parse_expr(rest));
            Ok((StmtItem::Expr(Box::new(parsed_expr)), tokens_after_expr))
        }
        // <expr>
        [Tok{token: _, ..}, ..] => {
            let (parsed_expr, tokens_after_expr) = try!(parse_expr(tokens));
            Ok((StmtItem::Bare(Box::new(parsed_expr)), tokens_after_expr))
        }
        [] => panic!("Missing EOF")
    }
}

// <item-list>
fn parse_item_list<'a>(tokens: &'a[Tok]) -> ParseResult<'a, Vec<StmtItem<'a>>> {
    let (parsed_item, tokens_after_item) = try!(parse_item(tokens));
    match tokens_after_item {
        // ... "," <item-list>
        [Tok{token: Token::Comma, ..}, rest..] => {
            let (mut parsed_list, tokens_after_list) = try!(parse_item_list(rest));
            parsed_list.insert(0, parsed_item);
            Ok((parsed_list, tokens_after_list))
        }
        // EPS
        _ => Ok((vec![parsed_item], tokens_after_item))
    }
}

// <stmt-items>
fn parse_stmt_items<'a>(tokens: &'a[Tok]) -> ParseResult<'a, Stmt<'a>> {
    let (parsed_items, tokens_after_items) = try!(parse_item_list(tokens));
    match tokens_after_items {
        // ... ":" <expr>
        [Tok{token: Token::Def, line, ..}, rest..] => {
            let (parsed_expr, tokens_after_expr) = try!(parse_expr(rest));
            Ok((Stmt{stmt: StmtType::Def{items: parsed_items, expr: Box::new(parsed_expr)},
                     data: AstData{line: line}}, tokens_after_expr))
        }
        // ... "=" <expr>
        [Tok{token: Token::Assign, line, ..}, rest..] => {
            let (parsed_expr, tokens_after_expr) = try!(parse_expr(rest));
            Ok((Stmt{stmt: StmtType::Assign{items: parsed_items, expr: Box::new(parsed_expr)},
                     data: AstData{line: line}}, tokens_after_expr))
        }
        // EPS
        [Tok{line, ..}, ..] => {
            Ok((Stmt{stmt: StmtType::Bare{items: parsed_items},
                     data: AstData{line: line}}, tokens_after_items))
        }
        [] => panic!("Missing EOF")
    }
}

// <stmt>
fn parse_stmt<'a>(tokens: &'a[Tok]) -> ParseResult<'a, Stmt<'a>> {
    match tokens {
        [ref start_tok, rest..] => {
            match start_tok.token {
                // "if" <expr> <if-statements>
                Token::If => {
                    let (parsed_expr, tokens_after_expr) = try!(parse_expr(rest));
                    match tokens_after_expr {
                        [Tok{token: Token::Do, ..}, rest..] => {
                            let (clauses, tokens_after_if) = {
                                try!(parse_if_statements(rest, parsed_expr))
                            };
                            Ok((Stmt{stmt: StmtType::If{clauses: clauses},
                                     data: AstData{line: start_tok.line}}, tokens_after_if))
                        }
                        [ref tok, ..] => Err(ParserError{
                            actual: tok.clone(),
                            error_type: ParserErrorType::ExpectedMatchingToken{
                                //TODO: should keep track of do-end errors
                                expected: Token::Do,
                                start_tok: start_tok.clone()
                            },
                            hint: Some("`if` statements must include a `do ... end` block")
                        }),
                        [] => panic!("Missing EOF")
                    }
                }
                // "while" <expr> <block-statements>
                Token::While => {
                    let (parsed_expr, tokens_after_expr) = try!(parse_expr(rest));
                    match tokens_after_expr {
                        [Tok{token: Token::Do, ..}, rest..] => {
                            let (stmt_list, tokens_after_list) = try!(parse_block_statements(rest));
                            Ok((Stmt{stmt: StmtType::While{condition: Box::new(parsed_expr),
                                                           statements: stmt_list},
                                     data: AstData{line: start_tok.line}}, tokens_after_list))
                        }
                        [ref tok, ..] => Err(ParserError{
                            actual: tok.clone(),
                            error_type: ParserErrorType::ExpectedMatchingToken{
                                //TODO: should keep track of do-end errors
                                expected: Token::Do,
                                start_tok: start_tok.clone()
                            },
                            hint: Some("`while` statements must include a `do ... end` block")
                        }),
                        [] => panic!("Missing EOF")
                    }
                }
                // "for" <rets> "in" <expr> <block-statements>
                Token::For => {
                    let (parsed_rets, tokens_after_rets) = try!(parse_rets(rest));
                    match tokens_after_rets {
                        [ref in_tok, rest..] if in_tok.token == Token::In => {
                            let (parsed_expr, tokens_after_expr) = try!(parse_expr(rest));
                            match tokens_after_expr {
                                [Tok{token: Token::Do, ..}, rest..] => {
                                    let (stmt_list, tokens_after_list) = try!(parse_block_statements(rest));
                                    Ok((Stmt{stmt: StmtType::For{idents: parsed_rets,
                                                                 iterator: Box::new(parsed_expr),
                                                                 statements: stmt_list},
                                             data: AstData{line: start_tok.line}}, tokens_after_list))
                                }
                                [ref tok, ..] => Err(ParserError{
                                    actual: tok.clone(),
                                    error_type: ParserErrorType::ExpectedMatchingToken{
                                        //TODO: should keep track of do-end errors
                                        expected: Token::Do,
                                        start_tok: in_tok.clone()
                                    },
                                    hint: Some("`for` statements must include a `do ... end` block")
                                }),
                                [] => panic!("Missing EOF")
                            }
                        }
                        [ref tok, ..] => Err(ParserError{
                            actual: tok.clone(),
                            error_type: ParserErrorType::ExpectedMatchingToken{
                                //TODO
                                expected: Token::In,
                                start_tok: start_tok.clone()
                            },
                            hint: Some("`for` statements must include an `in <expression>` block")
                        }),
                        [] => panic!("Missing EOF")
                    }
                }
                // "return"
                Token::Return => {
                    Ok((Stmt{stmt: StmtType::Return, data: AstData{line: start_tok.line}}, rest))
                }
                // <stmt-items>
                _ => parse_stmt_items(tokens),
            }
        }
        [] => panic!("Missing EOF")
    }
}

// <if-statements>
fn parse_if_statements<'a>(tokens: &'a[Tok],
                           expr: Expr<'a>) -> ParseResult<'a, Vec<IfClause<'a>>> {
    let mut statements = vec![];
    let mut clauses = vec![];
    let mut my_toks = tokens;
    let mut my_expr = expr;
    while my_toks.len() > 0 {
        let tok = &my_toks[0];
        match tok.token {
            // "else" <block-statements>
            Token::Else => {
                clauses.push(IfClause::If{condition: Box::new(my_expr), statements: statements});
                let (parsed_list, tokens_after_list) = try!(parse_block_statements(&my_toks[1..]));
                clauses.push(IfClause::Else(parsed_list));
                return Ok((clauses, tokens_after_list));
            }
            // "elif" <expr> <if-statements>
            Token::Elif => {
                clauses.push(IfClause::If{condition: Box::new(my_expr), statements: statements});
                statements = Vec::new();
                let (parsed_expr, tokens_after_expr) = try!(parse_expr(&my_toks[1..]));
                match tokens_after_expr {
                    [Tok{token: Token::Do, ..}, rest..] => {
                        my_toks = rest;
                        my_expr = parsed_expr;
                    }
                    [ref actual_tok, ..] => {return Err(ParserError{
                        actual: actual_tok.clone(),
                        error_type: ParserErrorType::ExpectedMatchingToken{
                            expected: Token::Do,
                            start_tok: actual_tok.clone()
                        },
                        hint: Some("`elif` blocks must include a `do ... end` block")
                    });},
                    [] => panic!("Missing EOF")
                };
            }
            // "end"
            Token::End => {
                clauses.push(IfClause::If{condition: Box::new(my_expr), statements: statements});
                return Ok((clauses, &my_toks[1..]))
            }
            Token::Eof => {return Err(ParserError{
                actual: tok.clone(),
                error_type: ParserErrorType::ExpectedTokens{
                    expected: vec!(Token::End),
                },
                hint: Some("blocks be closed with `end`")
            });},
            // <stmt> <if-statements>
            _ => {
                let (parsed_stmt, tokens_after_stmt) = try!(parse_stmt(my_toks));
                my_toks = tokens_after_stmt;
                statements.push(parsed_stmt);
            }
        }
    }
    panic!("Missing EOF")
}

// <block-statements>
fn parse_block_statements<'a>(tokens: &'a[Tok]) -> ParseResult<'a, Vec<Stmt<'a>>> {
    let mut statements = vec![];
    let mut my_toks = tokens;
    while my_toks.len() > 0 {
        let tok = &my_toks[0];
        match tok.token {
            // "end"
            Token::End => {return Ok((statements, &my_toks[1..]))}
            Token::Eof => {return Err(ParserError{
                actual: tok.clone(),
                error_type: ParserErrorType::ExpectedTokens{
                    expected: vec!(Token::End),
                },
                hint: Some("blocks must be closed with `end`")
            });},
            // <stmt> <block-statements>
            _ => {
                let (parsed_stmt, tokens_after_stmt) = try!(parse_stmt(my_toks));
                statements.push(parsed_stmt);
                my_toks = tokens_after_stmt;
            }
        }
    }
    panic!("Missing EOF")
}

// <clip-statements>
pub fn parse_clip_statements<'a>(tokens: &'a[Tok]) -> ParseResult<'a, Vec<Stmt<'a>>> {
    let mut statements = vec![];
    let mut my_toks = tokens;
    while my_toks.len() > 0 {
        let tok = &my_toks[0];
        match tok.token {
            // "}"
            Token::CloseCurly => {return Ok((statements, &my_toks[1..]))}
            Token::Eof => {return Err(ParserError{
                actual: tok.clone(),
                error_type: ParserErrorType::ExpectedTokens{
                    expected: vec!(Token::CloseCurly),
                },
                hint: Some("clip blocks must be closed with `}`")
            });},
            // <stmt> <clip-statements>
            _ => {
                let (parsed_stmt, tokens_after_stmt) = try!(parse_stmt(my_toks));
                statements.push(parsed_stmt);
                my_toks = tokens_after_stmt;
            }
        }
    }
    panic!("Missing EOF")
}

// <base-statements>
pub fn parse_base_statements<'a>(tokens: &'a[Tok],
                                 cur_statements: &'a mut Vec<Stmt<'a>>)
                                 -> ParseResult<'a, &'a Vec<Stmt<'a>>> {
    let mut my_toks = tokens;
    while my_toks.len() > 1 {
        let (parsed_stmt, tokens_after_stmt) = try!(parse_stmt(my_toks));
        cur_statements.push(parsed_stmt);
        my_toks = tokens_after_stmt;
    }
    Ok((cur_statements, my_toks))
}
