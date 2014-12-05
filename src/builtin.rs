use std::fmt;
use lval::LVal;
use lenv::LEnv;
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
    ($func:expr: ASSERT TYPE: $element:expr, $pos:expr, $typ:pat $typ_name:expr) => {
        match $element {
            $typ => {},
            _ => {
                err!("`{}` called with wrong type for argument {}: expected {}, got {}",
                            $func, $pos + 1, $typ_name, $element.type_name())
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

    ($func:expr: $args:ident [*] is $typ:pat $typ_name:expr) => {
        for (i, arg) in $args.iter().enumerate() {
            builtin_assert!($func: ASSERT TYPE: arg, i, &$typ $typ_name);
        }
    };

    ($func:expr: $args:ident [ $i:expr ] != {}) => {
        {
            builtin_assert!($func: ASSERT TYPE: $args[$i], $i, LVal::QExpr(_) "q-expr");
            if $args[$i].as_values().len() == 0 {
                err!("`{}` called with empty q-expr", $func)
            }
        }
    };

    ($func:expr: $args:ident [ $i:expr ] is $typ:pat $typ_name:expr) => {
        builtin_assert!($func: ASSERT TYPE: $args[$i], $i, $typ $typ_name);
    };
)


pub fn initialize(env: &mut LEnv) {
    env.put(LVal::sym("def"), LVal::func(builtin_def));
    env.put(LVal::sym("eval"), LVal::func(builtin_eval));

    // Lists
    env.put(LVal::sym("head"), LVal::func(builtin_head));
    env.put(LVal::sym("tail"), LVal::func(builtin_tail));
    env.put(LVal::sym("list"), LVal::func(builtin_list));
    env.put(LVal::sym("join"), LVal::func(builtin_join));
    env.put(LVal::sym("cons"), LVal::func(builtin_cons));

    // Math
    env.put(LVal::sym("+"), LVal::func(builtin_add));
    env.put(LVal::sym("-"), LVal::func(builtin_sub));
    env.put(LVal::sym("*"), LVal::func(builtin_mul));
    env.put(LVal::sym("/"), LVal::func(builtin_div));
    env.put(LVal::sym("%"), LVal::func(builtin_mod));
    env.put(LVal::sym("min"), LVal::func(builtin_min));
    env.put(LVal::sym("max"), LVal::func(builtin_max));
}


// --- Functions: Math ----------------------------------------------------------

#[deriving(PartialEq)]
enum ArithmeticOp {
    ADD, SUB, MUL, DIV, MOD,
    MIN, MAX
}

impl fmt::Show for ArithmeticOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::ArithmeticOp::*;

        match *self {
            ADD => write!(f, "{}", "+"),
            SUB => write!(f, "{}", "-"),
            MUL => write!(f, "{}", "*"),
            DIV => write!(f, "{}", "/"),
            MOD => write!(f, "{}", "%"),
            MIN => write!(f, "{}", "min"),
            MAX => write!(f, "{}", "max")
        }
    }
}


fn builtin_op(op: ArithmeticOp, args: LVal) -> LVal {
    use self::ArithmeticOp::*;

    let mut args = args.into_values();

    // Make sure all arguments are numbers
    builtin_assert!(op: args[*] is LVal::Num(_) "number");
    builtin_assert!(op: args.len() >= 1u);

    let mut x = args.remove(0).unwrap().into_num();

    // Perform unary minus operation
    if op == SUB && args.len() == 0 {
        return LVal::num(-1.0 * x)
    }

    builtin_assert!(op: args.len() >= 1u);

    for arg in args.into_iter() {
        let y = arg.into_num();

        x = match op {
            ADD => x + y,
            SUB => x - y,
            MUL => x * y,
            DIV => {
                if y == 0.0 { err!("division by zero!") }
                x / y
            },
            MOD => {
                if y == 0.0 { err!("division by zero!") }
                x % y
            },
            MIN => if x > y { y } else { x },
            MAX => if x > y { x } else { y }
        };
    }

    LVal::num(x)
}


#[allow(unused_variables)]
fn builtin_add(env: &mut LEnv, arg: LVal) -> LVal {
    builtin_op(ArithmeticOp::ADD, arg)
}


#[allow(unused_variables)]
fn builtin_sub(env: &mut LEnv, arg: LVal) -> LVal {
    builtin_op(ArithmeticOp::SUB, arg)
}


#[allow(unused_variables)]
fn builtin_mul(env: &mut LEnv, arg: LVal) -> LVal {
    builtin_op(ArithmeticOp::MUL, arg)
}


#[allow(unused_variables)]
fn builtin_div(env: &mut LEnv, arg: LVal) -> LVal {
    builtin_op(ArithmeticOp::DIV, arg)
}


#[allow(unused_variables)]
fn builtin_mod(env: &mut LEnv, arg: LVal) -> LVal {
    builtin_op(ArithmeticOp::MOD, arg)
}


#[allow(unused_variables)]
fn builtin_min(env: &mut LEnv, arg: LVal) -> LVal {
    builtin_op(ArithmeticOp::MIN, arg)
}


#[allow(unused_variables)]
fn builtin_max(env: &mut LEnv, arg: LVal) -> LVal {
    builtin_op(ArithmeticOp::MAX, arg)
}


// --- Functions: List ----------------------------------------------------------

#[allow(unused_variables)]
fn builtin_head(env: &mut LEnv, arg: LVal) -> LVal {
    let mut args = arg.into_values();

    builtin_assert!("head": args.len() == 1u);
    builtin_assert!("head": args[0u] != {});

    // Take 1st argument
    let mut qexpr = args.remove(0).unwrap().into_values();

    // Take 1st element and return it
    qexpr.remove(0).unwrap()
}


#[allow(unused_variables)]
fn builtin_tail(env: &mut LEnv, arg: LVal) -> LVal {
    let mut args = arg.into_values();

    builtin_assert!("tail": args.len() == 1u);
    builtin_assert!("tail": args[0u] != {});

    // Take 1st argument
    let qexpr = args.remove(0).unwrap();

    // Remove 1st element and return the tail
    LVal::QExpr(qexpr.into_values().into_iter().skip(1).collect())

}


#[allow(unused_variables)]
fn builtin_list(env: &mut LEnv, arg: LVal) -> LVal {
    LVal::QExpr(arg.into_values())
}


#[allow(unused_variables)]
fn builtin_join(env: &mut LEnv, arg: LVal) -> LVal {
    let mut args = arg.into_values();

    builtin_assert!("join": args[*] is LVal::QExpr(_) "q-expression");

    let mut joined = args.remove(0).unwrap();

    for arg in args.into_iter() {
        joined.extend(arg);
    }

    joined
}


#[allow(unused_variables)]
fn builtin_cons(env: &mut LEnv, arg: LVal) -> LVal {
    let mut args = arg.into_values();

    builtin_assert!("cons": args.len() == 2u);
    builtin_assert!("cons": args[1u] is LVal::QExpr(_) "q-expression");

    let mut value = LVal::qexpr();
    value.append(args.remove(0).unwrap());
    value.extend(LVal::QExpr(args));

    value
}

// --- Functions: Environment ---------------------------------------------------

fn builtin_def(env: &mut LEnv, arg: LVal) -> LVal {
    let mut args = arg.into_values();

    builtin_assert!("eval": args.len() >= 1u);
    builtin_assert!("eval": args[0u] is LVal::QExpr(_) "q-expression");
    let symbols = args.remove(0).unwrap().into_values();

    // Ensure all elements of first list are symbols
    for symbol in symbols.iter() {
        if let LVal::Sym(_) = *symbol {}
        else {
            err!("cannot `def`ine non-symbol: {}", symbol)
       }
    }

    // Check that number of symbols and values matches
    if symbols.len() != args.len() {
        err!("`def` called with number of symbols ({}) != number of values ({})",
             symbols.len(), args.len())
    }

    for (symbol, value) in symbols.iter().zip(args.into_iter()) {
        env.put(symbol.clone(), value);
    }

    LVal::sexpr()
}

fn builtin_eval(env: &mut LEnv, arg: LVal) -> LVal {
    let mut args = arg.into_values();

    builtin_assert!("eval": args.len() == 1u);
    builtin_assert!("eval": args[0u] is LVal::QExpr(_) "q-expression");

    // Take 1st argument
    let qexpr = args.remove(0).unwrap();
    eval(env, LVal::SExpr(qexpr.into_values()))
}


// --- Tests --------------------------------------------------------------------

#[cfg(test)]
mod test {
    use lval::LVal;
    use super::{builtin_op, ArithmeticOp};

    #[test]
    fn builtin_op_few_arguments() {
        assert_eq!(
            builtin_op(ArithmeticOp::ADD, LVal::SExpr(vec![
                LVal::num(2.0)
            ])),
            LVal::err("`+` called with too few arguments: expected at least 1, got 0".into_string())
        )
    }

    #[test]
    fn builtin_op_plus() {
        assert_eq!(
            builtin_op(ArithmeticOp::ADD, LVal::SExpr(vec![
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
            builtin_op(ArithmeticOp::SUB, LVal::SExpr(vec![
                LVal::num(2.0),
                LVal::num(3.0)
            ])),
            LVal::num(-1.0)
        )
    }

    #[test]
    fn builtin_op_minus_unary() {
        assert_eq!(
            builtin_op(ArithmeticOp::SUB, LVal::SExpr(vec![
                LVal::num(2.0)
            ])),
            LVal::num(-2.0)
        )
    }

    #[test]
    fn builtin_op_mul() {
        assert_eq!(
            builtin_op(ArithmeticOp::MUL, LVal::SExpr(vec![
                LVal::num(2.0),
                LVal::num(3.0)
            ])),
            LVal::num(6.0)
        )
    }

    #[test]
    fn builtin_op_div() {
        assert_eq!(
            builtin_op(ArithmeticOp::DIV, LVal::SExpr(vec![
                LVal::num(2.0),
                LVal::num(3.0)
            ])),
            LVal::num(2.0 / 3.0)
        )
    }

    #[test]
    fn builtin_op_modulo() {
        assert_eq!(
            builtin_op(ArithmeticOp::MOD, LVal::SExpr(vec![
                LVal::num(15.0),
                LVal::num(12.0)
            ])),
            LVal::num(3.0)
        )
    }

    #[test]
    fn builtin_op_min() {
        assert_eq!(
            builtin_op(ArithmeticOp::MIN, LVal::SExpr(vec![
                LVal::num(2.0),
                LVal::num(3.0)
            ])),
            LVal::num(2.0)
        )
    }

    #[test]
    fn builtin_op_max() {
        assert_eq!(
            builtin_op(ArithmeticOp::MAX, LVal::SExpr(vec![
                LVal::num(2.0),
                LVal::num(3.0)
            ])),
            LVal::num(3.0)
        )
    }
}