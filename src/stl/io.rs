use std::rc::Weak;

use crate::{lookup, extract, Owner, SymbolTable, Data, Symbol};

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