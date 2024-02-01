use speedy::{Readable, Writable};

use super::Form;

#[derive(Debug, Clone, Readable, Writable)]
pub struct Call {
    callable: Box<Form>, 
    arguments: Vec<Form>
}

impl Call {
    pub fn new(callable: Form, arguments: Vec<Form>) -> Self {
        Self { callable: Box::new(callable), arguments }
    }

    pub fn callable(&self) -> &Box<Form> {
        &self.callable
    }

    pub fn arguments(&self) -> &Vec<Form> {
        &self.arguments
    }
}