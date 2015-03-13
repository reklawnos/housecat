pub mod ast {

    //Literals
    #[derive(Debug)]
    pub enum Literal {
        Bool(bool),                                               // <bool>
        Int(i64),                                                 // <int>
        Float(f64),                                               // <float>
        String(Box<String>),                                      // <string>
        Clip(Vec<Box<String>>, Vec<Box<String>>, Vec<Stmt>), // <clip>
        Nil,                                                      // 'nil'
    }

    //Expressions
    #[derive(Debug)]
    pub enum Expr {
        UnOp(UnOp, Box<Expr>),                   // <UnOp> <Expr>
        BinOp(BinOp, Box<Expr>, Box<Expr>),      // <Expr> <BinOp> <Expr>
        Literal(Literal),                        // <Literal>
        Ident(Box<String>),                      // <Ident>
        Postfix(Box<Expr>, Vec<Postfix>),   // <Expr> <Postfix>
        Tuple(Box<Vec<Expr>>)                    // (<Expr>, <Expr>, ...)
    }

    //Postfix Operations
    #[derive(Debug)]
    pub enum Postfix {
        Play(Vec<Expr>), // ... ( <Args> ) <Postfix>
        Index(Box<Expr>),     // ... [ <Expr> ] <Postfix>
        Access(Box<String>),  // ... . <Ident> <Postfix>
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
    pub enum Stmt {
        Assignment(Vec<StmtItem>, Box<Expr>),
        Bare(Vec<StmtItem>),
        If(Box<Expr>, Vec<Stmt>, Vec<Stmt>),
        While(Box<Expr>, Vec<Stmt>),
        Return
    }

    //Statement item types
    #[derive(Debug)]
    pub enum StmtItem {
        Bare(Box<Expr>), // <Ident> <Postfix>
        Def(Box<Expr>),  // def <Ident> <Postfix>
        Var(Box<String>) // var <Ident>
    }
}
