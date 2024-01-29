use speedy::{Readable, Writable};

use crate::build::symbol::Symbol;

use super::Form;

#[derive(Debug, Clone, Readable, Writable)]

pub struct Function {
    captures: Option<Vec<Symbol>>, 
    parameters: Vec<Symbol>, 
    body: Box<Form>
}