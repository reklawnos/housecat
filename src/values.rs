#[derive(Debug, Clone)]
pub enum Value {
    Int(i64),
    Float(f64),
    Bool(bool),
    String(Box<String>),
    Tuple(Vec<Value>),
    Nil,
    Nothing //Type of function call with no returns
}
