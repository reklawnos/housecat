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
        | "{" <clip-block>
        | "fn" <clip-def>
        | "(" <expr> ")"
        | "(" <expr> "," <expr-list>

    <postfix-expr> ::=
        | <primary-expr> <postfix-continuation>

    <postfix-continuation> ::=
        | "(" <args> <postfix-continuation>
        | "." <ident> <postfix-continuation>
        | "[" <expr> "]" <postfix-continuation>
        | ""

    <args> ::=
        | <expr-list>
        | ")"

    <expr-list> ::=
        | <expr> ")"
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

    <in-expr> ::=
        | <additive-expr>
        | <additive-expr> "in" <in-expr>

    <relational-expr> ::=
        | <in-expr>
        | <in-expr> "<" <relational-expr>
        | <in-expr> "<=" <relational-expr>
        | <in-expr> ">" <relational-expr>
        | <in-expr> ">=" <relational-expr>

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
        | <and-expr> "||" <or-expr>

    <expr> ::=
        | <or-expr>


Statements
----------

    <item> ::=
        | "var" <ident>
        | "def" <expr>
        | <expr>

    <item-list> ::=
        | <item>
        | <item> "," <item-list>

    <stmt-items> ::=
        | <item-list>
        | <item-list> ":" <expr>

    <stmt> ::=
        | <stmt-items>
        | "{" <clip-block>
        | "fn" <clip-def>
        | "if" <expr> "then" <if-statements>

    <if-statements> ::=
        | <stmt> <if-statements>
        | "end"
        | "else" <block-statements>

    <block-statements> ::=
        | <stmt> <block-statements>
        | "end"

    <clip-statements> ::=
        | <stmt> <clip-statements>
        | "}"

    <base-statments> ::=
        | <stmt> <base-statements>
        | ""

Clips
-----

    <params> ::=
        | <params-list> ")"
        | ")"

    <params-list> ::=
        | <ident>
        | <ident> "," <params-list>

    <clip-def> ::=
        | "(" <params> "{" <clip-statements>
        | "(" <params> "->" <ident> <clip-statements>
        | "(" <params> "->" "(" <params> <clip-statements>