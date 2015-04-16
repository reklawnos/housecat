use super::values::Value;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub enum Op<'a> {
    //Stack manipulation
    Push(Box<Value<'a>>), // .. -> a, ..
    PushClip(Box<ClipParts<'a>>), // .. -> clip, ..
    MakeTuple(usize), // 1, ..., N, .. -> (1, ..., N), ..
    Jump(usize), // .. -> ..
    JumpIfFalse(usize), // bool, .. -> ..
    JumpTarget, // .. -> ..
    //Scoping
    PushScope,
    PopScope,
    //Variables
    Load(&'a str), // .. -> a, ..
    Store(&'a str), // a, .. -> ..
    Def(Box<Value<'a>>), // clip, value, .. -> ..
    DefSelf(Box<Value<'a>>), // value, .. -> ..
    //Postfixes
    Access(Box<Value<'a>>), // a, .. -> a.b, ..
    Play(usize), // 1, ..., N, a, .. -> a(1, ..., N), ..
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

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct ClipParts<'a> {
    pub params: Vec<&'a str>,
    pub returns: Vec<&'a str>,
    pub ops: Vec<Op<'a>>
}
