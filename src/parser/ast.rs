//! Abstract syntax tree

use std::fmt;
use parser::tokens::SourceLocation;
use parser::util::SharedString;


// --- AST Node: Expression -----------------------------------------------------

/// An expression AST node
#[deriving(PartialEq, Eq, Clone)]
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

#[deriving(PartialEq, Eq, Clone)]
pub enum Expr {
    /// A SExpr
    SExpr(Vec<ExprNode>),

    /// A QExpr
    QExpr(Vec<ExprNode>),

    /// A symbol
    Symbol(SharedString),

    /// A number
    Number(i64)
}

impl fmt::Show for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Expr::Number(i)         => write!(f, "{}", i),
            Expr::Symbol(ref token) => write!(f, "{}", token),
            Expr::SExpr(ref values) => {
                write!(f, "({})", values.iter()
                                        .map(|v| format!("{}", v))
                                        .collect::<Vec<_>>()
                                        .connect(" "))
            },
            Expr::QExpr(ref values) => {
                write!(f, "{{{}}}", values.iter()
                                        .map(|v| format!("{}", v))
                                        .collect::<Vec<_>>()
                                        .connect(" "))
            }
        }
    }
}