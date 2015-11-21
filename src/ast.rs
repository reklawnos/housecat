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
pub struct Expr<'a> {
    pub expr: ExprType<'a>,
    pub data: AstData
}

#[derive(Debug)]
pub enum ExprType<'a> {
    UnOp {
        op: UnOp,
        expr: Box<Expr<'a>>,
    },
    BinOp {
        op: BinOp,
        lhs: Box<Expr<'a>>,
        rhs: Box<Expr<'a>>,
    },
    Literal {
        value: Literal<'a>,
    },
    Ident {
        name: &'a str,
    },
    Postfix {
        expr: Box<Expr<'a>>,
        postfixes: Vec<Postfix<'a>>,
    },
    Tuple {
        values: Vec<Expr<'a>>,
    }
}

//Postfix Operations
#[derive(Debug)]
pub enum Postfix<'a> {
    Play(Vec<Expr<'a>>),
    PlaySelf(&'a str, Vec<Expr<'a>>),
    Index(Box<Expr<'a>>),
    Access(&'a str)
}

//Unary Operators
#[derive(Debug)]
pub enum UnOp {
    Neg,
    Not,
    Get
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
    And,
    Or
}

//Statement
#[derive(Debug)]
pub struct Stmt<'a> {
    pub stmt: StmtType<'a>,
    pub data: AstData
}

#[derive(Debug)]
pub enum StmtType<'a> {
    Def {
        items: Vec<StmtItem<'a>>,
        expr: Box<Expr<'a>>,
    },
    Assign {
        items: Vec<StmtItem<'a>>,
        expr: Box<Expr<'a>>,
    },
    Bare {
        items: Vec<StmtItem<'a>>,
    },
    If {
        clauses: Vec<IfClause<'a>>,
    },
    While {
        condition: Box<Expr<'a>>,
        statements: Vec<Stmt<'a>>,
    },
    For {
        idents: Vec<&'a str>,
        iterator: Box<Expr<'a>>,
        statements: Vec<Stmt<'a>>,
    },
    Return
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
    Expr(Box<Expr<'a>>),
    Var(&'a str),
    Let(&'a str)
}
