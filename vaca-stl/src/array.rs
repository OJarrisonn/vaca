use std::collections::LinkedList;

use vaca_core::{Symbol, SymbolTable, lookup, register, sym, Value, function, value::function::Function, ValueRef};

pub fn load(table: &mut SymbolTable) {
    register!(table, "nth", function!(nth, "index", "array"));
    register!(table, "map", function!(map, "f", "array"));
    register!(table, "reduce", function!(reduce, "f", "init", "array"));
    register!(table, "scan", function!(scan, "f", "init", "array"));
    register!(table, "append", function!(append, "item", "array"));
    register!(table, "prepend", function!(prepend, "item", "array"));
    register!(table, "concat", function!(concat, "init", "end"));
}

fn nth(table: &mut SymbolTable) -> Result<ValueRef, String> {
    let index = lookup!(table, "index").unwrap();
    let array = unsafe { lookup!(table, "array").unwrap().as_ref() }.unwrap().as_vec();

    let mut index = *match unsafe { index.as_ref() }.unwrap() {
        Value::Integer(i) => i,
        i => return Err(format!("Argument for `index` must be an integer not {i}"))
    };

    if array.len() == 0 {
        Ok(ValueRef::Owned(Value::Nil))
    } else {
        let index = if index >= 0 {
            index as usize % array.len()
        } else {
            while index < 0 {
                index += array.len() as i64
            }
            index as usize
        };

        Ok(ValueRef::Owned(array.into_iter().nth(index).unwrap().unwrap()))
    }
}

fn prepend(table: &mut SymbolTable) -> Result<ValueRef, String> {
    let item = lookup!(table, "item").unwrap();
    let mut array = unsafe { lookup!(table, "array").unwrap().as_ref() }.unwrap().as_vec();

    array.push_back(ValueRef::Owned(unsafe{ std::ptr::read(item) }));

    Ok(ValueRef::Owned(Value::Array(array)))
}

fn append(table: &mut SymbolTable) -> Result<ValueRef, String> {
    let item = unsafe { lookup!(table, "f").unwrap().as_ref() }.unwrap();
    let mut array = unsafe { lookup!(table, "array").unwrap().as_ref() }.unwrap().as_vec();

    array.push_front(ValueRef::Owned(item.clone()));

    Ok(ValueRef::Owned(Value::Array(array)))
}

fn concat(table: &mut SymbolTable) -> Result<ValueRef, String> {
    let mut init = unsafe { lookup!(table, "init").unwrap().as_ref() }.unwrap().as_vec();
    let end = unsafe { lookup!(table, "end").unwrap().as_ref() }.unwrap().as_vec();

    init.extend(end.into_iter());

    Ok(ValueRef::Owned(Value::Array(init)))
}

fn map(table: &mut SymbolTable) -> Result<ValueRef, String> {
    let f = unsafe { lookup!(table, "f").unwrap().as_ref() }.unwrap();
    let array = lookup!(table, "array").unwrap();

    let f = match f {
        Value::Function(f) => f,
        _ => return Err(format!("Argument for `f` should be a function not a {f}"))
    };

    let mut array = unsafe { array.as_ref() }.unwrap().as_vec();

    for item in array.iter_mut() {
        *item = f.exec(LinkedList::from([item.clone()]), table)?;
    }

    Ok(ValueRef::Owned(Value::Array(array)))
}

fn reduce(table: &mut SymbolTable) -> Result<ValueRef, String> {
    let f = unsafe { lookup!(table, "f").unwrap().as_ref() }.unwrap();
    let init = unsafe { lookup!(table, "init").unwrap().as_ref() }.unwrap();
    let array = lookup!(table, "array").unwrap();

    let f = match f {
        Value::Function(f) => f,
        _ => return Err(format!("Argument for `f` should be a function not a {f}"))
    };

    let array = unsafe{ array.as_ref() }.unwrap().as_vec();
    let mut acc = ValueRef::Owned(init.clone());

    for item in array.into_iter() {
        acc = f.exec(LinkedList::from([acc, item]), table)?;
    }

    Ok(acc)
}

fn scan(table: &mut SymbolTable) -> Result<ValueRef, String> {
    let f = unsafe { lookup!(table, "f").unwrap().as_ref() }.unwrap();
    let init = unsafe { lookup!(table, "init").unwrap().as_ref() }.unwrap();
    let array = lookup!(table, "array").unwrap();

    let f = match f  {
        Value::Function(f) => f,
        _ => return Err(format!("Argument for `f` should be a function not a {f}"))
    };

    let mut array = unsafe { array.as_ref() }.unwrap().as_vec();
    let mut acc = ValueRef::Owned(init.clone());

    for item in array.iter_mut() {
        acc = f.exec(LinkedList::from([acc, item.clone()]), table)?;
        *item = acc.clone();
    }

    Ok(ValueRef::Owned(Value::Array(array)))
}