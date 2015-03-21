#[macro_export]
macro_rules! get_parsed(
    ($parsed:expr) => ({
        match $parsed {
            Result::Ok(p, toks) => (p, toks),
            Result::Err(e) => {return Result::Err(e);}
        }
    });
);

#[macro_export]
macro_rules! parse_expr_binary_op(
    ($tokens:ident, $parse_lhs:ident, $parse_rhs:ident, [ $($tok:pat => $op:expr),+ ]) => ({
        // <LHS> ...
        let (parsed_lhs, tokens_after_lhs) = get_parsed!($parse_lhs($tokens));
        match tokens_after_lhs {
            [ref next_tok, rest..] => {
                match next_tok.token {
                    $(
                        // ... <op> <RHS>
                        $tok => {
                            let (parsed_rhs, tokens_after_term) = get_parsed!($parse_rhs(rest));
                            Result::Ok(
                                Expr::BinOp{
                                    op: $op,
                                    lhs: Box::new(parsed_lhs),
                                    rhs: Box::new(parsed_rhs),
                                    data: AstData{line: $tokens[0].line}
                                },
                                tokens_after_term
                            )
                        },
                    )+
                    // <LHS>
                    _ => Result::Ok(parsed_lhs, tokens_after_lhs),
                }
            }
            // <LHS>
            _ => Result::Ok(parsed_lhs, tokens_after_lhs),
        }
    });
);
