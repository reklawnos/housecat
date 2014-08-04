/*
pub mod Ast {
    pub enum Literal {
        LitBool(bool), // 'true' or 'false'
        LitInt(i64),   // integers
        LitFloat(f64), // floats
        LitString(Box<String>), // string literals
        LitNil, // 'nil'
    }

    //Expressions
    pub enum Exp {
        ExpMonoOp(MonoOp, Box<Exp>), // <MonoOp> <Exp>
        ExpBinaryOp(BinaryOp, Box<Exp>, Box<Exp>), // <Exp> <BinaryOp> <Exp>
        ExpLiteral(Literal), // Literal
        ExpIdent(Box<String>), // Ident
        ExpIf(Box<Exp>, Box<Exp>, Box<Exp>) // if <Exp> <Exp> <`else` Exp> 
    }

    //Statements
    pub enum Statement {
        StAssignment(Box<String>, Exp) // 
    }

    pub enum MonoOp {
        MonNeg, // '-'' (number negation)
        MonNot, // '!' (boolean not)
    }

    pub enum BinaryOp {
        BinAdd, // '+'
        BinSub, // '-'
        BinMul, // '*'
        BinDiv,  // '/'
    }
}
*/

/*
Basic grammar:
<Expr> ::=
    | <IdExpr>
    | <BinOpExpr>
    | ( <Expr> )

<BinOpExpr> ::=
    | <Expr> + <Expr>
    | <Expr> - <Expr>
    | <Expr> * <Expr>
    | <Expr> / <Expr>

Unambiguous LL(1) grammar:
<Expr> ::=
    | <Term>
    | <Term> + <Expr>
    | <Term> - <Expr>

<Term> ::=
    | <Factor>
    | <Factor> * <Term>
    | <Factor> / <Term>

<Factor> ::=
    | <Id>
    | ( <Expr> )
*/
pub mod test_ast {
    pub enum Expr {
        BinOpExpr(BinaryOp, Box<Expr>, Box<Expr>),
        IdExpr(Box<String>)
    }

    pub enum BinaryOp {
        BinAdd, // '+'
        BinSub, // '-'
        BinMul, // '*'
        BinDiv,  // '/'
    }
}

