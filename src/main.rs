#![feature(macro_rules)]
#![feature(slicing_syntax)]
#![feature(phase)]
#![feature(globs)]
#![feature(unboxed_closures)]

//! Lispy-rs

#[phase(plugin, link)] extern crate log;
extern crate term;
extern crate libc;

extern crate readline;

mod parser;
mod lval;
mod lenv;
mod eval;
mod builtin;
mod util;


#[cfg(not(test))]
mod main {
    use readline;

    use util::print_error;
    use eval::eval;
    use lval::LVal;
    use lenv::LEnv;
    use parser::Parser;
    use builtin;

    pub fn main() {
        let mut env = LEnv::new();
        builtin::initialize(&mut env);

        println!("MLisp Version 0.0.0.1");
        println!("Enter 'quit' to exit");
        println!("");

        // The REPL
        loop {
            // Reading
            let input = if let Some(i) = readline::readline("> ") { i }
                        else { println!(""); break };
            readline::add_history(input[]);

            if input == "quit" { break }

            // Parsing
            let ast = match Parser::parse(input[], "<input>") {
                Ok(lval) => lval,
                Err(err) => { print_error(format!("{}\n", err)[]); continue }
            };
            let lval = LVal::from_ast(ast);

            // Evaluating
            let result = eval(&mut env, lval);

            // Printing
            result.println(&env);
        }

        println!("Exiting...")
    }
}

#[cfg(not(test))]
fn main() {
    main::main()
}