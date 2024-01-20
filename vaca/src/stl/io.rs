use std::{rc::Weak, io::Write};

use crate::{lookup, extract, runtime::{data::{Data, owner::symbol_table::SymbolTable, function::Function}, symbol::Symbol}, register, function, symbol};

pub fn load(table: &mut SymbolTable) {
    register!(table, "print", function!(print, "text"));
    register!(table, "println", function!(println, "text"));
    register!(table, "readln", function!(readln));
}

pub fn print(table: &mut SymbolTable) -> Result<Weak<Data>, String> {
    let text = lookup!(table, "text");

    match text.as_ref() {
        Data::Array(list) => list.iter()
            .map(|t| extract!(t))
            .for_each(|t| print!("{}", t.as_ref())),
        d => print!("{}", d)
    };

    let _ = std::io::stdout().flush();
    
    Ok(owner.allocate(Data::Nil))
}

pub fn println(table: &mut SymbolTable) -> Result<Weak<Data>, String> {
    let text = lookup!(table, "text");

    match text.as_ref() {
        Data::Array(list) => list.iter()
            .map(|t| extract!(t))
            .for_each(|t| print!("{}", t.as_ref())),
        d => print!("{}", d)
    };

    println!("");
    
    Ok(owner.allocate(Data::Nil))
}

pub fn readln(_table: &mut SymbolTable) -> Result<Weak<Data>, String> {
    let mut line = String::new();
    
    let _ = std::io::stdin().read_line(&mut line);
    
    Ok(owner.allocate(Data::String(line)))
}