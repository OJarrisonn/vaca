use std::rc::Weak;

use crate::{lookup, extract, runtime::{data::{owner::Owner, symbol_table::SymbolTable, Data}, symbol::Symbol}};

pub fn print(owner: &mut Owner, table: &mut SymbolTable) -> Result<Weak<Data>, String> {
    let text = lookup!(table, "text");

    match text.as_ref() {
        Data::Array(list) => list.iter()
            .map(|t| extract!(t))
            .for_each(|t| print!("{}", t.as_ref())),
        d => print!("{}", d)
    };

    Ok(owner.insert(Data::Nil))
}