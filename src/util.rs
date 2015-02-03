use std::fmt;

use ansi_term::Colour::Red;


pub fn print_error(msg: &str) {
    print!("{} {}", Red.paint("Error:"), msg);
}


pub fn stringify_vec<T: fmt::Debug>(v: &Vec<T>) -> String {
    v.iter()
        .map(|v| format!("{:?}", v))
        .collect::<Vec<_>>()
        .connect(" ")
}