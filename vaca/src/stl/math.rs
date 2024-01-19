use std::rc::Weak;

use crate::{lookup, extract, runtime::{data::{Data, owner::Owner, symbol_table::SymbolTable}, symbol::Symbol}};

fn generic(owner: &mut Owner, table: &mut SymbolTable, f: impl Fn(&f64, &f64) -> f64) -> Result<Weak<Data>, String> {
    let a = lookup!(table, "a");
    let b = lookup!(table, "b");

    let res = match (a.as_ref(), b.as_ref()) {
        (Data::Integer(a), Data::Integer(b)) => Data::Integer(f(&(*a as f64), &(*b as f64)) as i64),
        (Data::Float(a), Data::Integer(b)) => Data::Float(f(a, &(*b as f64))),
        (Data::Integer(a), Data::Float(b)) => Data::Float(f(&(*a as f64), b)),
        (Data::Float(a), Data::Float(b)) => Data::Float(f(a, b)),
        (Data::Integer(_), _) | (Data::Float(_), _) => return Err(format!("Argument `b` should be a numeric value not {b}")),
        (_, Data::Integer(_)) | (_, Data::Float(_)) => return Err(format!("Argument `a` should be a numeric value not {a}")),
        (_, _) => return Err(format!("Arguments for `a` and `b` should be numeric values not {a} and {b}"))
    };

    Ok(owner.allocate(res))
}

pub fn sum(owner: &mut Owner, table: &mut SymbolTable) -> Result<Weak<Data>, String> {
    generic(owner, table, |a, b| a + b)
}

pub fn sub(owner: &mut Owner, table: &mut SymbolTable) -> Result<Weak<Data>, String> {
    generic(owner, table, |a, b| a - b)
}

pub fn mul(owner: &mut Owner, table: &mut SymbolTable) -> Result<Weak<Data>, String> {
    generic(owner, table, |a, b| a * b)
}

pub fn div(owner: &mut Owner, table: &mut SymbolTable) -> Result<Weak<Data>, String> {
    generic(owner, table, |a, b| a / b)
}