use std::{rc::Rc, io::Write};

use vaca_core::{Symbol, SymbolTable, lookup, register, sym, Value, function, value::function::Function, ErrorStack};

mod parse;

pub fn load(table: &mut SymbolTable) {
    register!(table, "format", function!(format, "values"));
    register!(table, "print", function!(print, "text"));
    register!(table, "println", function!(println, "text"));
    register!(table, "readln", function!(readln));
    register!(table, "parse-int", function!(parse::parse_int, "text"));
    register!(table, "parse-float", function!(parse::parse_float, "text"));
}

pub fn print(table: &mut SymbolTable) -> Result<Rc<Value>, ErrorStack> {
    let text = lookup!(table, "text")?;

    match text.as_ref() {
        Value::Array(list) => list.iter()
            .for_each(|t| print!("{}", t)),
        d => print!("{}", d)
    };

    let _ = std::io::stdout().flush();
    
    Ok(Rc::new(Value::Nil))
}

pub fn println(table: &mut SymbolTable) -> Result<Rc<Value>, ErrorStack> {
    let text = lookup!(table, "text")?;

    match text.as_ref() {
        Value::Array(list) => list.iter()
            .for_each(|t| print!("{}", t)),
        d => print!("{}", d)
    };

    println!("");
    
    Ok(Rc::new(Value::Nil))
}

pub fn readln(_table: &mut SymbolTable) -> Result<Rc<Value>, ErrorStack> {
    let mut line = String::new();
    
    let _ = std::io::stdin().read_line(&mut line);
    
    Ok(Rc::new(Value::String(line.trim().to_string())))
}

pub fn format(table: &mut SymbolTable) -> Result<Rc<Value>, ErrorStack> {
    let values = lookup!(table, "values")?;

    let formated = match values.as_ref() {
        Value::Array(list) => list.iter()
            .map(|t| t.to_string())
            .collect::<Vec<String>>()
            .join(""),
        d => d.to_string()
    };
    
    Ok(Rc::new(Value::String(formated)))
}