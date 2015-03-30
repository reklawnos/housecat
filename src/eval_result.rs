#[derive(Debug)]
pub enum Result<T> {
    Ok(T),
    Err(String)
}
