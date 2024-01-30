use speedy::{Readable, Writable};

use super::{form::Form, library::LibraryCollection};

/// The buildtime generated Program with the imported Libraries and its body
#[derive(Debug, Readable, Writable)]
pub struct Program {
    externals: LibraryCollection,
    forms: Vec<Form>
}

impl Program {
    /// Creates a new program from a LibraryCollection and a list of forms
    pub fn new(externals: LibraryCollection, forms: Vec<Form>) -> Self {
        Self { externals, forms }
    }

    pub fn forms(self) -> Vec<Form> {
        self.forms
    }
}