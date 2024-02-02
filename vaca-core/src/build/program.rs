use speedy::{Readable, Writable};

use super::form::Form;

/// The buildtime generated Program with the imported Libraries and its body
#[derive(Debug, Readable, Writable)]
pub struct Program {
    forms: Vec<Form>
}

impl Program {
    /// Creates a new program from a LibraryCollection and a list of forms
    pub fn new( forms: Vec<Form>) -> Self {
        Self { forms }
    }

    pub fn forms(&self) -> &Vec<Form> {
        &self.forms
    }
}

impl Into<Vec<Form>> for Program {
    fn into(self) -> Vec<Form> {
        self.forms
    }
}