use lval::LVal;
use lenv::LEnv;


pub fn builtin_head(_: &mut LEnv, mut args: Vec<LVal>) -> LVal {
    builtin_assert!("head"; args.len() == 1);
    builtin_assert!("head"; args[0] != {});

    // Take 1st argument
    let mut qexpr = args.remove(0).into_values();

    // Take 1st element and return it
    for _ in 0 .. qexpr.len() - 1 {
        qexpr.remove(1);
    }

    LVal::QExpr(qexpr)
}


pub fn builtin_tail(_: &mut LEnv, mut args: Vec<LVal>) -> LVal {
    builtin_assert!("tail"; args.len() == 1);
    builtin_assert!("tail"; args[0] != {});

    // Take 1st argument
    let qexpr = args.remove(0);

    // Remove 1st element and return the tail
    LVal::QExpr(qexpr.into_values().into_iter().skip(1).collect())

}


pub fn builtin_list(_: &mut LEnv, args: Vec<LVal>) -> LVal {
    LVal::QExpr(args)
}


pub fn builtin_join(_: &mut LEnv, mut args: Vec<LVal>) -> LVal {
    builtin_assert!("join"; args[*] is qexpr);

    let mut joined = args.remove(0);

    for arg in args {
        joined.extend(arg);
    }

    joined
}


pub fn builtin_cons(_: &mut LEnv, mut args: Vec<LVal>) -> LVal {
    builtin_assert!("cons"; args.len() == 2);
    builtin_assert!("cons"; args[1] is qexpr);

    let mut value = LVal::qexpr();
    value.append(args.remove(0));
    value.extend(LVal::QExpr(args));

    value
}