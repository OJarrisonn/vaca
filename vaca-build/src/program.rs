use vaca_core::Form;

use crate::library::{Library, LibraryCollection};

#[derive(Debug)]
pub struct Program {
    libraries: LibraryCollection,
    forms: Vec<Form>
}