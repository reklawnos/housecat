#[macro_export]
macro_rules! parse_expr_binary_op(
    ($tokens:ident, $parse_lhs:ident, $parse_rhs:ident, [ $($tok:pat => $op:expr),+ ]) => ({
        // <LHS> ...
        let (parsed_lhs, tokens_after_lhs) = try!($parse_lhs($tokens));
        match tokens_after_lhs {
            $(
                // ... <op> <RHS>
                [Tok{token: $tok, ..}, rest..] => {
                    let (parsed_rhs, tokens_after_term) = try!($parse_rhs(rest));
                    Ok((
                        Expr::BinOp{
                            op: $op,
                            lhs: Box::new(parsed_lhs),
                            rhs: Box::new(parsed_rhs),
                            data: AstData{line: $tokens[0].line}
                        },
                        tokens_after_term
                    ))
                }
            )+
            // <LHS>
            [Tok{token: _, ..}, ..] => Ok((parsed_lhs, tokens_after_lhs)),
            // <LHS>
            [] => Ok((parsed_lhs, tokens_after_lhs)),
        }
    });
);
