use super::value::Value;
use super::environment::ValueHolder;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub enum Op {
    //Stack manipulation
    Push(Box<Value>), // .. -> a, ..
    PushClip(ClipParts), // .. -> clip, ..
    MakeTuple(usize), // 1, ..., N, .. -> (1, ..., N), ..
    ExpandTuple(usize), // (1, ..., N), .. -> 1, ..., N, ..
    Jump(usize), // .. -> ..
    JumpIfFalse(usize), // bool, .. -> ..
    JumpTarget, // .. -> ..
    Return, // .. -> ..
    PushIterator, // a, .. -> ..
    PopIterator, // .. -> ..
    RetrieveIterator, // .. -> a, ..
    //Scoping
    PushScope, // .. -> ..
    PopScope, // .. -> ..
    //Variables
    Load(String), // .. -> a, ..
    LoadRef(Rc<RefCell<ValueHolder>>), // .. -> a, ..
    DeclareAndStore(String), // a, .. -> ..
    DeclareAndStoreImmutable(String), // a, .. -> ..
    Store(String), // a, .. -> ..
    StoreRef(Rc<RefCell<ValueHolder>>), // a, .. -> ..
    Def(Box<Value>), // clip, value, .. -> ..
    DefPop, // value, key, .. -> ..
    DefSelf(Box<Value>), // value, .. -> ..
    //Postfixes
    GetAndAccess, // b, a, .. -> a.b, ..
    Access(Box<Value>), // a, .. -> a.b, a, ..
    AccessPop(Box<Value>), // a, .. -> a.b, ..
    Play(usize), // 1, ..., N, a, .. -> a(1, ..., N), ..
    PlaySelf(usize), // 1, ..., N, func, self, .. -> func(self, 1, ..., N), ..
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

#[derive(Debug, Clone)]
pub struct ClipParts {
    pub params: Vec<String>,
    pub returns: Vec<String>,
    pub ops: Vec<Op>
}
