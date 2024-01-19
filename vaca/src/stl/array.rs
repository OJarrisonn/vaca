use std::rc::Weak;

use crate::{lookup, extract, runtime::{data::{Data, owner::Owner, symbol_table::SymbolTable, function::Function}, symbol::Symbol}, register, function, symbol};

pub fn load(owner: &mut Owner, table: &mut SymbolTable) {
    register!(owner, table, "map", function!(map, "f", "array"));
    register!(owner, table, "reduce", function!(reduce, "f", "init", "array"));
}

fn map(owner: &mut Owner, table: &mut SymbolTable) -> Result<Weak<Data>, String> {
    let f = lookup!(table, "f");
    let array = lookup!(table, "array");

    let f = match f.as_ref() {
        Data::Function(f) => f,
        _ => return Err(format!("Argument for `f` should be a function not a {f}"))
    };

    let array = array.as_vec();
    let mut res = vec![];

    for item in array.iter() {
        let mapped = f.exec(vec![item.clone()], owner, table)?;
        let mapped = owner.allocate_return(mapped);
        res.push(mapped);
    }

    Ok(owner.allocate(Data::Array(res)))
}

fn reduce(owner: &mut Owner, table: &mut SymbolTable) -> Result<Weak<Data>, String> {
    let f = lookup!(table, "f");
    let init = table.lookup(&symbol!("init"))?;
    let array = lookup!(table, "array");

    let f = match f.as_ref() {
        Data::Function(f) => f,
        _ => return Err(format!("Argument for `f` should be a function not a {f}"))
    };

    let array = array.as_vec();
    let mut acc = init;

    for item in array.iter() {
        acc = f.exec(vec![acc, item.clone()], owner, table)?;
    }

    Ok(owner.relocate(extract!(acc)))
}