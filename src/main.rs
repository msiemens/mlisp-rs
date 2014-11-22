#![feature(macro_rules, slicing_syntax, while_let, if_let, phase, globs)]

//! Lispy-rs

#[phase(plugin, link)] extern crate log;
extern crate term;

extern crate readline;

// FIXME(#18822): Remove `pub`
pub mod parser;
pub mod lval;


#[cfg(not(test))]
mod main {
    use readline;
    use term;

    use lval::LVal;
    use parser::Parser;

    pub fn main() {
        println!("MLisp VErsion 0.0.0.1");
        println!("Press Ctrl+c to exit");

        loop {
            let input = if let Some(i) = readline::readline("mlisp> ") { i }
                        else { println!(""); break };
            readline::add_history(input[]);

            let ast = match Parser::parse(input[], "<input>") {
                Ok(lval) => lval,
                Err(err) => {
                    let mut t = term::stdout().unwrap();
                    t.fg(term::color::RED).unwrap();
                    (write!(t, "Error: ")).unwrap();
                    t.reset().unwrap();
                    (write!(t, "{}\n", err)).unwrap();

                    continue
                }
            };
            let lval = LVal::from_ast(ast);

            println!("{}", lval);
        }

        println!("Exiting...")
    }
}

#[cfg(not(test))]
fn main() {
    main::main()
}