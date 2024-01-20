use std::rc::Weak;

use crate::{lookup, extract, register, function, symbol, runtime::{data::{Data, owner::Owner, symbol_table::SymbolTable, function::Function}, symbol::Symbol}};

mod math;
mod io;
mod logic;
mod array;

pub fn _eval(table: &mut SymbolTable) -> Result<Weak<Data>, String> {
    let expr = lookup!(table, "expr");

    match expr.as_ref() {
        Data::String(_expr) => todo!(),
        d => Err(format!("Not possible to parse and evaluate a non String `{d}`"))
    }
}

pub fn load(table: &mut SymbolTable) {
    register!(table, "pi", Data::Float(3.1415926));
    register!(table, "+", function!(math::sum, "a", "b"));
    register!(table, "-", function!(math::sub, "a", "b"));
    register!(table, "*", function!(math::mul, "a", "b"));
    register!(table, "/", function!(math::div, "a", "b"));
    io::load(table);
    logic::load(table);
    array::load(table);
}
