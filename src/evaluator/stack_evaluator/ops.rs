use super::values::Value;

#[derive(Debug, PartialEq)]
pub enum Op<'a> {
    //Stack manipulation
    Push(Value<'a>), // _ -> a
    //Pop, // a, .. -> ..
    MakeTuple(usize), // 1, ... N, .. -> (1, ..., N), ..
    Jump(usize), // .. -> ..
    //JumpIfTrue(usize), // a -> ..
    JumpIfFalse(usize), // a -> ..
    JumpTarget, // .. -> ..
    //Scoping
    PushScope,
    PopScope,
    //Variables
    //AssignDef(&'a str), // a, .. -> ..
    //AssignVar(&'a str), // a, .. -> ..
    Load(&'a str), // .. -> a, ..
    Store(&'a str), // a, .. -> ..
    Access(&'a str), // a, .. -> a.b, ..
    //Unary ops
    Get, // a, .. -> $a ..
    Neg, // a, .. -> -a ..
    Not, // a, .. -> !a ..
    //Binary ops
    Add, // b, a, .. -> a + b, ..
    Sub, // b, a, .. -> a - b, ..
    Mul, // b, a, .. -> a * b, ..
    Div, // b, a, .. -> a / b, ..
    Mod, // b, a, .. -> a % b, ..
    In, // b, a, .. -> a in b, ..
    Lt, // b, a, .. -> a < b, ..
    Lte, // b, a, .. -> a <= b, ..
    Gt, // b, a, .. -> a > b, ..
    Gte, // b, a, .. -> a >= b, ..
    Eq, // b, a, .. -> a = b, ..
    Neq, // b, a, .. -> a != b, ..
    And, // b, a, .. -> a && b, ..
    Or, // b, a, .. -> a || b, ..
}
