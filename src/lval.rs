use std::fmt;
use parser::ast::{Expr, ExprNode};


pub enum LVal {
    Num(i64),
    Err(String),
    Sym(String),
    SExpr(Vec<Box<LVal>>)
}

impl LVal {
    pub fn num(value: i64) -> LVal {
        LVal::Num(value)
    }

    pub fn err(msg: &str) -> LVal {
        LVal::Err(msg.into_string())
    }

    pub fn sym(symbol: &str) -> LVal {
        LVal::Sym(symbol.into_string())
    }

    pub fn sexpr() -> LVal {
        LVal::SExpr(vec![])
    }

    pub fn del(self) {}


    pub fn from_ast(node: ExprNode) -> LVal {
        match node.value {
            Expr::Number(i) => LVal::num(i),
            Expr::Symbol(s) => LVal::sym(s[]),
            Expr::SExpr(exprs) => {
                let mut sexpr = LVal::sexpr();
                for child in exprs.into_iter() {
                    sexpr.add(LVal::from_ast(child));
                }

                sexpr
            }
        }
    }


    pub fn add(&mut self, expr: LVal) {
        if let LVal::SExpr(ref mut values) = *self {
            values.push(box expr);
        }
    }

    pub fn print(&self) {
        print!("{}", self);
    }

    pub fn println(&self) {
        print!("{}\n", self);
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