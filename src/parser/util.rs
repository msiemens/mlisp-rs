//! Miscellaneous utilities

use std::borrow::ToOwned;
use std::rc::Rc;

// --- Shared string ------------------------------------------------------------

/// A string wrapped in a Rc
pub type SharedString = Rc<String>;

/// Create a shared string from a `&str`
pub fn rcstr<'a>(s: &'a str) -> SharedString {
    Rc::new(s.to_owned())
}

/// Create a shared string from a `String`
pub fn rcstring(s: String) -> SharedString {
    Rc::new(s)
}
