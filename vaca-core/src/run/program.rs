use std::sync::{Arc, RwLock};

use crate::build::{self, form::Form};
use super::table::SymbolTable;

/// The runtime ready Program with the imported Libraries and its body
pub struct Program {
    table: Arc<RwLock<SymbolTable>>,
    forms: Vec<Form>
}

impl Program {
    /// Creates a new program from a LibraryCollection and a list of forms
    pub fn new(forms: Vec<Form>) -> Self {
        Self { forms, table: SymbolTable::root() }
    }

    pub fn table(&self) -> Arc<RwLock<SymbolTable>> {
        Arc::clone(&self.table)
    }

    pub fn forms(&self) -> &Vec<Form> {
        &self.forms
    }
}

impl From<build::program::Program> for Program {
    fn from(value: build::program::Program) -> Self {
        Self::new(value.into())
    }
}