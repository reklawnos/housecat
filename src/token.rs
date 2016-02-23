use std::fmt;

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub enum Token<'a>{
    // Keywords
    Var,
    Let,
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
    ExprDef,
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

impl<'a> fmt::Display for Token<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match *self{
            // Keywords
            Token::Var => "var".to_string(),
            Token::Let => "let".to_string(),
            Token::Nil => "nil".to_string(),
            Token::Fn => "fn".to_string(),
            Token::Return => "return".to_string(),
            Token::In => "in".to_string(),
            Token::If => "if".to_string(),
            Token::Else => "else".to_string(),
            Token::Elif => "elif".to_string(),
            Token::While => "while".to_string(),
            Token::For => "for".to_string(),
            Token::End => "end".to_string(),
            Token::Do => "do".to_string(),

            // Symbols
            Token::Eof => "EOF".to_string(),
            Token::Assign => "=".to_string(),
            Token::Def => ":".to_string(),
            Token::Access => ".".to_string(),
            Token::AccessSelf => "|".to_string(),
            Token::ExprDef => "@".to_string(),
            Token::OpenCurly => "{".to_string(),
            Token::CloseCurly => "}".to_string(),
            Token::OpenBrac => "[".to_string(),
            Token::CloseBrac => "]".to_string(),
            Token::OpenParen => "(".to_string(),
            Token::CloseParen => ")".to_string(),
            Token::Comma => ",".to_string(),
            Token::Ret => "->".to_string(),

            // Token::User values
            Token::Bool(b) => b.to_string(),
            Token::Int(i) => i.to_string(),
            Token::Float(f) => f.to_string(),
            Token::Ident(ref s) => s.to_string(),
            Token::String(ref s) => format!("\"{}\"", s),

            // Token::Operators
            Token::Not => "!".to_string(),
            Token::Get => "$".to_string(),
            Token::Exp => "^".to_string(),
            Token::Mul => "*".to_string(),
            Token::Div => "/".to_string(),
            Token::Mod => "%".to_string(),
            Token::Add => "+".to_string(),
            Token::Sub => "-".to_string(),
            Token::Lt => "<=".to_string(),
            Token::Lte => ">".to_string(),
            Token::Gt => "==".to_string(),
            Token::Gte => ">=".to_string(),
            Token::Eq => "==".to_string(),
            Token::Neq => "!=".to_string(),
            Token::And => "&&".to_string(),
            Token::Or => "||".to_string(),
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug)]
pub struct Tok<'a> {
    pub token: Token<'a>,
    pub line: usize,
    pub line_string: &'a str,
    pub col: usize,
    pub char_index: usize
}
