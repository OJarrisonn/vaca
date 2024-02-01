use speedy::{Readable, Writable};

use crate::build::symbol::Symbol;

use super::Form;

#[derive(Debug, Clone, Readable, Writable)]

pub struct Function {
    parameters: Vec<Symbol>, 
    body: Box<Form>
}

impl Function {
    pub fn new(parameters: Option<Vec<Symbol>>, body: Form) -> Self {
        Self {
            parameters: parameters.unwrap_or_default(),
            body: Box::new(body)
        }
    }

    pub fn parameters(&self) -> &Vec<Symbol> {
        &self.parameters
    }

    pub fn body(&self) -> &Box<Form> {
        &self.body
    }
}