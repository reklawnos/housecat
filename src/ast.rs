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
        ExprPostfix(Box<Expr>, Box<Postfix>)    // <Expr> <Postfix>
    }

    //Postfix Operations
    #[derive(Debug)]
    pub enum Postfix {
        PostfixPlay(Box<Args>, Box<Postfix>),     // ... ( <Args> ) <Postfix>
        PostfixIndex(Box<Expr>, Box<Postfix>),    // ... [ <Expr> ] <Postfix>
        PostfixAccess(Box<String>, Box<Postfix>), // ... . <Ident> <Postfix>
        PostfixNone                               // EPS
    }

    //Arguments
    #[derive(Debug)]
    pub enum Args {
        ArgsItem(Box<Expr>, Box<Args>), // <Expr> , <Args>
        ArgsNone                       // EPS
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
