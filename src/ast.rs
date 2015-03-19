#[derive(Debug)]
pub struct AstData {
    pub line: usize,
    //pub start: usize,
    //pub end: usize
}

//Literals
#[derive(Debug)]
pub enum Literal<'a> {
    Bool {
        value: bool,
        data: AstData
    },
    Int {
        value: i64,
        data: AstData
    },
    Float {
        value: f64,
        data: AstData
    },
    String {
        value: &'a str,
        data: AstData
    },
    Clip {
        params: Vec<&'a str>,
        returns: Vec<&'a str>,
        statements: Vec<Stmt<'a>>,
        data: AstData
    },
    Nil {
        data: AstData
    }
}

//Expressions
#[derive(Debug)]
pub enum Expr<'a> {
    UnOp {
        op: UnOp,
        expr: Box<Expr<'a>>,
        data: AstData
    },
    BinOp {
        op: BinOp,
        lhs: Box<Expr<'a>>,
        rhs: Box<Expr<'a>>,
        data: AstData
    },
    Literal(Literal<'a>),                       // <Literal>
    Ident(&'a str),                             // <Ident>
    Postfix(Box<Expr<'a>>, Vec<Postfix<'a>>),   // <Expr> <Postfix>
    Tuple(Box<Vec<Expr<'a>>>)                   // (<Expr>, <Expr>, ...)
}

//Postfix Operations
#[derive(Debug)]
pub enum Postfix<'a> {
    Play(Vec<Expr<'a>>),  // ... ( <Args> ) <Postfix>
    Index(Box<Expr<'a>>), // ... [ <Expr> ] <Postfix>
    Access(&'a str),      // ... . <Ident> <Postfix>
}

//Unary Operators
#[derive(Debug)]
pub enum UnOp {
    Neg, // '-' (number negation)
    Not, // '!' (boolean not)
}

//Binary Operators
#[derive(Debug)]
pub enum BinOp {
    Add,   // '+'
    Sub,   // '-'
    Mul,   // '*'
    Div,   // '/'
    Mod,   // '%'
    Exp,   // '^'
    In,    // 'in'
    Lt,    // '<'
    Lte,   // '<='
    Gt,    // '>'
    Gte,   // '>='
    Eq,    // '='
    Neq,   // '!='
    Same,  // '=='
    Nsame, // '!=='
    And,   // '&&'
    Or,    // '||'
}

//Statement
#[derive(Debug)]
pub enum Stmt<'a> {
    Assignment(Vec<StmtItem<'a>>, Box<Expr<'a>>),
    Bare(Vec<StmtItem<'a>>),
    If(Box<Expr<'a>>, Vec<Stmt<'a>>, Vec<Stmt<'a>>),
    While(Box<Expr<'a>>, Vec<Stmt<'a>>),
    Return
}

//Statement item types
#[derive(Debug)]
pub enum StmtItem<'a> {
    Bare(Box<Expr<'a>>), // <Ident> <Postfix>
    Def(Box<Expr<'a>>),  // def <Ident> <Postfix>
    Var(&'a str)         // var <Ident>
}
