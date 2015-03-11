Grammar for Parsing Housecat
============================

Expressions
-----------
`<bool>`, `<int>`, `<float>`, `<string>`, and `<ident>` are all represented by strings of terminals.


    <primary-expr> ::=
        | <bool>
        | <int>
        | <float>
        | <string>
        | <ident>
        | "nil"
        | { <statements> }
        | ( <expr> )

    <postfix-expr> ::=
        | <primary-expr> <postfix-continuation>

    <postfix-continuation> ::=
        | "(" <args> <postfix-continuation>
        | "." <ident> <postfix-continuation>
        | "[" <expr> "]" <postfix-continuation>
        | ""

    <args> ::=
        | <args-list> ")"
        | ")"

    <args-list> ::=
        | <expr>
        | <expr> "," <args-list>

    <unary-expr> ::=
        | <postfix-expr>
        | "-" <unary-expr>
        | "!" <unary-expr>

    <exponential-expr> ::=
        | <unary-expr>
        | <unary-expr> "^" <exponential-expr>

    <multiplicative-expr> ::=
        | <exponential-expr>
        | <exponential-expr> "*" <multiplicative-expr>
        | <exponential-expr> "/" <multiplicative-expr>
        | <exponential-expr> "%" <multiplicative-expr>

    <additive-expr> ::=
        | <multiplicative-expr>
        | <multiplicative-expr> "+" <additive-expr>
        | <multiplicative-expr> "-" <additive-expr>

    <relational-expr> ::=
        | <additive-expr>
        | <additive-expr> "<" <relational-expr>
        | <additive-expr> "<=" <relational-expr>
        | <additive-expr> ">" <relational-expr>
        | <additive-expr> ">=" <relational-expr>

    <equality-expr> ::=
        | <relational-expr>
        | <relational-expr> "=" <equality-expr>
        | <relational-expr> "!=" <equality-expr>
        | <relational-expr> "==" <equality-expr>
        | <relational-expr> "!==" <equality-expr>

    <and-expr> ::=
        | <equality-expr>
        | <equality-expr> "&&" <and-expr>

    <or-expr> ::=
        | <and-expr>
        | <and-expr> "||" <and-expr>

    <expr> ::=
        | <or-expr>


Statements
----------

    <assignment-types> ::=
        | "var" <ident>
        | "def" <ident>
        | <ident>

    <assignment-idents> ::=
        | <assignment-types>
        | <assignment-types> "," <assignment-idents>

    <if-expr>

    <stmt> ::=
        | <assignment-idents> ":" <expr>
        | "if" <expr> "then" <statements> "end"
        | "if" <expr> "then" <statements> "else" <statements> "end"

    <statments> ::=
        | <stmt> <statements>
        | ""
