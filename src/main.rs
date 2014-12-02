#![feature(macro_rules, slicing_syntax, while_let, if_let, phase, globs)]

//! Lispy-rs

#[phase(plugin, link)] extern crate log;
extern crate term;

//extern crate readline;

// FIXME(#18822): Remove `pub`
pub mod parser;
pub mod lval;
pub mod eval;
pub mod builtin;
pub mod util;


extern {
    pub fn getch() -> i32;
    pub fn kbhit() -> i32;
}

#[cfg(not(test))]
mod main {
    //use readline;
    use std;

    use util::print_error;
    use eval::eval;
    use lval::LVal;
    use parser::Parser;

    pub fn main() {
        println!("MLisp Version 0.0.0.1");
        println!("Enter 'exit' to exit");
        println!("");

        let mut stdin = std::io::stdin();

        loop {
            print!("> ");

            // TODO: Use getch instead?
            let input = match stdin.read_line() {
                Ok(s) => s,
                Err(e) => {
                    println!("Error reading stdin: {}", e);
                    break
                }
            };

            //let input = if let Some(i) = readline::readline("> ") { i }
            //            else { println!(""); break };
            //readline::add_history(&*input);

            if &*input.trim() == "exit" { break }

            let ast = match Parser::parse(&*input, "<input>") {
                Ok(lval) => lval,
                Err(err) => { print_error(&*format!("{}\n", err)); continue }
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