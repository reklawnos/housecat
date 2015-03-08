pub fn get_caret_string(col: usize) -> String {
    let mut caret_string = String::with_capacity(col + 1);
    for _ in 0..col {
        caret_string.push(' ');
    }
    caret_string.push('^');
    return caret_string;
}