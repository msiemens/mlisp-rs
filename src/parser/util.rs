#![macro_escape]

//! Miscellaneous utilities

use std;
use std::rc::Rc;
use term;
use parser::tokens::SourceLocation;

// --- Shared string ------------------------------------------------------------

/// A string wrapped in a Rc
pub type SharedString = Rc<String>;

/// Create a shared string from a `&str`
pub fn rcstr<'a>(s: &'a str) -> SharedString {
    Rc::new(s.into_string())
}

/// Create a shared string from a `String`
pub fn rcstring(s: String) -> SharedString {
    Rc::new(s)
}


// --- Error handling -----------------------------------------------------------

/// Abort with a fatal error for a given AST node
macro_rules! fatal(
    ($msg:expr, $($args:expr),* @ $stmt:expr) => {
        fatal(format!($msg, $($args),*), &$stmt.location)
    };

    ($msg:expr @ $stmt:expr) => {
        fatal($msg.into_string(), &$stmt.location)
    };
)

/// Abort execution with a fatal error
pub fn fatal(msg: String, source: &SourceLocation) -> ! {
    let mut t = term::stdout().unwrap();

    t.fg(term::color::RED).unwrap();
    (write!(t, "Error ")).unwrap();

    t.reset().unwrap();
    (write!(t, "in {}: ", source)).unwrap();
    (write!(t, "{}\n", msg)).unwrap();

    t.reset().unwrap();

    std::io::stdio::set_stderr(box std::io::util::NullWriter);
    panic!();
}


/// Print a warning for a given AST node
macro_rules! warn(
    ($msg:expr, $($args:expr),* @ $stmt:expr ) => {
        warn(format!($msg, $($args),*), &$stmt.location)
    }
)

/// Print a warning
pub fn warn(msg: String, source: &SourceLocation) {
    let mut t = term::stdout().unwrap();

    t.fg(term::color::YELLOW).unwrap();
    (write!(t, "Warning ")).unwrap();

    t.reset().unwrap();
    (write!(t, "in {}: ", source)).unwrap();
    (write!(t, "{}\n", msg)).unwrap();

    t.reset().unwrap();
}