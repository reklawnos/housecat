Grammar for Parsing Housecat
============================

Expressions
-----------
`<bool>`, `<int>`, `<float>`, `<string>`, and `<ident>` are all represented by strings of terminals.
    

    <paren-continuation> ::=
        | ")"
        | "," <primary-expr> <paren-continuation>

    <brac-continuation> ::=
        | "]" <postfix-continuation>
    
    <postfix-continuation> ::=
        | "(" <args> <postfix-continuation>
        | "." <ident> <postfix-continuation>
        | "[" <expr> <brac-continuation>
        | ""

    <primary-expr> ::=
        | <bool> <postfix-continuation>
        | <int> <postfix-continuation>
        | <float> <postfix-continuation>
        | <string> <postfix-continuation>
        | <ident> <postfix-continuation>
        | "nil" <postfix-continuation>
        | "{" <clip-block> <postfix-continuation>
        | "fn" <clip-def> <postfix-continuation>
        | "(" <paren-continuation> <postfix-continuation>
        | "-" <primary-expr> <postfix-continuation>
        | "!" <primary-expr> <postfix-continuation>