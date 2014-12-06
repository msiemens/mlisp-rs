use lval::LVal;
use lenv::LEnv;


#[allow(unused_variables)]
pub fn builtin_head(env: &mut LEnv, arg: LVal) -> LVal {
    let mut args = arg.into_values();

    builtin_assert!("head": args.len() == 1u);
    builtin_assert!("head": args[0u] != {});

    // Take 1st argument
    let mut qexpr = args.remove(0).unwrap().into_values();

    // Take 1st element and return it
    for _ in range(0, qexpr.len() - 1) {
        qexpr.remove(1);
    }

    LVal::QExpr(qexpr)
}


#[allow(unused_variables)]
pub fn builtin_tail(env: &mut LEnv, arg: LVal) -> LVal {
    let mut args = arg.into_values();

    builtin_assert!("tail": args.len() == 1u);
    builtin_assert!("tail": args[0u] != {});

    // Take 1st argument
    let qexpr = args.remove(0).unwrap();

    // Remove 1st element and return the tail
    LVal::QExpr(qexpr.into_values().into_iter().skip(1).collect())

}


#[allow(unused_variables)]
pub fn builtin_list(env: &mut LEnv, arg: LVal) -> LVal {
    LVal::QExpr(arg.into_values())
}


#[allow(unused_variables)]
pub fn builtin_join(env: &mut LEnv, arg: LVal) -> LVal {
    let mut args = arg.into_values();

    builtin_assert!("join": args[*] is LVal::QExpr(_) "q-expression");

    let mut joined = args.remove(0).unwrap();

    for arg in args.into_iter() {
        joined.extend(arg);
    }

    joined
}


#[allow(unused_variables)]
pub fn builtin_cons(env: &mut LEnv, arg: LVal) -> LVal {
    let mut args = arg.into_values();

    builtin_assert!("cons": args.len() == 2u);
    builtin_assert!("cons": args[1u] is LVal::QExpr(_) "q-expression");

    let mut value = LVal::qexpr();
    value.append(args.remove(0).unwrap());
    value.extend(LVal::QExpr(args));

    value
}