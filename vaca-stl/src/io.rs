use std::io::Write;

use vaca_core::{Symbol, SymbolTable, lookup, register, sym, Value, function, value::function::Function, ValueRef};

mod parse;

pub fn load(table: &mut SymbolTable) {
    register!(table, "print", function!(print, "text"));
    register!(table, "println", function!(println, "text"));
    register!(table, "readln", function!(readln));
    register!(table, "parse-int", function!(parse::parse_int, "text"));
    register!(table, "parse-float", function!(parse::parse_float, "text"));
}

pub fn print(table: &mut SymbolTable) -> Result<ValueRef, String> {
    let text = unsafe { lookup!(table, "text").unwrap().as_ref() };

    let text = match text {
        Some(text) => text,
        None => return Err(format!("`text` is null")),
    };

    match text {
        Value::Array(list) => list.iter()
            .for_each(|t| print!("{}", t)),
        d => print!("{}", d)
    };

    let _ = std::io::stdout().flush();
    
    Ok(ValueRef::Owned(Value::Nil))
}

pub fn println(table: &mut SymbolTable) -> Result<ValueRef, String> {
    let text = unsafe { lookup!(table, "text").unwrap().as_ref() };

    let text = match text {
        Some(text) => text,
        None => return Err(format!("`text` is null")),
    };

    match text {
        Value::Array(list) => list.iter()
            .for_each(|t| print!("{}", t)),
        d => print!("{}", d)
    };

    println!("");
    
    Ok(ValueRef::Owned(Value::Nil))
}

pub fn readln(_table: &mut SymbolTable) -> Result<ValueRef, String> {
    let mut line = String::new();
    
    let _ = std::io::stdin().read_line(&mut line);
    
    Ok(ValueRef::Owned(Value::String(line.trim().to_string())))
}