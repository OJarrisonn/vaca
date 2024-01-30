use speedy::{Readable, Writable};

use crate::build::symbol::Symbol;

use super::Form;

#[derive(Debug, Clone, Readable, Writable)]

pub struct Function {
    captures: Option<Vec<Symbol>>, 
    parameters: Vec<Symbol>, 
    body: Box<Form>
}

impl Function {
    pub fn new(captures: Option<Vec<Symbol>>, parameters: Option<Vec<Symbol>>, body: Form) -> Self {
        Self {
            captures, 
            parameters: parameters.unwrap_or_default(),
            body: Box::new(body)
        }
    }
}