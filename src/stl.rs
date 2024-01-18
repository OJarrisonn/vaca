use std::rc::Weak;

use crate::{lookup, extract, register, function, symbol, runtime::{data::{Data, owner::Owner, symbol_table::SymbolTable, function::Function}, symbol::Symbol}};

mod math;
mod io;
mod logic;
mod array;

pub fn _eval(_owner: &mut Owner, table: &mut SymbolTable) -> Result<Weak<Data>, String> {
    let expr = lookup!(table, "expr");

    match expr.as_ref() {
        Data::String(_expr) => todo!(),
        d => Err(format!("Not possible to parse and evaluate a non String `{d}`"))
    }
}

pub fn load(owner: &mut Owner, table: &mut SymbolTable) {
    register!(owner, table, "pi", Data::Float(3.1415926));
    register!(owner, table, "+", function!(math::sum, "a", "b"));
    register!(owner, table, "-", function!(math::sub, "a", "b"));
    register!(owner, table, "*", function!(math::mul, "a", "b"));
    register!(owner, table, "/", function!(math::div, "a", "b"));
    io::load(owner, table);
    logic::load(owner, table);
    array::load(owner, table);
}
