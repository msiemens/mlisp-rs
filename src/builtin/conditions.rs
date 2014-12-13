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


pub fn builtin_lt(_: &mut LEnv, args: Vec<LVal>) -> LVal {
    builtin_ord(OrderingType::Less, args)
}


pub fn builtin_le(_: &mut LEnv, args: Vec<LVal>) -> LVal {
    builtin_ord(OrderingType::LessEqual, args)
}


pub fn builtin_ge(_: &mut LEnv, args: Vec<LVal>) -> LVal {
    builtin_ord(OrderingType::GreaterEqual, args)
}


pub fn builtin_gt(_: &mut LEnv, args: Vec<LVal>) -> LVal {
    builtin_ord(OrderingType::Greater, args)
}


pub fn builtin_ord(ord: OrderingType, args: Vec<LVal>) -> LVal {
    use self::OrderingType::*;

    builtin_assert!(ord: args.len() >= 2u);
    builtin_assert!(ord: args[*] is number);

    let mut result = true;
    let mut x = *args[0].as_num();

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


pub fn builtin_eq(_: &mut LEnv, args: Vec<LVal>) -> LVal {
    builtin_cmp(CmpType::Eq, args)
}


pub fn builtin_neq(_: &mut LEnv, args: Vec<LVal>) -> LVal {
    builtin_cmp(CmpType::Neq, args)
}


pub fn builtin_cmp(cmp: CmpType, args: Vec<LVal>) -> LVal {
    use self::CmpType::*;

    builtin_assert!(cmp: args.len() == 2u);

    let ref o1 = args[0];
    let ref o2 = args[1];

    match cmp {
        Eq  => LVal::Num((o1 == o2) as f64),
        Neq => LVal::Num((o1 != o2) as f64),
    }
}


pub fn builtin_if(env: &mut LEnv, mut args: Vec<LVal>) -> LVal {
    builtin_assert!("if": args.len() >= 2u);
    builtin_assert!("if": args.len() <= 3u);
    builtin_assert!("if": args[0u] is number);
    builtin_assert!("if": args[1u] is qexpr);

    if args.len() == 3 {
        builtin_assert!("if": args[2u] is qexpr);
    }

    let test = args.remove(0).unwrap().into_num();
    let consequence = args.remove(0).unwrap();
    let alternative = if args.len() == 1 {
        args.remove(0).unwrap()
    } else {
        LVal::sexpr()
    };

    let branch = if test != 0. {
        consequence
    } else {
        alternative
    };

    eval(env, LVal::SExpr(branch.into_values()))
}


pub fn builtin_and(_: &mut LEnv, args: Vec<LVal>) -> LVal {
    builtin_assert!("and": args.len() == 2u);
    builtin_assert!("and": args[*] is number);

    let n1 = (*args[0].as_num()) != 0.;
    let n2 = (*args[1].as_num()) != 0.;

    LVal::Num((n1 && n2) as f64)
}


pub fn builtin_or(_: &mut LEnv, args: Vec<LVal>) -> LVal {
    builtin_assert!("or": args.len() == 2u);
    builtin_assert!("or": args[*] is number);

    let n1 = (*args[0].as_num()) != 0.;
    let n2 = (*args[1].as_num()) != 0.;

    LVal::Num((n1 || n2) as f64)
}


pub fn builtin_not(_: &mut LEnv, args: Vec<LVal>) -> LVal {
    builtin_assert!("not": args.len() == 1u);
    builtin_assert!("not": args[0u] is number);

    let n = (*args[0].as_num()) != 0.;

    LVal::Num((!n) as f64)
}