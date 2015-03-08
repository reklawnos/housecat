#[derive(Debug)]
pub enum Token {
    // Keywords
    Def,                // 'def'
    Nil,                // 'nil'
    // Symbols
    //Eof,                // End of file
    Colon,              // :
    Dot,                // .
    OpenCurly,          // {
    CloseCurly,         // }
    OpenBrac,           // [
    CloseBrac,          // ]
    OpenParen,          // (
    CloseParen,         // )
    Comma,              // ,
    // User values
    Bool(bool),         // 'true' or 'false'
    Int(i64),           // ex. 1324, -43
    Float(f64),         // ex. 1.3, -34.432e-4
    Ident(Box<String>), // ex. foo, bar
    String(Box<String>),
    // Operators
    Not,                // '!'
    Exp,                // '^'
    Mul,                // '*'
    Div,                // '/'
    Mod,                // '%'
    Add,                // '+'
    Sub,                // '-'
    Lt,                 // '<'
    Lte,                // '<='
    Gt,                 // '>'
    Gte,                // '>='
    Eq,                 // '='
    Neq,                // '!='
    Same,               // '=='
    Nsame,              // '!=='
    And,                // '&&'
    Or,                 // '||'     

}

pub struct Tok<'a> {
    pub token: Token,
    pub line: usize,
    pub line_string: &'a String, 
    pub col: usize
}
