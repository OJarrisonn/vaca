//mod form;
pub mod value;
mod symbol;
mod table;
pub mod form;
mod error;
#[macro_use]
mod macros;

pub use crate::{value::{Value, result::ExecResult}, symbol::Symbol, table::SymbolTable, form::Form, error::GenericError};

#[cfg(test)]
mod tests {
    
}
