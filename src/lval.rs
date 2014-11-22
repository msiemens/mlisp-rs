#![macro_escape]

//! LVal: The basic object type

use std::fmt;
use parser::ast::{Expr, ExprNode};
use util::print_error;


/// Return an error
macro_rules! err(
    ($msg:expr) => (
        return LVal::err($msg.into_string())
    );

    ($msg:expr, $( $args:expr ),* ) => (
        return LVal::err(format!($msg, $($args),* ))
    );
)


/// A basic object
pub enum LVal {
    Num(f64),
    Err(String),
    Sym(String),
    SExpr(Vec<LVal>)
}

impl LVal {

    // --- Constructors ---------------------------------------------------------

    /// Create a new number lval
    pub fn num(value: f64) -> LVal {
        LVal::Num(value)
    }

    /// Create a new error lval
    pub fn err(msg: String) -> LVal {
        LVal::Err(msg)
    }

    /// Create a new symbol lval
    pub fn sym(symbol: &str) -> LVal {
        LVal::Sym(symbol.into_string())
    }

    /// Create a new sepxr lval
    pub fn sexpr() -> LVal {
        LVal::SExpr(vec![])
    }

    /// Construct a lval from a given AST
    pub fn from_ast(ast: ExprNode) -> LVal {
        match ast.value {
            Expr::Number(i) => LVal::num(i as f64),
            Expr::Symbol(s) => LVal::sym(s[]),
            Expr::SExpr(exprs) => {
                let mut sexpr = LVal::sexpr();
                for child in exprs.into_iter() {
                    sexpr.append(LVal::from_ast(child));
                }

                sexpr
            }
        }
    }

    // --- Public methods -------------------------------------------------------

    /// Delete a lval
    pub fn del(self) {}

    pub fn get_num(&self) -> f64 {
        if let LVal::Num(i) = *self {
            return i
        } else {
            panic!("Cannot get number of non-number: {}", self)
        }
    }

    /// Append a lval to a sexpr
    ///
    /// Panics when `self` is not a SExpr
    pub fn append(&mut self, expr: LVal) {
        if let LVal::SExpr(ref mut values) = *self {
            values.push(expr);
        } else {
            panic!("cannot extend a non-sexpr")
        }
    }
}


impl fmt::Show for LVal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            LVal::Num(i)          => write!(f, "{}", i),
            LVal::Err(ref msg)    => { print_error(msg[]); Ok(()) },
            LVal::Sym(ref symbol) => write!(f, "{}", symbol),
            LVal::SExpr(ref values) => {
                write!(f, "({})", values.iter()
                                        .map(|v| format!("{}", v))
                                        .collect::<Vec<_>>()
                                        .connect(" "))
            }
        }
    }
}