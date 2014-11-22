#![feature(macro_rules, slicing_syntax, while_let, if_let, phase, globs)]

//! Lispy-rs

#[phase(plugin, link)] extern crate log;
extern crate term;

extern crate readline;

// FIXME(#18822): Remove `pub`
pub mod parser;
pub mod lval;
pub mod eval;


#[cfg(not(test))]
mod main {
    use readline;
    use term;

    use eval::eval;
    use lval::LVal;
    use parser::{Parser, ParserError};

    fn print_error(err: ParserError) {
        let mut t = term::stdout().unwrap();
        t.fg(term::color::RED).unwrap();
        (write!(t, "Error: ")).unwrap();
        t.reset().unwrap();
        (write!(t, "{}\n", err)).unwrap();
    }

    pub fn main() {
        println!("MLisp Version 0.0.0.1");
        println!("Enter 'quit' to exit");
        println!("");

        loop {
            let input = if let Some(i) = readline::readline("> ") { i }
                        else { println!(""); break };
            readline::add_history(input[]);

            if input[] == "quit" { break }

            let ast = match Parser::parse(input[], "<input>") {
                Ok(lval) => lval,
                Err(err) => { print_error(err); continue }
            };
            let lval = LVal::from_ast(ast);

            println!("{}", eval(lval));
        }

        println!("Exiting...")
    }
}

#[cfg(not(test))]
fn main() {
    main::main()
}