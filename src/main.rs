#![feature(macro_rules, slicing_syntax, while_let, if_let, phase, globs)]

//! Lispy-rs

extern crate term;
#[phase(plugin, link)] extern crate log;

use lval::LVal;

// FIXME(#18822): Remove `pub`
pub mod parser;
pub mod lval;


fn main() {
    println!("MLisp VErsion 0.0.0.1");
    println!("Press Ctrl+c to exit");

    loop {
        print!("mlisp> ");

        let input = std::io::stdin().read_line().unwrap();
        if input.as_bytes()[0] == 4 {
            break
        }

        let ast = parser::Parser::new(input[], "<input>").parse();
        let lval = LVal::from_ast(ast);

        println!("{}", lval);
    }

    println!("Exiting...")
}