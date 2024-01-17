mod stl;
mod parser;
mod runtime;

#[macro_use]
mod macros;

use std::io::Write;

use crate::parser::parse;
use crate::runtime::{data::{Data, owner::Owner, function::Function, symbol_table::SymbolTable}, expr::{Expr, Literal}, symbol::Symbol};
//use crate::runtime::data::{owner::Owner, symbol_table::SymbolTable, function::Function, symbol::Symbol, Data};

fn main() {
    let mut owner = Owner::new();
    let mut table = SymbolTable::new();

    owner.create_scope();
    table.create_scope();

    register!(owner, table, "pi", Data::Float(3.1415926));
    register!(owner, table, "+", function!(stl::math::sum, "a", "b"));
    register!(owner, table, "-", function!(stl::math::sub, "a", "b"));
    register!(owner, table, "*", function!(stl::math::mul, "a", "b"));
    register!(owner, table, "/", function!(stl::math::div, "a", "b"));
    register!(owner, table, "print", function!(stl::io::print, "text"));

    loop {
        let mut input = String::new();

        print!(">> ");
        let _ = std::io::stdout().flush();
        let _ = std::io::stdin().read_line(&mut input);

        if input.trim() == "." { break; }

        let program = parse(input).unwrap();
        let res = program.eval(&mut owner, &mut table);

        dbg!(res.unwrap().upgrade());
        println!("");

    }



    table.drop_scope();
    owner.drop_scope();
}