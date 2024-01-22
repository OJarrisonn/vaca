use std::rc::Rc;

use vaca_core::{SymbolTable, Symbol, Value, lookup, ErrorStack};

pub fn parse_int(table: &mut SymbolTable) -> Result<Rc<Value>, ErrorStack> {
    let text = lookup!(table, "text")?;

    let parsed: i64 = match text.as_ref() {
        Value::String(s) => match s.parse() {
            Ok(i) => i,
            Err(e) => return Err(ErrorStack::Top { src: None, msg: format!("{e}")}),
        },
        other => return Err(format!("Can't parse non String {other} into an Int").into())
    };

    Ok(Rc::new(Value::Integer(parsed)))
}

pub fn parse_float(table: &mut SymbolTable) -> Result<Rc<Value>, ErrorStack> {
    let text = lookup!(table, "text")?;

    let parsed: f64 = match text.as_ref() {
        Value::String(s) => match s.parse() {
            Ok(f) => f,
            Err(e) => return Err(ErrorStack::Top { src: None, msg: format!("{e}")}),
        },
        other => return Err(format!("Can't parse non String {other} into a Float").into())
    };

    Ok(Rc::new(Value::Float(parsed)))
}