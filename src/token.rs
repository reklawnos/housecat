#[derive(Debug)]
pub enum Token<'a>{
    // Keywords
    Def,     // `def`
    Var,     // `var`
    Nil,     // `nil`
    Fn,      // `fn`
    Return,  // `return`
    In,      // `in`
    If,      // `if`
    Else,    // `else`
    Elif,    // `elif`
    While,   // `while`
    End,     // `end`

    // Symbols
    //Eof,       // End of file
    Assign,      // `:`
    Dot,         // `.`
    OpenCurly,   // `{`
    CloseCurly,  // `}`
    OpenBrac,    // `[`
    CloseBrac,   // `]`
    OpenParen,   // `(`
    CloseParen,  // `)`
    Comma,       // `,`
    Ret,         // `->`

    // User values
    Bool(bool),           // `true` or `false`
    Int(i64),             // ex. `1324`, `-43`
    Float(f64),           // ex. `1.3`, `-34.432e-4`
    Ident(&'a str),       // ex. `foo`, `bar`
    String(Box<String>),  // ex. `"foo"`
    
    // Operators
    Not,    // `!`
    Exp,    // `^`
    Mul,    // `*`
    Div,    // `/`
    Mod,    // `%`
    Add,    // `+`
    Sub,    // `-`
    Lt,     // `<`
    Lte,    // `<=`
    Gt,     // `>`
    Gte,    // `>=`
    Eq,     // `=`
    Neq,    // `!=`
    Same,   // `==`
    Nsame,  // `!==`
    And,    // `&&`
    Or,     // `||`
}

#[derive(Debug)]
pub struct Tok<'a> {
    pub token: Token<'a>,
    pub line: usize,
    pub line_string: &'a str, 
    pub col: usize,
    pub char_index: usize
}
