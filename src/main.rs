#![feature(macro_rules, slicing_syntax, while_let, if_let, phase, globs)]

extern crate term;
#[phase(plugin, link)] extern crate log;

use lval::LVal;

mod parser;
mod lval;


fn main() {
    println!("MLisp VErsion 0.0.0.1");
    println!("Press Ctrl+c to exit");

    loop {
        print!("mlisp> ");

        let input = std::io::stdin().read_line().unwrap();
        let ast = parser::Parser::new(input[], "<input>").parse();

        let lval = LVal::from_ast(ast);

        lval.println();
    }
}