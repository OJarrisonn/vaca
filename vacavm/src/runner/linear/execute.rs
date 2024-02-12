use std::{collections::LinkedList, iter::zip};

use vaca_core::{build::form::Form, run::{result::RunResult, value::{function::Function, macros::Macro, Value}, valueref::ValueRef}};

use super::{run_form, table::SymbolTableStack};

pub fn execute_function(table: &mut SymbolTableStack, func: &Function, arguments: LinkedList<ValueRef>) -> RunResult<ValueRef> {
    if arguments.len() > func.arity() {
        return Err(vaca_core::run::error::RunErrorStack::Top { src: None, msg: format!("Too many arguments provided to function call. Expected {}, got {}", func.arity(), arguments.len()) })
    } 
    
    if arguments.len() < func.arity() {
        return Ok(ValueRef::new(Value::Function(Function::partial(&func, arguments.into_iter().collect()))))
    }

    table.create_scope();

    // Assign each argument to it's parameter name
    zip(&func.params, arguments).for_each(|(symbol, value)| {let _ = table.assign(symbol.clone(), value, false); });

    // TODO: Find a better solution than cloning the function body everytime you run it
    let res = run_form(table, func.body.clone());

    table.drop_scope();

    res
}

pub fn execute_macro(table: &mut SymbolTableStack, mac: &Macro, arguments: Vec<Form>) -> RunResult<ValueRef> {
    todo!()
}