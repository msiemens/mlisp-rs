use lval::LVal;
use eval::eval;

macro_rules! builtin_assert(

    ($func:expr: ASSERT LENGTH EQ, $length:expr, $expected:expr) => {
        if $length < $expected {
            err!("`{}` called with too few arguments: expected {}, got {}",
                        $func, $expected, $length)
        } else if $length > $expected {
            err!("`{}` called with too many arguments: expected {}, got {}",
                        $func, $expected, $length)
        }
    };

    ($func:expr: ASSERT LENGTH GE, $length:expr, $expected:expr) => {
        if $length < $expected {
            err!("`{}` called with too few arguments: expected at least {}, got {}",
                        $func, $expected, $length)
        }
    };

    ($func:expr: ASSERT LENGTH LE, $length:expr, $expected:expr) => {
        if $length > $expected {
            err!("`{}` called with too few arguments: expected at least {}, got {}",
                        $func, $expected, $length)
        }
    };

    // FIXME: Find a solution without typ_name
    ($func:expr: ASSERT TYPE: $element:expr, $typ:pat $typ_name:expr) => {
        match $element {
            $typ => {},
            _ => {
                err!("`{}` called with wrong argument type: expected {}, got {}",
                            $func, $typ_name, $element.type_name())
            }
        }
    };

    // --------------------------------------------------------------------------

    ($func:expr: $args:ident . len() == $expected:expr) => {
        builtin_assert!($func: ASSERT LENGTH EQ, $args.len(), $expected)
    };

    ($func:expr: $args:ident . len() >= $expected:expr) => {
        builtin_assert!($func: ASSERT LENGTH GE, $args.len(), $expected)
    };

    ($func:expr: $args:ident . len() <= $expected:expr) => {
        builtin_assert!($func: ASSERT LENGTH LE, $args.len(), $expected)
    };

    ($func:expr: $args:ident * is $typ:pat $typ_name:expr) => {
        for arg in $args.iter() {
            builtin_assert!($func: ASSERT TYPE: arg, &$typ $typ_name);
        }
    };

    ($func:expr: $args:ident [ $i:expr ] != {}) => {
        {
            builtin_assert!($func: ASSERT TYPE: $args[$i], LVal::QExpr(_) "q-expr");
            if $args[$i].as_values().len() == 0 {
                err!("`{}` called with empty q-expr", $func)
            }
        }
    };

    ($func:expr: $args:ident [ $i:expr ] is $typ:pat $typ_name:expr) => {
        builtin_assert!($func: ASSERT TYPE: $args[$i], $typ $typ_name);
    };
)


pub fn builtin(op: String, args: LVal) -> LVal {
    match &*op {
        "head" => builtin_head(args),
        "tail" => builtin_tail(args),
        "list" => builtin_list(args),
        "eval" => builtin_eval(args),
        "join" => builtin_join(args),
        "cons" => builtin_cons(args),
        "+" | "-" | "*" | "/" | "%" => builtin_op(op, args),
        _ => err!("unknown function: {}", op)
    }
}


fn builtin_op(op: String, args: LVal) -> LVal {
    let mut args = args.into_values();

    // Make sure all arguments are numbers
    builtin_assert!(op: args* is LVal::Num(_) "number");
    builtin_assert!(op: args.len() >= 1u);

    let mut x = args.remove(0).unwrap().into_num();

    // Perform unary minus operation
    if &*op == "-" && args.len() == 0 {
        return LVal::num(-1.0 * x)
    }

    builtin_assert!(op: args.len() >= 1u);

    for arg in args.into_iter() {
        let y = arg.into_num();

        x = match &*op {
            "+" => x + y,
            "-" => x - y,
            "*" => x * y,
            "/" => {
                if y == 0.0 { err!("division by zero!") }
                x / y
            },
            "%" => {
                if y == 0.0 { err!("division by zero!") }
                x % y
            },
            "min" => if x > y { y } else { x },
            "max" => if x > y { x } else { y },
            _ => err!("invalid operator: {}", op)
        };
    }

    LVal::num(x)
}


fn builtin_head(arg: LVal) -> LVal {
    let mut args = arg.into_values();

    builtin_assert!("head": args.len() == 1u);
    builtin_assert!("head": args[0] != {});

    // Take 1st argument
    let mut qexpr = args.remove(0).unwrap().into_values();

    // Take 1st element and return it
    qexpr.remove(0).unwrap()
}


fn builtin_tail(arg: LVal) -> LVal {
    let mut args = arg.into_values();

    builtin_assert!("tail": args.len() == 1u);
    builtin_assert!("tail": args[0] != {});

    // Take 1st argument
    let qexpr = args.remove(0).unwrap();

    // Remove 1st element and return the tail
    LVal::QExpr(qexpr.into_values().into_iter().skip(1).collect())

}


fn builtin_list(arg: LVal) -> LVal {
    LVal::QExpr(arg.into_values())
}


fn builtin_eval(arg: LVal) -> LVal {
    let mut args = arg.into_values();

    builtin_assert!("eval": args.len() == 1u);
    builtin_assert!("eval": args[0] is LVal::QExpr(_) "q-expression");

    // Take 1st argument
    let qexpr = args.remove(0).unwrap();
    eval(LVal::SExpr(qexpr.into_values()))
}


fn builtin_join(arg: LVal) -> LVal {
    let mut args = arg.into_values();

    builtin_assert!("join": args* is LVal::QExpr(_) "q-expression");

    let mut joined = args.remove(0).unwrap();

    for arg in args.into_iter() {
        joined.extend(arg);
    }

    joined
}


fn builtin_cons(arg: LVal) -> LVal {
    let mut args = arg.into_values();

    builtin_assert!("cons": args.len() == 2u);
    builtin_assert!("cons": args[1] is LVal::QExpr(_) "q-expression");

    let mut value = LVal::qexpr();
    value.append(args.remove(0).unwrap());
    value.extend(LVal::QExpr(args));

    value
}


#[cfg(test)]
mod test {
    use lval::LVal;
    use super::builtin_op;

    #[test]
    fn builtin_op_few_arguments() {
        assert_eq!(
            builtin_op("+".into_string(), LVal::SExpr(vec![
                LVal::num(2.0)
            ])),
            LVal::err("`+` called with too few arguments: expected at least 1, got 0".into_string())
        )
    }

    #[test]
    fn builtin_op_plus() {
        assert_eq!(
            builtin_op("+".into_string(), LVal::SExpr(vec![
                LVal::num(2.0),
                LVal::num(3.0),
                LVal::num(4.0),
                LVal::num(5.0)
            ])),
            LVal::num(14.0)
        )
    }

    #[test]
    fn builtin_op_minus() {
        assert_eq!(
            builtin_op("-".into_string(), LVal::SExpr(vec![
                LVal::num(2.0),
                LVal::num(3.0)
            ])),
            LVal::num(-1.0)
        )
    }

    #[test]
    fn builtin_op_minus_unary() {
        assert_eq!(
            builtin_op("-".into_string(), LVal::SExpr(vec![
                LVal::num(2.0)
            ])),
            LVal::num(-2.0)
        )
    }

    #[test]
    fn builtin_op_mul() {
        assert_eq!(
            builtin_op("*".into_string(), LVal::SExpr(vec![
                LVal::num(2.0),
                LVal::num(3.0)
            ])),
            LVal::num(6.0)
        )
    }

    #[test]
    fn builtin_op_div() {
        assert_eq!(
            builtin_op("/".into_string(), LVal::SExpr(vec![
                LVal::num(2.0),
                LVal::num(3.0)
            ])),
            LVal::num(2.0 / 3.0)
        )
    }

    #[test]
    fn builtin_op_modulo() {
        assert_eq!(
            builtin_op("%".into_string(), LVal::SExpr(vec![
                LVal::num(15.0),
                LVal::num(12.0)
            ])),
            LVal::num(3.0)
        )
    }

    #[test]
    fn builtin_op_min() {
        assert_eq!(
            builtin_op("min".into_string(), LVal::SExpr(vec![
                LVal::num(2.0),
                LVal::num(3.0)
            ])),
            LVal::num(2.0)
        )
    }

    #[test]
    fn builtin_op_max() {
        assert_eq!(
            builtin_op("max".into_string(), LVal::SExpr(vec![
                LVal::num(2.0),
                LVal::num(3.0)
            ])),
            LVal::num(3.0)
        )
    }
}