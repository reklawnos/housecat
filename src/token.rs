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

impl Token {
    pub fn to_string(&self) -> String {
        match self {
            &Def => format!("Def"),
            &Ident(ref s) => format!("Ident({})", s),
            &Bool(b) => format!("Bool({})", b),
            &Float(f) => format!("Float({})", f),
            &Int(i) => format!("Int({})", i),
            &String(ref s) => format!("String({})", s),
            &Colon => format!("Colon"),
            &Dot => format!("Dot"),
            &OpenBrac => format!("OpenBrac"),
            &CloseBrac => format!("CloseBrac"),
            &OpenParen => format!("OpenParen"),
            &CloseParen => format!("CloseParen"),
            &Add => format!("Add"),
            &Sub => format!("Sub"),
            &Div => format!("Div"),
            &Mul => format!("Mul"),
            _ => fail!("not implemented")
        }
    }
}

pub struct Tok {
    pub token: Token,
    pub line: uint,
    pub col: uint
}
