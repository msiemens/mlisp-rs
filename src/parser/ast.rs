//! Abstract syntax tree

use std::fmt;
use parser::tokens::SourceLocation;
use parser::util::SharedString;
use util::stringify_vec;


// --- AST Node: Expression -----------------------------------------------------

/// An expression AST node
#[deriving(PartialEq, Clone)]
pub struct ExprNode {
    /// The expression's value
    pub value: Expr,

    /// The location of the definition in the input
    pub location: SourceLocation
}

impl ExprNode {
    pub fn new(expr: Expr, location: SourceLocation) -> ExprNode {
        ExprNode {
            value: expr,
            location: location
        }
    }
}

impl fmt::Show for ExprNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

// --- AST Node: Expression: Values ---------------------------------------------

#[deriving(PartialEq, Clone)]
pub enum Expr {
    /// A SExpr
    SExpr(Vec<ExprNode>),

    /// A QExpr
    QExpr(Vec<ExprNode>),

    /// A string
    String(SharedString),

    /// A symbol
    Symbol(SharedString),

    /// A number
    Number(f64)
}

impl fmt::Show for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Expr::Number(i)          => write!(f, "{}", i),
            Expr::String(ref string) => write!(f, "{}", string),
            Expr::Symbol(ref token)  => write!(f, "{}", token),
            Expr::SExpr(ref values)  => {
                write!(f, "({})", stringify_vec(values))
            },
            Expr::QExpr(ref values)  => {
                write!(f, "{{{}}}", stringify_vec(values))
            }
        }
    }
}