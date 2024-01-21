use vaca_core::{SymbolTable, Symbol, Value, lookup, ValueRef};

pub fn parse_int(table: &mut SymbolTable) -> Result<ValueRef, String> {
    let text = unsafe { lookup!(table, "text").unwrap().as_ref() }.unwrap();

    let parsed: i64 = match text {
        Value::String(s) => match s.parse() {
            Ok(i) => i,
            Err(e) => return Err(format!("{e}")),
        },
        other => return Err(format!("Can't parse non String {other} into an Int"))
    };

    Ok(ValueRef::Owned(Value::Integer(parsed)))
}

pub fn parse_float(table: &mut SymbolTable) -> Result<ValueRef, String> {
    let text = unsafe { lookup!(table, "text").unwrap().as_ref() }.unwrap();

    let parsed: f64 = match text {
        Value::String(s) => match s.parse() {
            Ok(f) => f,
            Err(e) => return Err(format!("{e}")),
        },
        other => return Err(format!("Can't parse non String {other} into a Float"))
    };

    Ok(ValueRef::Owned(Value::Float(parsed)))
}