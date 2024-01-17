mod stl;
mod parser;
mod runtime;

#[macro_use]
mod macros;

use std::io::Write;

use crate::parser::parse;
use crate::runtime::{data::{Data, owner::Owner, symbol_table::SymbolTable}, expr::{Expr, Literal}, symbol::Symbol};



fn main() {
    let mut owner = Owner::new();
    let mut table = SymbolTable::new();

    owner.create_scope();
    table.create_scope();

    stl::load(&mut owner, &mut table);

    

    table.drop_scope();
    owner.drop_scope();
}

fn repl(owner: &mut Owner, table: &mut SymbolTable) {
    loop {
        let mut input = String::new();

        print!(">> ");
        let _ = std::io::stdout().flush();
        let _ = std::io::stdin().read_line(&mut input);

        if input.trim() == "." { break; }

        let program = parse(input).unwrap();
        let res = program.eval(owner, table);

        dbg!(res.unwrap().upgrade());
        println!("");
    }
}

fn compiler(owner: &mut Owner, table: &mut SymbolTable, filename: &str) {

}

fn runner(owner: &mut Owner, table: &mut SymbolTable, filename: &str) {

}