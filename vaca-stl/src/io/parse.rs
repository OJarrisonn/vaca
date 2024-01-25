use vaca_core::{lookup, value::valueref::ValueRef, ErrorStack, Symbol, SymbolTable, Value};

pub fn parse_int(table: &mut SymbolTable) -> Result<ValueRef, ErrorStack> {
    let text = lookup!(table, "text")?;

    let parsed: i64 = match text.as_ref() {
        Value::String(s) => match s.parse() {
            Ok(i) => i,
            Err(e) => return Err(ErrorStack::Top { src: None, msg: format!("{e}")}),
        },
        other => return Err(format!("Can't parse non String {other} into an Int").into())
    };

    Ok(ValueRef::own(Value::Integer(parsed)))
}

pub fn parse_float(table: &mut SymbolTable) -> Result<ValueRef, ErrorStack> {
    let text = lookup!(table, "text")?;

    let parsed: f64 = match text.as_ref() {
        Value::String(s) => match s.parse() {
            Ok(f) => f,
            Err(e) => return Err(ErrorStack::Top { src: None, msg: format!("{e}")}),
        },
        other => return Err(format!("Can't parse non String {other} into a Float").into())
    };

    Ok(ValueRef::own(Value::Float(parsed)))
}