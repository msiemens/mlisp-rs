#![macro_escape]

//! LVal: The basic object type

use std::fmt;
use lenv::LEnv;
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


pub type LBuiltin = fn(&mut LEnv, LVal) -> LVal;


/// A basic object
// TODO: Store source location of this LVal?

#[allow(raw_pointer_deriving)]
#[deriving(PartialEq, Clone)]
pub enum LVal {
    Num(f64),
    Err(String),
    Sym(String),
    Builtin(*const ()),
    SExpr(Vec<LVal>),
    QExpr(Vec<LVal>)
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

    /// Create a new function lval
    pub fn func(f: LBuiltin) -> LVal {
        /*unsafe {
            println!("p(): {}", mem::transmute::<_, fn(LVal) -> LVal>(p)(LVal::qexpr()))
        }*/

        LVal::Builtin(f as *const ())
    }

    /// Create a new sepxr lval
    pub fn sexpr() -> LVal {
        LVal::SExpr(vec![])
    }

    /// Create a new sepxr lval
    pub fn qexpr() -> LVal {
        LVal::QExpr(vec![])
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
            },
            Expr::QExpr(exprs) => {
                let mut qexpr = LVal::qexpr();
                for child in exprs.into_iter() {
                    qexpr.append(LVal::from_ast(child));
                }

                qexpr
            }
        }
    }

    // --- Public methods: Conversions ------------------------------------------

    pub fn as_values(&self) -> &Vec<LVal> {
        if let &LVal::SExpr(ref values) = self {
            values
        } else if let &LVal::QExpr(ref values) = self {
            values
        } else {
            panic!("LVal::as_sexpr(self={})", self);
        }
    }

    pub fn into_values(self) -> Vec<LVal> {
        if let LVal::SExpr(values) = self {
            values
        } else if let LVal::QExpr(values) = self {
            values
        } else {
            panic!("LVal::as_sexpr(self={})", self);
        }
    }

    pub fn as_num(&mut self) -> &mut f64 {
        if let &LVal::Num(ref mut float) = self {
            //let Float(ref mut i) = float;
            //return i
            return float
        } else {
            panic!("LVal::as_num(self={})", self)
        }
    }

    pub fn into_num(self) -> f64 {
        if let LVal::Num(float) = self {
            //let Float(ref mut i) = float;
            //return i
            return float
        } else {
            panic!("LVal::into_num(self={})", self)
        }
    }
    pub fn as_sym(&mut self) -> &mut String {
        if let &LVal::Sym(ref mut value) = self {
            return value
        } else {
            panic!("LVal::as_sym(self={})", self)
        }
    }

    // --- Public methods: Other functions --------------------------------------

    /// Delete a lval
    pub fn del(self) {}

    /// Append a lval to a sexpr
    ///
    /// Panics when `self` is not a SExpr
    pub fn append(&mut self, expr: LVal) {
        if let LVal::SExpr(ref mut values) = *self {
            values.push(expr);
        } else if let LVal::QExpr(ref mut values) = *self {
            values.push(expr);
        } else {
            panic!("LVal::append(self={}, expr={})", self, expr);
        }
    }

    pub fn extend(&mut self, container: LVal) {
        if let LVal::SExpr(ref mut values) = *self {
            values.extend(container.into_values().into_iter());
        } else if let LVal::QExpr(ref mut values) = *self {
            values.extend(container.into_values().into_iter());
        } else {
            panic!("LVal::extend(self={}, expr={})", self, container);
        }
    }

    pub fn type_name(&self) -> &'static str {
        match *self {
            LVal::Num(..)     => "number",
            LVal::Err(..)     => "error",
            LVal::Sym(..)     => "symbol",
            LVal::Builtin(..) => "builtin function",
            LVal::SExpr(..)   => "s-expression",
            LVal::QExpr(..)   => "q-expression",
        }
    }
}


impl fmt::Show for LVal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            LVal::Num(i)            => write!(f, "{}", i),
            LVal::Err(ref msg)      => { print_error(msg[]); Ok(()) },
            LVal::Sym(ref symbol)   => write!(f, "{}", symbol),
            // TODO: Find a smart way to print the function name
            LVal::Builtin(..)       => write!(f, "<function>"),
            LVal::SExpr(ref values) => {
                write!(f, "({})", values.iter()
                                        .map(|v| format!("{}", v))
                                        .collect::<Vec<_>>()
                                        .connect(" "))
            },
            LVal::QExpr(ref values) => {
                write!(f, "{{{}}}", values.iter()
                                        .map(|v| format!("{}", v))
                                        .collect::<Vec<_>>()
                                        .connect(" "))
            }
        }
    }
}