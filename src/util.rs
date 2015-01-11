use term;
use std::fmt;


pub fn print_error(msg: &str) {
    let mut t = term::stdout().unwrap();
    t.fg(term::color::RED).unwrap();
    (write!(t, "Error: ")).unwrap();
    t.reset().unwrap();
    (write!(t, "{}", msg)).unwrap();
}


pub fn stringify_vec<T: fmt::String>(v: &Vec<T>) -> String {
    v.iter()
        .map(|v| format!("{}", v))
        .collect::<Vec<_>>()
        .connect(" ")
}