use lval::{LVal, LBuiltin};
use lenv::LEnv;


/// Evaluate a calculation
pub fn eval(env: &mut LEnv, node: LVal) -> LVal {
    match node {
        LVal::SExpr(_) => eval_sexpr(env, node),
        LVal::Sym(ref name) => env.get(name[]),
        node => node
    }
}

/// Evaluate a SExpression
fn eval_sexpr(env: &mut LEnv, node: LVal) -> LVal {
    let values = match node {
        LVal::SExpr(values) => values,
        _ => panic!("eval_sexpr got a non-sexpr: {}", node)
    };

    // Evaluate values & check for errors
    let mut values: Vec<LVal> = values.into_iter()
        .map(|val| {
            match eval(env, val) {
                LVal::Err(msg) => return LVal::err(msg),
                val => val
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
        LVal::Builtin(LBuiltin(f)) => f,
        LVal::Sym(ref name) => {
            // FIXME: Why is this needed? Why may a symbol not be already evaluated?
            if let LVal::Builtin(LBuiltin(f)) = env.get(name[]) { f }
               else { err!("first element is not a function") }  // TODO: print it!
        },
        _ => err!("first element is not a function")  // TODO: print it!
    };

    // Call with builtin operator
    f(env, LVal::SExpr(values))
}


#[cfg(test)]
mod test {
    use super::eval;
    use lval::LVal;
    use lenv::LEnv;

    #[test]
    fn eval_not_a_symbol() {
        assert_eq!(
            eval(&mut LEnv::new(), LVal::SExpr(vec![
                LVal::num(2.0),
                LVal::num(2.0),
                LVal::num(2.0),
            ])),
            LVal::err("first element is not a function".into_string())
        )
    }
}