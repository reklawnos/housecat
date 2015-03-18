#[derive(Debug, Clone)]
pub enum Value<'a> {
    Int(i64),
    Float(f64),
    Bool(bool),
    String(&'a str),
    Tuple(Vec<Value<'a>>),
    Nil,
    Nothing //Type of function call with no returns
}
