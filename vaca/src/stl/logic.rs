use std::rc::Weak;

use crate::{lookup, extract, runtime::{data::{Data, owner::symbol_table::SymbolTable, function::Function}, symbol::Symbol, expr::Expr}, register, function, symbol};

pub fn load(table: &mut SymbolTable) {
    register!(table, "if", Data::Macro(if_macro));
    register!(table, "==", function!(eq, "a", "b"));
    register!(table, "!=", function!(neq, "a", "b"));
    register!(table, "<", function!(lt, "a", "b"));
    register!(table, ">", function!(gt, "a", "b"));
    register!(table, "<=", function!(le, "a", "b"));
    register!(table, ">=", function!(ge, "a", "b"));
    register!(table, "&", function!(and, "a", "b"));
    register!(table, "|", function!(or, "a", "b"));
}

fn if_macro(table: &mut SymbolTable, args: &Vec<Expr>) -> Result<Weak<Data>, String> {
    if args.len() != 3 {
        return Err(format!("Wrong argument count for if. Needed a condition, a truth expression and a fake expression"))
    }
    let cond = &args[0];
    let truth = &args[1];
    let fake = &args[2];

    if extract!(cond.eval(table)?).as_boolean() {
        truth.eval(table)
    } else {
        fake.eval(table)
    }
}

/*fn iff(table: &mut SymbolTable) -> Result<Weak<Data>, String> {
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
}*/

fn generic_rel(table: &mut SymbolTable, f: impl Fn(&Data, &Data) -> bool) -> Result<Weak<Data>, String> {
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

fn eq(table: &mut SymbolTable) -> Result<Weak<Data>, String> {
    generic_rel(table, |a, b| a == b)
}

fn neq(table: &mut SymbolTable) -> Result<Weak<Data>, String> {
    generic_rel(table, |a, b| a != b)
}

fn gt(table: &mut SymbolTable) -> Result<Weak<Data>, String> {
    generic_rel(table, |a, b| a > b)
}

fn lt(table: &mut SymbolTable) -> Result<Weak<Data>, String> {
    generic_rel(table, |a, b| a < b)
}

fn ge(table: &mut SymbolTable) -> Result<Weak<Data>, String> {
    generic_rel(table, |a, b| a >= b)
}

fn le(table: &mut SymbolTable) -> Result<Weak<Data>, String> {
    generic_rel(table, |a, b| a <= b)
}

fn generic_bool(table: &mut SymbolTable, f: impl Fn(bool, bool) -> bool) -> Result<Weak<Data>, String> {
    let a = lookup!(table, "a");
    let b = lookup!(table, "b");

    match (a.as_ref(), b.as_ref()) {
        (Data::Bool(bl), Data::Bool(br)) => Ok(owner.allocate(Data::Bool(f(*bl, *br)))),
        (Data::Bool(_), _) => Err(format!("Argument `b` should be a boolean value not {b}")),
        (_, Data::Bool(_)) => Err(format!("Argument `a` should be a boolean value not {a}")),
        (a, b) => Err(format!("Trying boolean operation between {a} with {b} which isn't possible"))
    }
}

fn and(table: &mut SymbolTable) -> Result<Weak<Data>, String> {
    generic_bool(table, |a, b| a && b)
}

fn or(table: &mut SymbolTable) -> Result<Weak<Data>, String> {
    generic_bool(table, |a, b| a || b)
}