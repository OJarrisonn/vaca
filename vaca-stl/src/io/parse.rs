use std::rc::Rc;

use vaca_core::{SymbolTable, Symbol, Value, lookup};

pub fn parse_int(table: &mut SymbolTable) -> Result<Rc<Value>, String> {
    let text = lookup!(table, "text").unwrap();

    let parsed: i64 = match text.as_ref() {
        Value::String(s) => match s.parse() {
            Ok(i) => i,
            Err(e) => return Err(format!("{e}")),
        },
        other => return Err(format!("Can't parse non String {other} into an Int"))
    };

    Ok(Rc::new(Value::Integer(parsed)))
}

pub fn parse_float(table: &mut SymbolTable) -> Result<Rc<Value>, String> {
    let text = lookup!(table, "text").unwrap();

    let parsed: f64 = match text.as_ref() {
        Value::String(s) => match s.parse() {
            Ok(f) => f,
            Err(e) => return Err(format!("{e}")),
        },
        other => return Err(format!("Can't parse non String {other} into a Float"))
    };

    Ok(Rc::new(Value::Float(parsed)))
}