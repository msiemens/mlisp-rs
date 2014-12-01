use lval::LVal;
use builtin::builtin;


/// Evaluate a calculation
pub fn eval(node: LVal) -> LVal {
    match node {
        LVal::SExpr(_) => eval_sexpr(node),
        node => node
    }
}

/// Evaluate a SExpression
fn eval_sexpr(node: LVal) -> LVal {
    let values = match node {
        LVal::SExpr(values) => values,
        _ => panic!("eval_sexpr got a non-sexpr: {}", node)
    };

    // Evaluate values & check for errors
    let mut values: Vec<LVal> = values.into_iter()
        .map(|v| {
            match eval(v) {
                LVal::Err(msg) => return LVal::err(msg),
                v => v
            }
        })
        .collect();

    // Handle empty expression
    if values.len() == 0 {
        return LVal::SExpr(vec![])
    }

    // Handle single expression
    if values.len() == 1 {
        return values.remove(0).unwrap()
    }

    // Ensure first element is a symbol
    let f = match values.remove(0).unwrap() {
        LVal::Sym(s) => s,
        _ => err!("S-Expression doesn't start with symbol")
    };

    // Call with builtin operator
    builtin(f, LVal::SExpr(values))
}


#[cfg(test)]
mod test {
    use super::eval;
    use lval::LVal;

    #[test]
    fn eval_not_a_symbol() {
        assert_eq!(
            eval(LVal::SExpr(vec![
                LVal::num(2.0),
                LVal::num(2.0),
                LVal::num(2.0),
            ])),
            LVal::err("S-Expression doesn't start with symbol".into_string())
        )
    }
}