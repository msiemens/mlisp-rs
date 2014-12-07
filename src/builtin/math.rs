use std::fmt;
use lval::LVal;
use lenv::LEnv;


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


pub fn builtin_add(_: &mut LEnv, arg: LVal) -> LVal {
    builtin_op(ArithmeticOp::ADD, arg)
}


pub fn builtin_sub(_: &mut LEnv, arg: LVal) -> LVal {
    builtin_op(ArithmeticOp::SUB, arg)
}


pub fn builtin_mul(_: &mut LEnv, arg: LVal) -> LVal {
    builtin_op(ArithmeticOp::MUL, arg)
}


pub fn builtin_div(_: &mut LEnv, arg: LVal) -> LVal {
    builtin_op(ArithmeticOp::DIV, arg)
}


pub fn builtin_mod(_: &mut LEnv, arg: LVal) -> LVal {
    builtin_op(ArithmeticOp::MOD, arg)
}


pub fn builtin_min(_: &mut LEnv, arg: LVal) -> LVal {
    builtin_op(ArithmeticOp::MIN, arg)
}


pub fn builtin_max(_: &mut LEnv, arg: LVal) -> LVal {
    builtin_op(ArithmeticOp::MAX, arg)
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