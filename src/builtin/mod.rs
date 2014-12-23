use lval::LVal;
use lenv::LEnv;
use builtin::conditions::*;
use builtin::env::*;
use builtin::list::*;
use builtin::math::*;


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
    ($func:expr: ASSERT TYPE: $element:expr, $pos:expr, $typ:ident) => {
        if !lval_is!($element, $typ) {
            err!("`{}` called with wrong type for argument {}: expected {}, got {}: `{}`",
                //$func, $pos + 1, $typ_name, $element.type_name(), $element)
                $func, $pos + 1, lval_type_name!($typ), $element.type_name(), $element)
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

    ($func:expr: $args:ident [*] is $typ:ident) => {
        for (i, arg) in $args.iter().enumerate() {
            builtin_assert!($func: ASSERT TYPE: *arg, i, $typ);
        }
    };

    ($func:expr: $args:ident [ $i:expr ] != {}) => {
        {
            builtin_assert!($func: ASSERT TYPE: $args[$i], $i, qexpr);
            if $args[$i].as_values().len() == 0 {
                err!("`{}` called with empty q-expr", $func)
            }
        }
    };

    ($func:expr: $args:ident [ $i:expr ] is $typ:ident) => {
        builtin_assert!($func: ASSERT TYPE: $args[$i], $i, $typ);
    };

);


mod conditions;
pub mod env;
mod list;
mod math;


pub fn initialize(env: &mut LEnv) {
    // Environment
    env.put(LVal::sym("\\"),    LVal::func(builtin_lambda));
    env.put(LVal::sym("def"),   LVal::func(builtin_def));
    env.put(LVal::sym("="),     LVal::func(builtin_put));
    env.put(LVal::sym("eval"),  LVal::func(builtin_eval));
    env.put(LVal::sym("load"),  LVal::func(builtin_load));
    env.put(LVal::sym("error"), LVal::func(builtin_error));
    env.put(LVal::sym("println"), LVal::func(builtin_println));

    // Conditions
    env.put(LVal::sym("<"),     LVal::func(builtin_lt));
    env.put(LVal::sym("<="),    LVal::func(builtin_le));
    env.put(LVal::sym(">="),    LVal::func(builtin_ge));
    env.put(LVal::sym(">"),     LVal::func(builtin_gt));
    env.put(LVal::sym("=="),    LVal::func(builtin_eq));
    env.put(LVal::sym("!="),    LVal::func(builtin_neq));
    env.put(LVal::sym("if"),    LVal::func(builtin_if));
    env.put(LVal::sym("or"),    LVal::func(builtin_or));
    env.put(LVal::sym("and"),   LVal::func(builtin_and));
    env.put(LVal::sym("not"),   LVal::func(builtin_not));

    // Lists
    env.put(LVal::sym("head"),  LVal::func(builtin_head));
    env.put(LVal::sym("tail"),  LVal::func(builtin_tail));
    env.put(LVal::sym("list"),  LVal::func(builtin_list));
    env.put(LVal::sym("join"),  LVal::func(builtin_join));
    env.put(LVal::sym("cons"),  LVal::func(builtin_cons));

    // Math
    env.put(LVal::sym("+"),     LVal::func(builtin_add));
    env.put(LVal::sym("-"),     LVal::func(builtin_sub));
    env.put(LVal::sym("*"),     LVal::func(builtin_mul));
    env.put(LVal::sym("/"),     LVal::func(builtin_div));
    env.put(LVal::sym("%"),     LVal::func(builtin_mod));
    env.put(LVal::sym("min"),   LVal::func(builtin_min));
    env.put(LVal::sym("max"),   LVal::func(builtin_max));
}
