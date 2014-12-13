#![macro_escape]

//! LVal: The basic object type

use std::mem;
use std::fmt;
use lenv::LEnv;
use parser::ast::{Expr, ExprNode};
use util::{print_error, stringify_vec};


/// Return an error
macro_rules! err(
    ($msg:expr) => (
        return LVal::err($msg.into_string())
    );

    ($msg:expr, $( $args:expr ),* ) => (
        return LVal::err(format!($msg, $($args),* ))
    );
)


macro_rules! lval_is(
    ($el:expr, number) => ( if let LVal::Num(..)   = $el { true } else { false } );
    ($el:expr, qexpr)  => ( if let LVal::QExpr(..) = $el { true } else { false } );
    ($el:expr, sexpr)  => ( if let LVal::SExpr(..) = $el { true } else { false } );
)

macro_rules! lval_type_name(
    (number)   => ("a number");
    (err)      => ("an error");
    (sym)      => ("a symbol");
    (function) => ("a lambda");
    (builtin)  => ("a builtin function");
    (sexpr)    => ("a s-expression");
    (qexpr)    => ("a q-expression");
)


/// A builtin function
///
/// Used to implement PartialEq for the function pointer
#[deriving(Clone)]
pub struct LBuiltin(pub fn(& mut LEnv, Vec<LVal>) -> LVal);

impl PartialEq for LBuiltin {
    fn eq(&self, other: &LBuiltin) -> bool {
        unsafe {
            let ptr_self:  *const () = mem::transmute(self);
            let ptr_other: *const () = mem::transmute(other);
            ptr_self == ptr_other
        }
    }
}


/// A basic object
#[deriving(PartialEq, Clone)]
pub enum LVal {
    Num(f64),
    Err(String),
    Sym(String),  // TODO: Use SharedString?
    //Str(String),
    Function {
        env: LEnv,
        formals: Vec<LVal>,  // List of formal argument symbols
        body:    Vec<LVal>   // Actually a S-Expr
    },
    Builtin(LBuiltin),
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

    // Create a new lambda lval
    pub fn lambda(formals: LVal, body: LVal) -> LVal {
        LVal::Function {
            env:     LEnv::new(),
            formals: formals.into_values(),
            body:    body.into_values()
        }
    }

    /// Create a new function lval
    pub fn func(f: fn(&mut LEnv, Vec<LVal>) -> LVal) -> LVal {
        LVal::Builtin(LBuiltin(f))
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
            panic!("LVal::as_values(self={})", self);
        }
    }

    pub fn into_values(self) -> Vec<LVal> {
        if let LVal::SExpr(values) = self {
            values
        } else if let LVal::QExpr(values) = self {
            values
        } else {
            panic!("LVal::into_values(self={})", self);
        }
    }

    pub fn as_num(&self) -> &f64 {
        if let &LVal::Num(ref float) = self {
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
    pub fn as_sym(&self) -> &String {
        if let &LVal::Sym(ref value) = self {
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

    /// Append all values from a sexpr to a sexpr
    ///
    /// Panics when `self` or `container` is not a SExpr
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
            LVal::Num(..)      => "a number",
            LVal::Err(..)      => "an error",
            LVal::Sym(..)      => "a symbol",
            //LVal::Str(..)      => "a string",
            LVal::Function{..} => "a lambda",
            LVal::Builtin(..)  => "a builtin function",
            LVal::SExpr(..)    => "a s-expression",
            LVal::QExpr(..)    => "a q-expression"
        }
    }

    pub fn to_string(&self, env: &LEnv) -> String {
        match *self {
            LVal::Sym(ref name) => {
                match env.get(name[]) {
                    LVal::Err(..) => name.clone(),
                    value         => value.to_string(env)
                }
            },
            LVal::SExpr(ref values) => {
                format!(
                    "({})",
                    values.iter()
                        .map(|v| v.to_string(env))
                        .collect::<Vec<_>>()
                        .connect(" ")
                )
            }
            LVal::Function { ref env, ref formals, ref body } => {
                format!(
                    "\\ {{{}}} {{{}}}",
                    stringify_vec(formals),
                    body.iter()
                        .map(|v| v.to_string(env))
                        .collect::<Vec<_>>()
                        .connect(" ")
                )
            },
            LVal::Builtin(..) => match env.look_up(self) {
                Some(name) => format!("<builtin: '{}'>", name),
                None => format!("{}", self)
            },
            _ => { format!("{}", self) }
        }
    }

    pub fn print(&self, env: &LEnv) {
        if let LVal::Err(ref msg) = *self {
            print_error(msg[]);
        } else {
            print!("{}", self.to_string(env));
        }
    }

    pub fn println(&self, env: &LEnv) {
        self.print(env);
        println!("");
    }
}


// Used for debugging and when the environment is not available
impl fmt::Show for LVal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            LVal::Num(i)            => write!(f, "{}", i),
            LVal::Err(ref msg)      => write!(f, "{}", msg),
            LVal::Sym(ref symbol)   => write!(f, "{}", symbol),
            LVal::Function{ env: _, ref formals, ref body } => {
                write!(f, "\\ {{{}}} {{{}}}", stringify_vec(formals),
                                              stringify_vec(body))
            },
            LVal::Builtin(..)       => write!(f, "<function>"),
            LVal::SExpr(ref values) => {
                write!(f, "({})", stringify_vec(values))
            },
            LVal::QExpr(ref values) => {
                write!(f, "{{{}}}", stringify_vec(values))
            }
        }
    }
}