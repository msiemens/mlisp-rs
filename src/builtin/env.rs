use std::fmt;
use lval::LVal;
use lenv::LEnv;
use eval::eval;


#[allow(unused_variables)]
pub fn builtin_lambda(env: &mut LEnv, arg: LVal) -> LVal {
    let mut args = arg.into_values();

    builtin_assert!("\\": args.len() == 2u);
    builtin_assert!("\\": args[0u] is LVal::QExpr(_) "q-expression");
    builtin_assert!("\\": args[1u] is LVal::QExpr(_) "q-expression");

    let formals = args.remove(0).unwrap();
    let body    = args.remove(0).unwrap();

    for argument in formals.as_values().iter() {
        if let LVal::Sym(_) = *argument {}
        else {
            err!("cannot use non-symbol as argument: `{}`", argument)
        }
    }

    LVal::lambda(formals, body)
}


enum VariableLocation {
    Local,
    Global
}

impl fmt::Show for VariableLocation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            VariableLocation::Local => write!(f, "{}", "="),
            VariableLocation::Global => write!(f, "{}", "def")
        }
    }
}


pub fn builtin_var(loc: VariableLocation, env: &mut LEnv, arg: LVal) -> LVal {
    let mut args = arg.into_values();

    builtin_assert!(loc: args.len() >= 1u);
    builtin_assert!(loc: args[0u] is LVal::QExpr(_) "q-expression");

    let symbols = args.remove(0).unwrap().into_values();

    // Ensure all elements of first list are symbols
    for symbol in symbols.iter() {
        if let LVal::Sym(_) = *symbol {}
        else {
            err!("cannot `def`ine non-symbol: `{}`", symbol)
       }
    }

    // Check that number of symbols and values matches
    if symbols.len() != args.len() {
        err!("`def` called with number of symbols ({}) != number of values ({})",
             symbols.len(), args.len())
    }

    for (symbol, value) in symbols.iter().zip(args.into_iter()) {
        match loc {
            VariableLocation::Local => {
                env.put(symbol.clone(), value);
            },
            VariableLocation::Global => {
                env.def(symbol.clone(), value);
            }
        }
    }

    LVal::sexpr()
}


pub fn builtin_def(env: &mut LEnv, arg: LVal) -> LVal {
    builtin_var(VariableLocation::Global, env, arg)
}


pub fn builtin_put(env: &mut LEnv, arg: LVal) -> LVal {
    builtin_var(VariableLocation::Local, env, arg)
}


pub fn builtin_eval(env: &mut LEnv, arg: LVal) -> LVal {
    let mut args = arg.into_values();

    builtin_assert!("eval": args.len() == 1u);
    builtin_assert!("eval": args[0u] is LVal::QExpr(_) "q-expression");

    // Take 1st argument
    let qexpr = args.remove(0).unwrap();

    // Evaluate it
    eval(env, LVal::SExpr(qexpr.into_values()))
}