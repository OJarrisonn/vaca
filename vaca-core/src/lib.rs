//mod form;
mod value;
mod symbol;
mod table;
mod form;
mod error;

pub use crate::{value::Value, symbol::Symbol, table::SymbolTable, form::Form, error::GenericError};

#[cfg(test)]
mod tests {
    
}
