    # AST
    program : <expr>*
    expr    : <number> | <symbol> | <string> | <sexpr>
    sexpr   : <lparen> <expr>* <rparen>
    number  : (<minus> [0-9]+ | [0-9]+)

    # Tokens
    string  : " ([^"]* | \" ) "
    symbol  : [+-*/%a-zA-Z_\=<>!?&`]
    lparen  : '('
    rparen  : ')'