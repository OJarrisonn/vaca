use std::rc::Rc;

use vaca_core::{Symbol, SymbolTable, lookup, register, sym, Value, function, value::function::Function};

pub fn load(table: &mut SymbolTable) {
    register!(table, "map", function!(map, "f", "array"));
    register!(table, "reduce", function!(reduce, "f", "init", "array"));
}

fn map(table: &mut SymbolTable) -> Result<Rc<Value>, String> {
    let f = lookup!(table, "f").unwrap();
    let array = lookup!(table, "array").unwrap();

    let f = match f.as_ref() {
        Value::Function(f) => f,
        _ => return Err(format!("Argument for `f` should be a function not a {f}"))
    };

    let array = array.as_vec();
    let mut res = vec![];

    for item in array.iter() {
        let mapped = f.exec(vec![item.clone()], table)?;
        res.push(mapped);
    }

    Ok(Rc::new(Value::Array(res)))
}

fn reduce(table: &mut SymbolTable) -> Result<Rc<Value>, String> {
    let f = lookup!(table, "f").unwrap();
    let init = lookup!(table, "init").unwrap();
    let array = lookup!(table, "array").unwrap();

    let f = match f.as_ref() {
        Value::Function(f) => f,
        _ => return Err(format!("Argument for `f` should be a function not a {f}"))
    };

    let array = array.as_vec();
    let mut acc = init;

    for item in array.iter() {
        acc = f.exec(vec![acc, item.clone()], table)?;
    }

    Ok(acc)
}