use std::rc::Rc;

use vaca_core::{Symbol, SymbolTable, lookup, register, sym, Value, function, value::function::Function, ErrorStack};

pub fn load(table: &mut SymbolTable) {
    register!(table, "pi", Value::Float(3.1415926));
    register!(table, "+", function!(sum, "a", "b"));
    register!(table, "-", function!(sub, "a", "b"));
    register!(table, "*", function!(mul, "a", "b"));
    register!(table, "/", function!(div, "a", "b"));
    register!(table, "^", function!(pow, "a", "b"));
    register!(table, "brt", function!(root, "a", "b"));
    register!(table, "mod", function!(modulus, "a", "b"));
    register!(table, "//", function!(intdiv, "a", "b"));
    register!(table, "max", function!(max, "a", "b"));
    register!(table, "min", function!(min, "a", "b"));
}


fn generic(table: &mut SymbolTable, f: impl Fn(&f64, &f64) -> f64) -> Result<Rc<Value>, ErrorStack> {
    let a = lookup!(table, "a")?;
    let b = lookup!(table, "b")?;

    let res = match (a.as_ref(), b.as_ref()) {
        (Value::Integer(a), Value::Integer(b)) => { 
            let f = f(&(*a as f64), &(*b as f64)); 
            if f.is_nan() { 
                Value::NotANumber 
            } else { 
                Value::Integer(f as i64) 
            } 
        },
        (Value::Float(a), Value::Integer(b)) => Value::Float(f(a, &(*b as f64))),
        (Value::Integer(a), Value::Float(b)) => Value::Float(f(&(*a as f64), b)),
        (Value::Float(a), Value::Float(b)) => Value::Float(f(a, b)),
        (Value::Integer(_), _) | (Value::Float(_), _) => return Err(format!("Argument `b` should be a numeric value not `{b}`").into()),
        (_, Value::Integer(_)) | (_, Value::Float(_)) => return Err(format!("Argument `a` should be a numeric value not `{a}`").into()),
        (_, _) => return Err(format!("Arguments for `a` and `b` should be numeric values not `{a}` and `{b}`").into())
    };

    let res = match res {
        Value::Float(f) if f.is_nan() => Value::NotANumber,
        v => v
    };

    Ok(Rc::new(res))
}

fn sum(table: &mut SymbolTable) -> Result<Rc<Value>, ErrorStack> {
    generic(table, |a, b| a + b)
}

fn sub(table: &mut SymbolTable) -> Result<Rc<Value>, ErrorStack> {
    generic(table, |a, b| a - b)
}

fn mul(table: &mut SymbolTable) -> Result<Rc<Value>, ErrorStack> {
    generic(table, |a, b| a * b)
}

fn pow(table: &mut SymbolTable) -> Result<Rc<Value>, ErrorStack> {
    generic(table, |a, b| a.powf(*b))
}

fn root(table: &mut SymbolTable) -> Result<Rc<Value>, ErrorStack> {
    generic(table, |a, b| a.powf(1.0 / *b))
}

fn div(table: &mut SymbolTable) -> Result<Rc<Value>, ErrorStack> {
    generic(table, |a, b| a / b)
}

fn intdiv(table: &mut SymbolTable) -> Result<Rc<Value>, ErrorStack> {
    generic(table, |a, b| ((*a) as i64 / (*b) as i64) as f64)
}

fn modulus(table: &mut SymbolTable) -> Result<Rc<Value>, ErrorStack> {
    generic(table, |a, b| ((*a) as i64 % (*b) as i64) as f64)
}

fn max(table: &mut SymbolTable) -> Result<Rc<Value>, ErrorStack> {
    generic(table, |a, b| a.max(*b))
}

fn min(table: &mut SymbolTable) -> Result<Rc<Value>, ErrorStack> {
    generic(table, |a, b| a.min(*b))
}