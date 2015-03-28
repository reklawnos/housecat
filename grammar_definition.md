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
        | "(" <params> <postfix-continuation>
        | "." <ident> <postfix-continuation>
        | "[" <expr> "]" <postfix-continuation>
        | ""

    <params> ::=
        | <expr-list>
        | ")"

    <expr-list> ::=
        | <expr> ")"
        | <expr> "," <expr-list>

    <unary-expr> ::=
        | <postfix-expr>
        | "-" <unary-expr>
        | "!" <unary-expr>
        | "$" <unary-expr>

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
        | "if" <expr> <if-statements>
        | "while" <expr> <block-statements>
        | "return"

    <if-statements> ::=
        | <stmt> <if-statements>
        | "end"
        | "elif" <expr> <if-statements>
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
        | <ident-list>
        | ")"

    <rets> ::=
        | "(" <ident-list>
        | <ident>

    <ident-list> ::=
        | <ident> ")"
        | <ident> "," <ident-list>

    <clip-def> ::=
        | "(" <params> "{" <clip-statements>
        | "(" <params> "->" <rets> <clip-statements>
