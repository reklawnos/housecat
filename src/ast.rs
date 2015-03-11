pub mod ast {
    //Literals
    #[derive(Debug)]
    pub enum Literal {
        LitBool(bool),          // <bool>
        LitInt(i64),            // <int>
        LitFloat(f64),          // <float>
        LitString(Box<String>), // <string>
        LitNil,                 // 'nil'
    }

    //Expressions
    #[derive(Debug)]
    pub enum Expr {
        ExprUnOp(UnOp, Box<Expr>),              // <UnOp> <Expr>
        ExprBinOp(BinOp, Box<Expr>, Box<Expr>), // <Expr> <BinOp> <Expr>
        ExprLiteral(Literal),                   // <Literal>
        ExprIdent(Box<String>),                 // <Ident>
        ExprPostfix(Box<Expr>, Box<Postfix>),   // <Expr> <Postfix>
        ExprTuple(Box<ExprList>)                // (<Expr>, <Expr>, ...)
    }

    //Postfix Operations
    #[derive(Debug)]
    pub enum Postfix {
        PostfixPlay(Box<ExprList>, Box<Postfix>), // ... ( <Args> ) <Postfix>
        PostfixIndex(Box<Expr>, Box<Postfix>),    // ... [ <Expr> ] <Postfix>
        PostfixAccess(Box<String>, Box<Postfix>), // ... . <Ident> <Postfix>
        PostfixNone                               // EPS
    }

    //Lists of expressions
    #[derive(Debug)]
    pub enum ExprList {
        ListItem(Box<Expr>, Box<ExprList>), // <Expr> , <ExprList>
        ListNone                            // EPS
    }

    //Statements
    // #[derive(Debug)]
    // pub enum Statement {
    //     StAssignment(Box<String>, Expr) // 
    // }

    //Unary Operators
    #[derive(Debug)]
    pub enum UnOp {
        UnNeg, // '-' (number negation)
        UnNot, // '!' (boolean not)
    }

    //Binary Operators
    #[derive(Debug)]
    pub enum BinOp {
        BinAdd,   // '+'
        BinSub,   // '-'
        BinMul,   // '*'
        BinDiv,   // '/'
        BinMod,   // '%'
        BinExp,   // '^'
        BinLt,    // '<'
        BinLte,   // '<='
        BinGt,    // '>'
        BinGte,   // '>='
        BinEq,    // '='
        BinNeq,   // '!='
        BinSame,  // '=='
        BinNsame, // '!=='
        BinAnd,   // '&&'
        BinOr,    // '||'
    }
}
