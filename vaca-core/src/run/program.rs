use std::sync::{Arc, RwLock};

use crate::build::form::Form;
use super::{library::LibraryCollection, table::SymbolTable};

/// The runtime ready Program with the imported Libraries and its body
pub struct Program {
    externals: LibraryCollection,
    table: Arc<RwLock<SymbolTable>>,
    forms: Vec<Form>
}

impl Program {
    /// Creates a new program from a LibraryCollection and a list of forms
    pub fn new(externals: LibraryCollection, forms: Vec<Form>) -> Self {
        Self { externals, forms, table: SymbolTable::root() }
    }
}