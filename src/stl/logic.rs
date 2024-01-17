use std::rc::Weak;

use crate::{lookup, extract, runtime::{data::{Data, owner::Owner, symbol_table::SymbolTable}, symbol::Symbol}};

pub fn iff(owner: &mut Owner, table: &mut SymbolTable) -> Result<Weak<Data>, String> {
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