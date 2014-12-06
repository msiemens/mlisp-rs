use std::mem;
use lval::{LVal, LBuiltin};
use lenv::LEnv;
use util::stringify_vec;


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

    // Handle function call
    match values.remove(0).unwrap() {
        LVal::Function {
            env: mut lenv,
            mut args,
            body
        } => {
            let given = values.len();
            let total = args.len();

            while values.len() > 0 {
                if args.len() == 0 {
                    err!("function (\\ {} {}) passed too many arguments: expecteded {}, got {}",
                         stringify_vec(&args), stringify_vec(&body), given, total)
                }

                let mut symbol = args.remove(0).unwrap();

                if *symbol.as_sym() == "..." {
                    if args.len() != 1 {
                        err!("invalid function arguments: `...` is not followed by a single symbol")
                    }

                    // Bind varargs
                    lenv.put(args.remove(0).unwrap(), LVal::QExpr(values));
                    break
                }

                let value = values.remove(0).unwrap();
                lenv.put(symbol, value);
            }

            // If `...` has not been processed yet, bind it to an empty list
            if args.len() > 0 && *args.get_mut(0).unwrap().as_sym() == "..." {
                if args.len() != 2 {
                    err!("invalid function arguments: `...` is not followed by a single symbol")
                }

                // Delete `...`
                args.remove(0);

                let symbol = args.remove(0).unwrap();
                let value = LVal::qexpr();

                lenv.put(symbol, value);
            }

            // If all arguments have been bound
            if args.len() == 0 {
                //lenv.parent = Some(Rc::new(RefCell::new(env)));
                unsafe {
                    lenv.parent = Some(mem::transmute(env));
                }
                eval(&mut lenv, LVal::SExpr(body))
            } else {
                LVal::Function {
                    env: lenv,
                    args: args,
                    body: body
                }
            }
        },
        LVal::Builtin(LBuiltin(f)) => {
            // Call with builtin operator
            f(env, LVal::SExpr(values))
        },
        LVal::Sym(ref name) => {
            // FIXME: Why is this needed? Why may a symbol not be already evaluated?
            if let LVal::Builtin(LBuiltin(f)) = env.get(name[]) {
                f(env, LVal::SExpr(values))
            }
            else {
                err!("first element is not a function: {}", name)
            }
        },
        LVal::Err(msg) => err!(msg),
        first => err!("first element is not a function but a {} (= `{}`)",
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
            LVal::err("first element is not a function".into_string())
        )
    }
}