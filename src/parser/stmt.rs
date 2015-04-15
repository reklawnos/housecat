use token::{Token, Tok};
use ast::*;
use utils::*;
use parser::expr::parse_expr;
use parser::ParseResult;

// <item>
fn parse_item<'a>(tokens: &'a[Tok]) -> ParseResult<'a, StmtItem<'a>> {
    match tokens {
        // "var" <ident>
        [Tok{token: Token::Var, ..}, rest..] => {
            match rest {
                [Tok{token: Token::Ident(id), ..}, rest..]=> Ok((StmtItem::Var(id), rest)),
                [Tok{ref token, line, col, line_string, ..}, ..] => Err(format!(
                    "PARSING FAILURE at {},{}: Expected Ident but found {:?}\n{}\n{}",
                    line + 1,
                    col + 1,
                    token,
                    line_string,
                    get_caret_string(col)
                )),
                [] => Err(format!("PARSING FAILURE: Reached end of file but expected an ident"))
            }
        }
        // "def" <ident>
        [Tok{token: Token::Def, ..}, rest..] => {
            match rest {
                [Tok{token: Token::Ident(id), ..}, rest..]=> Ok((StmtItem::Def(id), rest)),
                [Tok{ref token, line, col, line_string, ..}, ..] => Err(format!(
                    "PARSING FAILURE at {},{}: Expected Ident but found {:?}\n{}\n{}",
                    line + 1,
                    col + 1,
                    token,
                    line_string,
                    get_caret_string(col)
                )),
                [] => Err(format!("PARSING FAILURE: Reached end of file but expected an ident"))
            }
        }
        // <expr>
        [Tok{token: _, ..}, ..] => {
            let (parsed_expr, tokens_after_expr) = try!(parse_expr(tokens));
            Ok((StmtItem::Bare(Box::new(parsed_expr)), tokens_after_expr))
        }
        [] => Err(format!("PARSING FAILURE: Reached end of file but expected a statement"))
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
        [Tok{token: Token::Assign, line, ..}, rest..] => {
            let (parsed_expr, tokens_after_expr) = try!(parse_expr(rest));
            Ok((Stmt::Assignment{items: parsed_items, expr: Box::new(parsed_expr),
                                 data: AstData{line: line}}, tokens_after_expr))
        }
        // EPS
        [Tok{line, ..}, ..] => {
            Ok((Stmt::Bare{items: parsed_items, data: AstData{line: line}}, tokens_after_items))
        }
        //TODO: fix line number for last bare statement
        [] => Ok((Stmt::Bare{items: parsed_items, data: AstData{line: 0}}, tokens_after_items))
    }
}

// <stmt>
fn parse_stmt<'a>(tokens: &'a[Tok]) -> ParseResult<'a, Stmt<'a>> {
    match tokens {
        // "if" <expr> <if-statements>
        [Tok{token: Token::If, line, ..}, rest..] => {
            let (parsed_expr, tokens_after_expr) = try!(parse_expr(rest));
            let (clauses, tokens_after_if) = {
                try!(parse_if_statements(tokens_after_expr, parsed_expr))
            };
            Ok((Stmt::If{clauses: clauses, data: AstData{line: line}}, tokens_after_if))
        }
        // "while" <expr> <block-statements>
        [Tok{token: Token::While, line, ..}, rest..] => {
            let (parsed_expr, tokens_after_expr) = try!(parse_expr(rest));
            let (stmt_list, tokens_after_list) = try!(parse_block_statements(tokens_after_expr));
            Ok((Stmt::While{condition: Box::new(parsed_expr), statements: stmt_list,
                            data: AstData{line: line}}, tokens_after_list))

        }
        // "return"
        [Tok{token: Token::Return, line, ..}, rest..] => {
            Ok((Stmt::Return{data: AstData{line: line}}, rest))
        }
        // <stmt-items>
        [_, ..] => parse_stmt_items(tokens),
        [] => Err(format!("PARSING FAILURE: Reached end of file but expected a statement"))
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
                my_toks = tokens_after_expr;
                my_expr = parsed_expr;
            }
            // "end"
            Token::End => {
                clauses.push(IfClause::If{condition: Box::new(my_expr), statements: statements});
                return Ok((clauses, &my_toks[1..]))
            }
            // <stmt> <if-statements>
            _ => {
                let (parsed_stmt, tokens_after_stmt) = try!(parse_stmt(my_toks));
                my_toks = tokens_after_stmt;
                statements.push(parsed_stmt);
            }
        }
    }
    Err(format!("PARSING FAILURE: Reached end of file but expected '}}'"))
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
            // <stmt> <block-statements>
            _ => {
                let (parsed_stmt, tokens_after_stmt) = try!(parse_stmt(my_toks));
                statements.push(parsed_stmt);
                my_toks = tokens_after_stmt;
            }
        }
    }
    Err(format!("PARSING FAILURE: Reached end of file but expected '}}'"))
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
            // <stmt> <clip-statements>
            _ => {
                let (parsed_stmt, tokens_after_stmt) = try!(parse_stmt(my_toks));
                statements.push(parsed_stmt);
                my_toks = tokens_after_stmt;
            }
        }
    }
    Err(format!("PARSING FAILURE: Reached end of file but expected '}}'"))
}

// <base-statements>
pub fn parse_base_statements<'a>(tokens: &'a[Tok],
                                 cur_statements: &'a mut Vec<Stmt<'a>>)
                                 -> ParseResult<'a, &'a Vec<Stmt<'a>>> {
    let mut my_toks = tokens;
    while my_toks.len() > 0 {
        let (parsed_stmt, tokens_after_stmt) = try!(parse_stmt(my_toks));
        cur_statements.push(parsed_stmt);
        my_toks = tokens_after_stmt;
    }
    Ok((cur_statements, my_toks))
}
