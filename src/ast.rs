pub mod Ast {
    //Literals
    #[deriving(Show)]
    pub enum Literal {
        LitBool(bool),          // <bool>
        LitInt(i64),            // <int>
        LitFloat(f64),          // <float>
        LitString(Box<String>), // <string>
        LitNil,                 // 'nil'
    }

    //Expressions
    #[deriving(Show)]
    pub enum Expr {
        ExprUnOp(UnOp, Box<Expr>),              // <UnOp> <Expr>
        ExprBinOp(BinOp, Box<Expr>, Box<Expr>), // <Expr> <BinOp> <Expr>
        ExprLiteral(Literal),                   // <literal>
        ExprIdent(Box<String>),                 // <Ident>
    }

    //Statements
    #[deriving(Show)]
    pub enum Statement {
        StAssignment(Box<String>, Expr) // 
    }

    //Unary Operators
    #[deriving(Show)]
    pub enum UnOp {
        UnNeg, // '-' (number negation)
        UnNot, // '!' (boolean not)
    }

    //Binary Operators
    #[deriving(Show)]
    pub enum BinOp {
        BinAdd,   // '+'
        BinSub,   // '-'
        BinMul,   // '*'
        BinDiv,   // '/'
        BinExp,   // '^'
        BinLt,    // '<'
        BinLte,   // '<='
        BinGt,    // '>'
        BinGte,   // '>='
        BinEq,    // '='
        BinNeq,   // '!='
        BinSame,  // '=='
        BinNsame, // '!=='
    }
}
