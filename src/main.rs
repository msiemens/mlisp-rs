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

    use lval::LVal;
    use parser::Parser;

    pub fn main() {
        println!("MLisp VErsion 0.0.0.1");
        println!("Press Ctrl+c to exit");

        loop {
            let input = if let Some(i) = readline::readline("mlisp> ") { i }
                        else { println!(""); break };
            readline::add_history(input[]);

            let ast = Parser::new(input[], "<input>").parse();
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