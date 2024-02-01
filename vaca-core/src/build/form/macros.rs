use speedy::{Readable, Writable};

use crate::build::symbol::Symbol;


use super::Form;
#[derive(Debug, Clone, Readable, Writable)]
pub struct Macro {
    captures: Option<Vec<Symbol>>, 
    parameters: Vec<Symbol>, 
    body: Box<Form>
}

impl Macro {
    pub fn new(captures: Option<Vec<Symbol>>, parameters: Option<Vec<Symbol>>, body: Form) -> Self {
        Self {
            captures,
            parameters: parameters.unwrap_or_default(), 
            body: Box::new(body)
        }
    }

    pub fn captures(&self) -> &Option<Vec<Symbol>> {
        &self.captures
    }

    pub fn parameters(&self) -> &Vec<Symbol> {
        &self.parameters
    }

    pub fn body(&self) -> &Box<Form> {
        &self.body
    }
}