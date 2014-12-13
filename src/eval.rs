use lval::{LVal, LBuiltin};
use lenv::LEnv;
use util::stringify_vec;


/// Evaluate a lvalue
pub fn eval(env: &mut LEnv, node: LVal) -> LVal {
    match node {
        LVal::SExpr(_) => eval_sexpr(env, node),
        LVal::Sym(ref name) => env.get(name[]),
        node => node
    }
}

/// Evaluate an expression
fn eval_sexpr(env: &mut LEnv, node: LVal) -> LVal {
    let values = node.into_values();

    // Evaluate values & check for errors
    let mut values: Vec<_> = values.into_iter()
        .map(|val| eval(env, val))
        .collect();

    // TODO: Return early if error is found instead of checking here
    for val in values.iter() {
        if let LVal::Err(..) = *val {
            return val.clone()
        }
    }

    // Handle empty expression: Return S-Expr
    if values.len() == 0 {
        return LVal::SExpr(vec![])
    }

    // Handle single expression: Return the value itself
    if values.len() == 1 {
        return values.remove(0).unwrap()
    }

    // Handle function calls
    match values.remove(0).unwrap() {

        // Call a lambda function
        LVal::Function {
            env: mut lenv,
            mut formals,
            body
        } => {
            let given = values.len();
            let total = formals.len();

            // Bind argument values to formal arguments
            while values.len() > 0 {
                if formals.len() == 0 {
                    // No more arguments to bind
                    err!("function (\\ {} {}) passed too many arguments: expecteded {}, got {}",
                         stringify_vec(&formals), stringify_vec(&body), given, total)
                }

                let symbol = formals.remove(0).unwrap();

                // Process varargs
                if *symbol.as_sym() == "..." {
                    if formals.len() != 1 {
                        err!("invalid function arguments: `...` is not followed by a single symbol")
                    }

                    // Bind vararg
                    lenv.put(formals.remove(0).unwrap(), LVal::QExpr(values));
                    break
                }

                let value = values.remove(0).unwrap();
                lenv.put(symbol, value);
            }

            // If `...` has not been processed yet, bind it to an empty list
            if formals.len() > 0 && *formals.get_mut(0).unwrap().as_sym() == "..." {
                if formals.len() != 2 {
                    err!("invalid function arguments: `...` is not followed by a single symbol")
                }

                // Delete `...`
                formals.remove(0);

                let symbol = formals.remove(0).unwrap();
                let value = LVal::qexpr();

                lenv.put(symbol, value);
            }

            if formals.len() == 0 {
                // If all arguments have been bound: execute
                let parent: *mut LEnv = env;
                lenv.parent = Some(parent);
                eval(&mut lenv, LVal::SExpr(body))
            } else {
                // Else: Return partially evaluated function
                LVal::Function {
                    env: lenv,
                    formals: formals,
                    body: body
                }
            }
        },

        // Call a builtin
        LVal::Builtin(LBuiltin(f)) => {
            // Call with builtin operator
            f(env, values)
        },

        // FIXME: Why is this needed? Why may a symbol not be already evaluated?
        LVal::Sym(ref name) => {
            if let LVal::Builtin(LBuiltin(f)) = env.get(name[]) {
                f(env, values)
            }
            else {
                err!("first element is not a function: {}", name)
            }
        },

        first => err!("first element is not a function but {}: `{}`",
                      first.type_name(), first)
    }
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
            LVal::err("first element is not a function but a number: `2`".into_string())
        )
    }
}