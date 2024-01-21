use crate::{SymbolTable, Form, ValueRef};

pub type NativeMacro = fn(&mut SymbolTable, &Vec<Form>) -> Result<ValueRef, String>;