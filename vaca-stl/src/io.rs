use std::{rc::Rc, io::Write};


use vaca_core::{Symbol, SymbolTable, lookup, register, sym, Value, function, value::function::Function};

pub fn load(table: &mut SymbolTable) {
    register!(table, "print", function!(print, "text"));
    register!(table, "println", function!(println, "text"));
    register!(table, "readln", function!(readln));
}

pub fn print(table: &mut SymbolTable) -> Result<Rc<Value>, String> {
    let text = lookup!(table, "text").unwrap();

    match text.as_ref() {
        Value::Array(list) => list.iter()
            .for_each(|t| print!("{}", t)),
        d => print!("{}", d)
    };

    let _ = std::io::stdout().flush();
    
    Ok(Rc::new(Value::Nil))
}

pub fn println(table: &mut SymbolTable) -> Result<Rc<Value>, String> {
    let text = lookup!(table, "text").unwrap();

    match text.as_ref() {
        Value::Array(list) => list.iter()
            .for_each(|t| print!("{}", t)),
        d => print!("{}", d)
    };

    println!("");
    
    Ok(Rc::new(Value::Nil))
}

pub fn readln(_table: &mut SymbolTable) -> Result<Rc<Value>, String> {
    let mut line = String::new();
    
    let _ = std::io::stdin().read_line(&mut line);
    
    Ok(Rc::new(Value::String(line)))
}