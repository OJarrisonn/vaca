use std::rc::Rc;

use vaca_core::{Symbol, SymbolTable, lookup, register, sym, Value, function, value::function::Function, ErrorStack};

pub fn load(table: &mut SymbolTable) {
    register!(table, "nth", function!(nth, "index", "array"));
    register!(table, "map", function!(map, "f", "array"));
    register!(table, "reduce", function!(reduce, "f", "init", "array"));
    register!(table, "scan", function!(scan, "f", "init", "array"));
    register!(table, "append", function!(append, "item", "array"));
    register!(table, "prepend", function!(prepend, "item", "array"));
    register!(table, "concat", function!(concat, "init", "end"));
}

fn nth(table: &mut SymbolTable) -> Result<Rc<Value>, ErrorStack> {
    let index = lookup!(table, "index")?;
    
    let mut index = *match index.as_ref() {
        Value::Integer(i) => i,
        i => return Err(format!("Argument for `index` must be an integer not {i}").into())
    };

    let array = lookup!(table, "array")?.to_array();

    if array.len() == 0 {
        Ok(Rc::new(Value::Nil))
    } else {
        let index = if index >= 0 {
            index as usize % array.len()
        } else {
            while index < 0 {
                index += array.len() as i64
            }
            index as usize
        };

        Ok(Rc::clone(&array[index]))
    }
}

fn prepend(table: &mut SymbolTable) -> Result<Rc<Value>, ErrorStack> {
    let item = lookup!(table, "item")?;
    let mut array = lookup!(table, "array")?.to_array();

    array.insert(0, item);

    Ok(Rc::new(Value::Array(array)))
}

fn append(table: &mut SymbolTable) -> Result<Rc<Value>, ErrorStack> {
    let item = lookup!(table, "item")?;
    let mut array = lookup!(table, "array")?.to_array();

    array.push(item);

    Ok(Rc::new(Value::Array(array)))
}

fn concat(table: &mut SymbolTable) -> Result<Rc<Value>, ErrorStack> {
    let mut init = lookup!(table, "init")?.to_array();
    let end = lookup!(table, "end")?.to_array();

    init.extend(end.into_iter());

    Ok(Rc::new(Value::Array(init)))
}

fn map(table: &mut SymbolTable) -> Result<Rc<Value>, ErrorStack> {
    let f = lookup!(table, "f")?;
    
    let f = match f.as_ref() {
        Value::Function(f) => f,
        _ => return Err(format!("Argument for `f` should be a function not a {f}").into())
    };

    let mut array = lookup!(table, "array")?.to_array();

    for item in array.iter_mut() {
        *item = f.exec(vec![item.clone()], table)?;
    }

    Ok(Rc::new(Value::Array(array)))
}

fn reduce(table: &mut SymbolTable) -> Result<Rc<Value>, ErrorStack> {
    let f = lookup!(table, "f")?;
    
    let f = match f.as_ref() {
        Value::Function(f) => f,
        _ => return Err(format!("Argument for `f` should be a function not a {f}").into())
    };

    let mut acc = lookup!(table, "init")?;
    let array = lookup!(table, "array")?.to_array();

    for item in array.iter() {
        acc = f.exec(vec![acc, item.clone()], table)?;
    }

    Ok(acc)
}

fn scan(table: &mut SymbolTable) -> Result<Rc<Value>, ErrorStack> {
    let f = lookup!(table, "f")?;
    
    let f = match f.as_ref() {
        Value::Function(f) => f,
        _ => return Err(format!("Argument for `f` should be a function not a {f}").into())
    };

    let mut acc = lookup!(table, "init")?;
    let mut array = lookup!(table, "array")?.to_array();

    for item in array.iter_mut() {
        acc = f.exec(vec![acc, item.clone()], table)?;
        *item = acc.clone();
    }

    Ok(Rc::new(Value::Array(array)))
}