use std::fmt;
use std::fs::File;
use std::io::Read;
use lval::LVal;
use lenv::LEnv;
use eval::eval;
use parser::Parser;


pub fn builtin_lambda(_: &mut LEnv, mut args: Vec<LVal>) -> LVal {
    builtin_assert!("\\"; args.len() == 2);
    builtin_assert!("\\"; args[0] is qexpr);
    builtin_assert!("\\"; args[1] is qexpr);

    let formals = args.remove(0);
    let body    = args.remove(0);

    for argument in formals.as_values() {
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

impl fmt::Display for VariableLocation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            VariableLocation::Local => write!(f, "{}", "="),
            VariableLocation::Global => write!(f, "{}", "def")
        }
    }
}


pub fn builtin_var(loc: VariableLocation, env: &mut LEnv, mut args: Vec<LVal>) -> LVal {
    builtin_assert!(loc; args.len() >= 1);
    builtin_assert!(loc; args[0] is qexpr);

    let symbols = args.remove(0).into_values();

    // Ensure all elements of first list are symbols
    for symbol in &symbols {
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


pub fn builtin_def(env: &mut LEnv, args: Vec<LVal>) -> LVal {
    builtin_var(VariableLocation::Global, env, args)
}


pub fn builtin_put(env: &mut LEnv, args: Vec<LVal>) -> LVal {
    builtin_var(VariableLocation::Local, env, args)
}


pub fn builtin_eval(env: &mut LEnv, mut args: Vec<LVal>) -> LVal {
    builtin_assert!("eval"; args.len() == 1);
    builtin_assert!("eval"; args[0] is qexpr);

    // Take 1st argument
    let qexpr = args.remove(0);

    // Evaluate it
    eval(env, LVal::SExpr(qexpr.into_values()))
}


pub fn builtin_load(env: &mut LEnv, mut args: Vec<LVal>) -> LVal {
    builtin_assert!("load"; args.len() == 1);
    builtin_assert!("load"; args[0] is string);

    // Read the file
    let filename = args.remove(0).into_str();
    let mut file = match File::open(&Path::new(&filename)) {
        Ok(f) => f,
        Err(err) => return LVal::err(format!("{}", err))
    };

    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(s) => s,
        Err(err) => return LVal::err(format!("{}", err))
    };

    // Parse it
    let ast = match Parser::parse(&contents, &filename) {
        Ok(lval) => lval,
        Err(err) => return LVal::err(format!("{:?}", err))
    };
    let exprs = LVal::from_ast(ast).into_values();

    // Run it
    for val in exprs {
        let result = eval(env, val);

        if let LVal::Err(..) = result {
            result.println(env);
        }
    }

    LVal::sexpr()
}


pub fn builtin_error(_: &mut LEnv, mut args: Vec<LVal>) -> LVal {
    builtin_assert!("error"; args.len() == 1);
    builtin_assert!("error"; args[0] is string);

    let msg = args.remove(0).into_str();
    LVal::Err(msg)
}


pub fn builtin_println(env: &mut LEnv, mut args: Vec<LVal>) -> LVal {
    builtin_assert!("println"; args.len() == 1);

    let lval = args.remove(0);
    lval.println(env);

    LVal::sexpr()
}