use speedy::{Readable, Writable};

use super::Form;

#[derive(Debug, Clone, Readable, Writable)]
pub struct Call {
    callable: Box<Form>, 
    arguments: Vec<Form>
}