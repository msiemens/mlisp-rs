    # AST
    program : <expr>*
    expr    : <number> | <symbol> | <sexpr>
    sexpr   : <lparen> <expr>* <rparen>
    number  : (<minus> [0-9]+ | [0-9]+)

    # Tokens
    symbol  : [+-*/%a-zA-Z_\=<>!?&`]
    lparen  : '('
    rparen  : ')'