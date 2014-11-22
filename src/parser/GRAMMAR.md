    # AST
    program : <expr>*
    expr    : <number> | <symbol> | <sexpr>
    sexpr   : <lparen> <expr>* <rparen>
    number  : (<minus> [0-9]+ | [0-9]+)

    # Tokens
    symbol  : <plus> | <minus> | <mul> | <div> | [a-zA-Z_]
    lparen  : '('
    rparen  : ')'
    plus    : '+'
    minus   : '-'
    mul     : '*'
    dif     : '/'