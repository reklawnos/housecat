#[derive(Debug)]
pub struct AstData {
    pub line: usize,
    //pub start: usize,
    //pub end: usize
}

//Literals
#[derive(Debug)]
pub enum Literal<'a> {
    Bool(bool),
    Int(i64),
    Float(f64),
    String(&'a str),
    Clip {
        params: Vec<&'a str>,
        returns: Vec<&'a str>,
        statements: Vec<Stmt<'a>>
    },
    Nil
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
    Literal {
        value: Literal<'a>,
        data: AstData
    },
    Ident {
        name: &'a str,
        data: AstData
    },
    Postfix {
        expr: Box<Expr<'a>>,
        postfix: Vec<Postfix<'a>>,
        data: AstData
    },
    Tuple {
        values: Vec<Expr<'a>>,
        data: AstData
    }
}

//Postfix Operations
#[derive(Debug)]
pub enum Postfix<'a> {
    Play(Vec<Expr<'a>>),
    Index(Box<Expr<'a>>),
    Access(&'a str)
}

//Unary Operators
#[derive(Debug)]
pub enum UnOp {
    Neg,
    Not
}

//Binary Operators
#[derive(Debug)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Exp,
    In,
    Lt,
    Lte,
    Gt,
    Gte,
    Eq,
    Neq,
    Same,
    Nsame,
    And,
    Or
}

//Statement
#[derive(Debug)]
pub enum Stmt<'a> {
    Assignment {
        items: Vec<StmtItem<'a>>,
        expr: Box<Expr<'a>>,
        data: AstData
    },
    Bare {
        items: Vec<StmtItem<'a>>,
        data: AstData
    },
    If {
        clauses: Vec<IfClause<'a>>,
        data: AstData
    },
    While {
        condition: Box<Expr<'a>>,
        statements: Vec<Stmt<'a>>,
        data: AstData
    },
    Return {
        data: AstData
    }
}

//If statement clauses
#[derive(Debug)]
pub enum IfClause<'a> {
    If {
        condition: Box<Expr<'a>>,
        statements: Vec<Stmt<'a>>
    },
    Else(Vec<Stmt<'a>>)
}

//Statement item types
#[derive(Debug)]
pub enum StmtItem<'a> {
    Bare(Box<Expr<'a>>),
    Def(Box<Expr<'a>>),
    Var(&'a str)
}
