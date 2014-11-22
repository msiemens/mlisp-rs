use lval::LVal;


pub fn builtin_op(op: String, args: Vec<LVal>) -> LVal {
    let mut args = args;

    // Make sure all arguments are numbers
    for arg in args.iter() {
        match *arg {
            LVal::Num(_) => {},
            _ => err!("cannot operate on non-number: `{}`", arg)
        }
    }

    if args.len() == 0 { err!("too few arguments: `{}`", args) }
    let mut x = args.remove(0).unwrap().get_num();

    // Perform unary minus operation
    if op[] == "-" && args.len() == 0 {
        return LVal::num(-1.0 * x)
    }

    if args.len() == 0 { err!("too few arguments: `{}`", args) }

    for arg in args.into_iter() {
        let y = arg.get_num();

        x = match op[] {
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
            _ => return err!("invalid operator: {}", op)
        };
    }

    LVal::num(x)
}


#[cfg(test)]
mod test {
    use lval::LVal;
    use super::builtin_op;

    #[test]
    fn builtin_op_few_arguments() {
        assert_eq!(
            builtin_op("+".into_string(), vec![
                LVal::num(2.0)
            ]),
            LVal::err("too few arguments: `[]`".into_string())
        )
    }

    #[test]
    fn builtin_op_plus() {
        assert_eq!(
            builtin_op("+".into_string(), vec![
                LVal::num(2.0),
                LVal::num(3.0),
                LVal::num(4.0),
                LVal::num(5.0)
            ]),
            LVal::num(14.0)
        )
    }

    #[test]
    fn builtin_op_minus() {
        assert_eq!(
            builtin_op("-".into_string(), vec![
                LVal::num(2.0),
                LVal::num(3.0)
            ]),
            LVal::num(-1.0)
        )
    }

    #[test]
    fn builtin_op_minus_unary() {
        assert_eq!(
            builtin_op("-".into_string(), vec![
                LVal::num(2.0)
            ]),
            LVal::num(-2.0)
        )
    }

    #[test]
    fn builtin_op_mul() {
        assert_eq!(
            builtin_op("*".into_string(), vec![
                LVal::num(2.0),
                LVal::num(3.0)
            ]),
            LVal::num(6.0)
        )
    }

    #[test]
    fn builtin_op_div() {
        assert_eq!(
            builtin_op("/".into_string(), vec![
                LVal::num(2.0),
                LVal::num(3.0)
            ]),
            LVal::num(2.0 / 3.0)
        )
    }

    #[test]
    fn builtin_op_modulo() {
        assert_eq!(
            builtin_op("%".into_string(), vec![
                LVal::num(15.0),
                LVal::num(12.0)
            ]),
            LVal::num(3.0)
        )
    }

    #[test]
    fn builtin_op_min() {
        assert_eq!(
            builtin_op("min".into_string(), vec![
                LVal::num(2.0),
                LVal::num(3.0)
            ]),
            LVal::num(2.0)
        )
    }

    #[test]
    fn builtin_op_max() {
        assert_eq!(
            builtin_op("max".into_string(), vec![
                LVal::num(2.0),
                LVal::num(3.0)
            ]),
            LVal::num(3.0)
        )
    }
}