#[macro_use]
mod macros;
mod stmt;
pub mod expr;
mod clip;

use token::Tok;
use ast::*;
use parser::stmt::parse_base_statements;


pub enum ParseResult<'a, T> {
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

pub fn parse_tokens<'a>(tokens: &'a[Tok], cur_statements: &'a mut Vec<Stmt<'a>>) -> Result<&'a Vec<Stmt<'a>>, String> {
    match parse_base_statements(tokens, cur_statements) {
        ParseResult::Ok(v, _) => Ok(v),
        ParseResult::Err(s) => Err(s)
    }
}
