use std::rc::Weak;

use crate::runtime::expr::Expr;

use super::{symbol_table::SymbolTable, owner::Owner, Data};

pub type NativeMacro = fn(&mut Owner, &mut SymbolTable, &Vec<Expr>) -> Result<Weak<Data>, String>;