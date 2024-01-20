use std::rc::Rc;

use crate::{Value, SymbolTable, Form};



pub type NativeMacro = fn(&mut SymbolTable, &Vec<Form>) -> Result<Rc<Value>, String>;