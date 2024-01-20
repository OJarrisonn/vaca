use std::rc::Rc;

use vaca_core::{Symbol, SymbolTable, lookup, register, sym, Value, function, value::function::Function};

pub fn load(table: &mut SymbolTable) {
    register!(table, "pi", Value::Float(3.1415926));
    register!(table, "+", function!(sum, "a", "b"));
    register!(table, "-", function!(sub, "a", "b"));
    register!(table, "*", function!(mul, "a", "b"));
    register!(table, "/", function!(div, "a", "b"));
}


fn generic(table: &mut SymbolTable, f: impl Fn(&f64, &f64) -> f64) -> Result<Rc<Value>, String> {
    let a = lookup!(table, "a").unwrap();
    let b = lookup!(table, "b").unwrap();

    let res = match (a.as_ref(), b.as_ref()) {
        (Value::Integer(a), Value::Integer(b)) => Value::Integer(f(&(*a as f64), &(*b as f64)) as i64),
        (Value::Float(a), Value::Integer(b)) => Value::Float(f(a, &(*b as f64))),
        (Value::Integer(a), Value::Float(b)) => Value::Float(f(&(*a as f64), b)),
        (Value::Float(a), Value::Float(b)) => Value::Float(f(a, b)),
        (Value::Integer(_), _) | (Value::Float(_), _) => return Err(format!("Argument `b` should be a numeric value not {b}")),
        (_, Value::Integer(_)) | (_, Value::Float(_)) => return Err(format!("Argument `a` should be a numeric value not {a}")),
        (_, _) => return Err(format!("Arguments for `a` and `b` should be numeric values not {a} and {b}"))
    };

    Ok(Rc::new(res))
}

fn sum(table: &mut SymbolTable) -> Result<Rc<Value>, String> {
    generic(table, |a, b| a + b)
}

fn sub(table: &mut SymbolTable) -> Result<Rc<Value>, String> {
    generic(table, |a, b| a - b)
}

fn mul(table: &mut SymbolTable) -> Result<Rc<Value>, String> {
    generic(table, |a, b| a * b)
}

fn div(table: &mut SymbolTable) -> Result<Rc<Value>, String> {
    generic(table, |a, b| a / b)
}