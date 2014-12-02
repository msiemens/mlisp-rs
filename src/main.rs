#![feature(macro_rules)]
#![feature(slicing_syntax)]
#![feature(while_let)]
#![feature(if_let)]
#![feature(phase)]
#![feature(globs)]
#![feature(unboxed_closures)]

//! Lispy-rs

#[phase(plugin, link)] extern crate log;
extern crate term;
extern crate libc;

extern crate readline;

// FIXME(#18822): Remove `pub`
pub mod parser;
pub mod lval;
pub mod lenv;
pub mod eval;
pub mod builtin;
pub mod util;


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

        loop {
            let input = if let Some(i) = readline::readline("> ") { i }
                        else { println!(""); break };
            readline::add_history(input[]);

            if input[] == "quit" { break }

            let ast = match Parser::parse(input[], "<input>") {
                Ok(lval) => lval,
                Err(err) => { print_error(format!("{}\n", err)[]); continue }
            };
            let lval = LVal::from_ast(ast);

            println!("{}", eval(&mut env, lval));
        }

        println!("Exiting...")
    }
}

#[cfg(not(test))]
fn main() {
    main::main()
}