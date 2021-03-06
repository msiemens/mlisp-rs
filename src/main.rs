#![feature(collections)]
#![feature(core)]
#![feature(fs)]
#![feature(io)]
#![feature(env)]
#![feature(path)]

#![feature(plugin)]
#![feature(unboxed_closures)]

//! Lispy-rs

#[macro_use] extern crate log;

extern crate ansi_term;
extern crate "readline-sys" as readline;

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

    pub fn repl() {
        let mut env = LEnv::new();
        builtin::initialize(&mut env);

        println!("MLisp Version 0.0.0.1");
        println!("Enter 'quit' to exit");
        println!("");

        // The REPL
        loop {
            // Reading
            let mut input = if let Some(i) = readline::readline("> ") { i }
                            else { println!(""); break };

            loop {
                let parens_l = input.as_bytes().iter().filter(|c| **c == ('(' as u8)).count();
                let parens_r = input.as_bytes().iter().filter(|c| **c == (')' as u8)).count();
                let braces_l = input.as_bytes().iter().filter(|c| **c == ('{' as u8)).count();
                let braces_r = input.as_bytes().iter().filter(|c| **c == ('}' as u8)).count();

                if parens_l == parens_r && braces_l == braces_r {
                    break
                }

                let s = if let Some(i) = readline::readline(". ") { i }
                        else { println!(""); break };

                input.push_str(&s);

            }

            readline::add_history(&input);

            if input == "quit" { break }

            // Parsing
            let ast = match Parser::parse(&input, "<input>") {
                Ok(lval) => lval,
                Err(err) => { print_error(&format!("{:?}\n", err)); continue }
            };
            let lval = LVal::from_ast(ast);

            // Evaluating
            let result = eval(&mut env, lval);

            // Printing
            if let LVal::SExpr(ref v) = result {
                if v.len() == 0 {
                    continue
                }
            }

            result.println(&env);
        }

        println!("Exiting...")
    }

    pub fn run_files(args: Vec<String>) {
        for file in args.into_iter().skip(1) {
            let mut env = LEnv::new();
            builtin::initialize(&mut env);

            let result = builtin::env::builtin_load(&mut env, vec![LVal::str(&file)]);
            if let LVal::Err(..) = result {
                result.println(&env);
            }
        }
    }
}

#[cfg(not(test))]
fn main() {
    use std::env;

    let args: Vec<_> = env::args().collect();

    if args.len() >= 2 {
        main::run_files(args)
    } else {
        main::repl()
    }
}