use term;


pub fn print_error(msg: &str) {
    let mut t = term::stdout().unwrap();
    t.fg(term::color::RED).unwrap();
    (write!(t, "Error: ")).unwrap();
    t.reset().unwrap();
    (write!(t, "{}", msg)).unwrap();
}