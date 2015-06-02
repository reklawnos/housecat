#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub enum Token<'a>{
    // Keywords
    Var,
    Nil,
    Fn,
    Return,
    In,
    If,
    Else,
    Elif,
    While,
    For,
    End,
    Do,

    // Symbols
    Eof,
    Assign,
    Def,
    Access,
    AccessSelf,
    OpenCurly,
    CloseCurly,
    OpenBrac,
    CloseBrac,
    OpenParen,
    CloseParen,
    Comma,
    Ret,

    // User values
    Bool(bool),
    Int(i64),
    Float(f64),
    Ident(&'a str),
    String(String),
    
    // Operators
    Not,
    Get,
    Exp,
    Mul,
    Div,
    Mod,
    Add,
    Sub,
    Lt,
    Lte,
    Gt,
    Gte,
    Eq,
    Neq,
    And,
    Or
}

#[derive(Debug)]
pub struct Tok<'a> {
    pub token: Token<'a>,
    pub line: usize,
    pub line_string: &'a str, 
    pub col: usize,
    pub char_index: usize
}
