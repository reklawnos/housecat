pub mod ast {

    //Literals
    #[derive(Debug)]
    pub enum Literal {
        Bool(bool),          // <bool>
        Int(i64),            // <int>
        Float(f64),          // <float>
        String(Box<String>), // <string>
        Nil,                 // 'nil'
    }

    //Expressions
    #[derive(Debug)]
    pub enum Expr {
        UnOp(UnOp, Box<Expr>),              // <UnOp> <Expr>
        BinOp(BinOp, Box<Expr>, Box<Expr>), // <Expr> <BinOp> <Expr>
        Literal(Literal),                   // <Literal>
        Ident(Box<String>),                 // <Ident>
        Postfix(Box<Expr>, Box<Postfix>),   // <Expr> <Postfix>
        Tuple(Box<ExprList>)                // (<Expr>, <Expr>, ...)
    }

    //Postfix Operations
    #[derive(Debug)]
    pub enum Postfix {
        Play(Box<ExprList>, Box<Postfix>), // ... ( <Args> ) <Postfix>
        Index(Box<Expr>, Box<Postfix>),    // ... [ <Expr> ] <Postfix>
        Access(Box<String>, Box<Postfix>), // ... . <Ident> <Postfix>
        None                               // EPS
    }

    //Lists of expressions
    #[derive(Debug)]
    pub enum ExprList {
        Item(Box<Expr>, Box<ExprList>), // <Expr> , <ExprList>
        None                            // EPS
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
    // pub enum Stmt {
    //     Assignment
    // }

    //List of statements
    #[derive(Debug)]
    // pub enum StmtList {
    //     Item(Box<Stmt>, Box<StmtList>),
    //     None
    // }
}
