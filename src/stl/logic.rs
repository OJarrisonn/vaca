use std::rc::Weak;

use crate::{lookup, extract, runtime::{data::{Data, owner::Owner, symbol_table::SymbolTable, function::Function}, symbol::Symbol}, register, function, symbol};

pub fn load(owner: &mut Owner, table: &mut SymbolTable) {
    register!(owner, table, "if", function!(iff, "cond", "truth", "fake"));
    register!(owner, table, "==", function!(eq, "a", "b"));
    register!(owner, table, "!=", function!(neq, "a", "b"));
    register!(owner, table, "<", function!(lt, "a", "b"));
    register!(owner, table, ">", function!(gt, "a", "b"));
    register!(owner, table, "<=", function!(le, "a", "b"));
    register!(owner, table, ">=", function!(ge, "a", "b"));
    register!(owner, table, "&", function!(and, "a", "b"));
    register!(owner, table, "|", function!(or, "a", "b"));
}

fn iff(owner: &mut Owner, table: &mut SymbolTable) -> Result<Weak<Data>, String> {
    let cond = lookup!(table, "cond");
    let truth = lookup!(table, "truth");
    let fake = lookup!(table, "fake");

    let cond = match cond.as_ref() {
        Data::Nil => false,
        Data::Bool(b) => *b,
        Data::Integer(i) => *i != 0,
        Data::Float(f) => *f != 0.,
        Data::Char(c) => *c != '\0',
        Data::String(s) => s.len() != 0,
        Data::Array(a) => a.len() != 0,
        Data::Function(_) => false,
    };

    if cond {
        Ok(owner.relocate(truth))
    } else {
        Ok(owner.relocate(fake))
    }
}

fn generic_rel(owner: &mut Owner, table: &mut SymbolTable, f: impl Fn(&Data, &Data) -> bool) -> Result<Weak<Data>, String> {
    let a = lookup!(table, "a");
    let b = lookup!(table, "b");

    match (a.as_ref(), b.as_ref()) {
        (Data::Function(_), _) => Err(format!("Argument for `a` is a function which isn't comparable")),
        (_, Data::Function(_)) => Err(format!("Argument for `b` is a function which isn't comparable")),
        (Data::Bool(_), Data::Bool(_)) |
        (Data::Integer(_), Data::Integer(_)) |
        (Data::Integer(_), Data::Float(_)) |
        (Data::Float(_), Data::Float(_)) |
        (Data::Float(_), Data::Integer(_)) |
        (Data::Char(_), Data::Char(_)) |
        (Data::String(_), Data::String(_)) |
        (Data::Array(_), Data::Array(_)) |
        (Data::Nil, Data::Nil) => Ok(owner.allocate(Data::Bool(f(a.as_ref(), b.as_ref())))),
        (a, b) => Err(format!("Trying to compare {a} with {b} which isn't possible")) 
    }
}

fn eq(owner: &mut Owner, table: &mut SymbolTable) -> Result<Weak<Data>, String> {
    generic_rel(owner, table, |a, b| a == b)
}

fn neq(owner: &mut Owner, table: &mut SymbolTable) -> Result<Weak<Data>, String> {
    generic_rel(owner, table, |a, b| a != b)
}

fn gt(owner: &mut Owner, table: &mut SymbolTable) -> Result<Weak<Data>, String> {
    generic_rel(owner, table, |a, b| a > b)
}

fn lt(owner: &mut Owner, table: &mut SymbolTable) -> Result<Weak<Data>, String> {
    generic_rel(owner, table, |a, b| a < b)
}

fn ge(owner: &mut Owner, table: &mut SymbolTable) -> Result<Weak<Data>, String> {
    generic_rel(owner, table, |a, b| a >= b)
}

fn le(owner: &mut Owner, table: &mut SymbolTable) -> Result<Weak<Data>, String> {
    generic_rel(owner, table, |a, b| a <= b)
}

fn generic_bool(owner: &mut Owner, table: &mut SymbolTable, f: impl Fn(bool, bool) -> bool) -> Result<Weak<Data>, String> {
    let a = lookup!(table, "a");
    let b = lookup!(table, "b");

    match (a.as_ref(), b.as_ref()) {
        (Data::Bool(bl), Data::Bool(br)) => Ok(owner.allocate(Data::Bool(f(*bl, *br)))),
        (Data::Bool(_), _) => Err(format!("Argument `b` should be a boolean value not {b}")),
        (_, Data::Bool(_)) => Err(format!("Argument `a` should be a boolean value not {a}")),
        (a, b) => Err(format!("Trying boolean operation between {a} with {b} which isn't possible"))
    }
}

fn and(owner: &mut Owner, table: &mut SymbolTable) -> Result<Weak<Data>, String> {
    generic_bool(owner, table, |a, b| a && b)
}

fn or(owner: &mut Owner, table: &mut SymbolTable) -> Result<Weak<Data>, String> {
    generic_bool(owner, table, |a, b| a || b)
}