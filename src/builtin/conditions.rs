use std::fmt;
use lval::LVal;
use lenv::LEnv;
use eval::eval;


enum OrderingType {
    Less,
    LessEqual,
    GreaterEqual,
    Greater
}

impl fmt::Show for OrderingType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            OrderingType::Less         => write!(f, "{}", "<"),
            OrderingType::LessEqual    => write!(f, "{}", "<="),
            OrderingType::GreaterEqual => write!(f, "{}", ">="),
            OrderingType::Greater      => write!(f, "{}", ">")
        }
    }
}


pub fn builtin_lt(_: &mut LEnv, arg: LVal) -> LVal {
    builtin_ord(OrderingType::Less, arg)
}


pub fn builtin_le(_: &mut LEnv, arg: LVal) -> LVal {
    builtin_ord(OrderingType::LessEqual, arg)
}


pub fn builtin_ge(_: &mut LEnv, arg: LVal) -> LVal {
    builtin_ord(OrderingType::GreaterEqual, arg)
}


pub fn builtin_gt(_: &mut LEnv, arg: LVal) -> LVal {
    builtin_ord(OrderingType::Greater, arg)
}


pub fn builtin_ord(ord: OrderingType, arg: LVal) -> LVal {
    use self::OrderingType::*;

    let mut args = arg.into_values();

    builtin_assert!(ord: args.len() >= 2u);
    builtin_assert!(ord: args[*] is LVal::Num(..) "number");

    let mut result = true;
    let mut x = args.remove(0).unwrap().into_num();

    for arg in args.into_iter() {
        let y = arg.into_num();

        match ord {
            Less         => { result &= x <  y; },
            LessEqual    => { result &= x <= y; },
            GreaterEqual => { result &= x >= y; },
            Greater      => { result &= x >  y; }
        }

        x = y;
    }

    LVal::Num(result as f64)
}


enum CmpType {
    Eq,
    Neq
}

impl fmt::Show for CmpType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CmpType::Eq  => write!(f, "{}", "=="),
            CmpType::Neq => write!(f, "{}", "!=")
        }
    }
}


pub fn builtin_eq(_: &mut LEnv, arg: LVal) -> LVal {
    builtin_cmp(CmpType::Eq, arg)
}


pub fn builtin_neq(_: &mut LEnv, arg: LVal) -> LVal {
    builtin_cmp(CmpType::Neq, arg)
}


pub fn builtin_cmp(cmp: CmpType, arg: LVal) -> LVal {
    use self::CmpType::*;

    let args = arg.into_values();

    builtin_assert!(cmp: args.len() == 2u);

    let ref o1 = args[0];
    let ref o2 = args[1];

    match cmp {
        Eq  => LVal::Num((o1 == o2) as f64),
        Neq => LVal::Num((o1 != o2) as f64),
    }
}


pub fn builtin_if(env: &mut LEnv, arg: LVal) -> LVal {
    let mut args = arg.into_values();

    builtin_assert!("if": args.len() >= 2u);
    builtin_assert!("if": args.len() <= 3u);
    builtin_assert!("if": args[0u] is LVal::Num(_) "number");
    builtin_assert!("if": args[1u] is LVal::QExpr(_) "q-expression");

    if args.len() == 3 {
        builtin_assert!("if": args[2u] is LVal::QExpr(_) "q-expression");
    }

    let branch = if args.remove(0).unwrap().into_num() != 0. {
        args.remove(0).unwrap()
    } else if args.len() == 2 {
        args.remove(1).unwrap()
    } else {
        LVal::sexpr()
    };

    eval(env, LVal::SExpr(branch.into_values()))
}


pub fn builtin_and(_: &mut LEnv, arg: LVal) -> LVal {
    let args = arg.into_values();

    builtin_assert!("and": args.len() == 2u);
    builtin_assert!("and": args[*] is LVal::Num(_) "number");

    let n1 = (*args[0].as_num()) != 0.;
    let n2 = (*args[1].as_num()) != 0.;

    LVal::Num((n1 && n2) as f64)
}


pub fn builtin_or(_: &mut LEnv, arg: LVal) -> LVal {
    let args = arg.into_values();

    builtin_assert!("or": args.len() == 2u);
    builtin_assert!("or": args[*] is LVal::Num(_) "number");

    let n1 = (*args[0].as_num()) != 0.;
    let n2 = (*args[1].as_num()) != 0.;

    LVal::Num((n1 || n2) as f64)
}


pub fn builtin_not(_: &mut LEnv, arg: LVal) -> LVal {
    let args = arg.into_values();

    builtin_assert!("not": args.len() == 1u);
    builtin_assert!("not": args[0u] is LVal::Num(_) "number");

    let n = (*args[0].as_num()) != 0.;

    LVal::Num((!n) as f64)
}