//! LVal: The basic object type

use std::fmt;
use parser::ast::{Expr, ExprNode};


/// A basic object
pub enum LVal {
    Num(i64),
    Err(String),
    Sym(String),
    SExpr(Vec<Box<LVal>>)
}

impl LVal {

    // --- Constructors ---------------------------------------------------------

    /// Create a new number lval
    pub fn num(value: i64) -> LVal {
        LVal::Num(value)
    }

    /// Create a new error lval
    pub fn err(msg: &str) -> LVal {
        LVal::Err(msg.into_string())
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
            Expr::Number(i) => LVal::num(i),
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


    /// Append a lval to a sexpr
    ///
    /// Panics when `self` is not a SExpr
    pub fn append(&mut self, expr: LVal) {
        if let LVal::SExpr(ref mut values) = *self {
            values.push(box expr);
        } else {
            assert!(false, "cannot extend a non-sexpr")
        }
    }
}


impl fmt::Show for LVal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            LVal::Num(i)          => write!(f, "{}", i),
            LVal::Err(ref msg)    => write!(f, "Error: {}", msg),
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