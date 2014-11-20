use std::fmt;
use parser::tokens::SourceLocation;
use parser::tokens::Token;
use parser::util::SharedString;


macro_rules! define(
    ( $name:ident -> $wrapper:ident : $( $variants:ident ( $( $arg:ty ),* ) ),* ) => {
        #[deriving(PartialEq, Eq, Clone)]
        pub struct $wrapper {
            pub value: $name,
            pub location: SourceLocation
        }

        impl $wrapper {
            pub fn new(stmt: $name, location: SourceLocation) -> $wrapper {
                $wrapper {
                    value: stmt,
                    location: location
                }
            }
        }

        impl fmt::Show for $wrapper {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{}", self.value)
            }
        }

        #[deriving(PartialEq, Eq, Clone)]
        pub enum $name {
            $( $variants ( $( $arg ),* ) ),*
        }
    };
)


define!(
Expr -> ExprNode:
    SExpr(Vec<ExprNode>),
    Symbol(SharedString),
    Number(i64)
)


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
            }
        }
    }
}