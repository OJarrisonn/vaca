use std::rc::Weak;

use crate::{Owner, SymbolTable, Data, Symbol, lookup, extract};

pub mod math;

pub fn _eval(_owner: &mut Owner, table: &mut SymbolTable) -> Result<Weak<Data>, String> {
    let expr = lookup!(table, "expr");

    match expr.as_ref() {
        Data::String(_expr) => todo!(),
        d => Err(format!("Not possible to parse and evaluate a non String `{d}`"))
    }
}