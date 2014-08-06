#[deriving(Show)]
pub enum Token {
    // Keywords
    Def,                // 'def'
    // Symbols
    Eof,                // End of file
    Colon,              // :
    Dot,                // .
    OpenBrac,           // {
    CloseBrac,          // }
    OpenParen,          // (
    CloseParen,         // )
    // User values
    Bool(bool),         // 'true' or 'false'
    Int(i64),           // ex. 1324, -43
    Float(f64),         // ex. 1.3, -34.432e-4
    Ident(Box<String>), // ex. foo, bar
    String(Box<String>),
    // Binary ops
    Add,                // '+'
    Sub,                // '-'
    Mul,                // '*'
    Div,                // '/'
}

pub struct Tok {
    pub token: Token,
    pub line: uint,
    pub col: uint
}