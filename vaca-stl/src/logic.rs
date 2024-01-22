use std::rc::Rc;

use vaca_core::{Symbol, SymbolTable, lookup, register, sym, Value, Form, function, value::function::Function, ErrorStack};

pub fn load(table: &mut SymbolTable) {
    register!(table, "if", Value::Macro(if_macro));
    register!(table, "assert", Value::Macro(assert));
    register!(table, "==", function!(eq, "a", "b"));
    register!(table, "!=", function!(neq, "a", "b"));
    register!(table, "<", function!(lt, "a", "b"));
    register!(table, ">", function!(gt, "a", "b"));
    register!(table, "<=", function!(le, "a", "b"));
    register!(table, ">=", function!(ge, "a", "b"));
    register!(table, "&", function!(and, "a", "b"));
    register!(table, "|", function!(or, "a", "b"));
}

fn if_macro(table: &mut SymbolTable, args: &Vec<Form>) -> Result<Rc<Value>, ErrorStack> {
    if args.len() != 3 {
        return Err(format!("Wrong argument count for if. Needed a condition, a truth expression and a fake expression").into())
    }
    let cond = &args[0];
    let truth = &args[1];
    let fake = &args[2];

    if cond.eval(table)?.as_boolean() {
        truth.eval(table)
    } else {
        fake.eval(table)
    }
}


fn assert(table: &mut SymbolTable, args: &Vec<Form>) -> Result<Rc<Value>, ErrorStack> {
    for arg in args.iter() {
        if !arg.eval(table).map_err(|err| ErrorStack::Stream { src: Some(arg.to_string()), from: Box::new(err), note: None })?.as_boolean() {
            return Err(ErrorStack::Top { src: Some(arg.to_string()), msg: "Assertion failed".into() })
        }
    }

    Ok(Rc::new(Value::Bool(true)))
}

fn generic_rel(table: &mut SymbolTable, f: impl Fn(&Value, &Value) -> bool) -> Result<Rc<Value>, ErrorStack> {
    let a = lookup!(table, "a").unwrap();
    let b = lookup!(table, "b").unwrap();

    match (a.as_ref(), b.as_ref()) {
        (Value::Function(_), _) => Err(format!("Argument for `a` is a function which isn't comparable").into()),
        (_, Value::Function(_)) => Err(format!("Argument for `b` is a function which isn't comparable").into()),
        (Value::Bool(_), Value::Bool(_)) |
        (Value::Integer(_), Value::Integer(_)) |
        (Value::Integer(_), Value::Float(_)) |
        (Value::Float(_), Value::Float(_)) |
        (Value::Float(_), Value::Integer(_)) |
        (Value::Char(_), Value::Char(_)) |
        (Value::String(_), Value::String(_)) |
        (Value::Array(_), Value::Array(_)) |
        (Value::Nil, Value::Nil) => Ok(Rc::new(Value::Bool(f(a.as_ref(), b.as_ref())))),
        (a, b) => Err(format!("Trying to compare {a} with {b} which isn't possible").into()) 
    }
}

fn eq(table: &mut SymbolTable) -> Result<Rc<Value>, ErrorStack> {
    generic_rel(table, |a, b| a == b)
}

fn neq(table: &mut SymbolTable) -> Result<Rc<Value>, ErrorStack> {
    generic_rel(table, |a, b| a != b)
}

fn gt(table: &mut SymbolTable) -> Result<Rc<Value>, ErrorStack> {
    generic_rel(table, |a, b| a > b)
}

fn lt(table: &mut SymbolTable) -> Result<Rc<Value>, ErrorStack> {
    generic_rel(table, |a, b| a < b)
}

fn ge(table: &mut SymbolTable) -> Result<Rc<Value>, ErrorStack> {
    generic_rel(table, |a, b| a >= b)
}

fn le(table: &mut SymbolTable) -> Result<Rc<Value>, ErrorStack> {
    generic_rel(table, |a, b| a <= b)
}

fn generic_bool(table: &mut SymbolTable, f: impl Fn(bool, bool) -> bool) -> Result<Rc<Value>, ErrorStack> {
    let a = lookup!(table, "a").unwrap();
    let b = lookup!(table, "b").unwrap();

    match (a.as_ref(), b.as_ref()) {
        (Value::Bool(bl), Value::Bool(br)) => Ok(Rc::new(Value::Bool(f(*bl, *br)))),
        (Value::Bool(_), _) => Err(format!("Argument `b` should be a boolean value not {b}").into()),
        (_, Value::Bool(_)) => Err(format!("Argument `a` should be a boolean value not {a}").into()),
        (a, b) => Err(format!("Trying boolean operation between {a} with {b} which isn't possible").into())
    }
}

fn and(table: &mut SymbolTable) -> Result<Rc<Value>, ErrorStack> {
    generic_bool(table, |a, b| a && b)
}

fn or(table: &mut SymbolTable) -> Result<Rc<Value>, ErrorStack> {
    generic_bool(table, |a, b| a || b)
}