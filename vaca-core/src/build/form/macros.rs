use speedy::{Readable, Writable};

use crate::build::symbol::Symbol;


use super::Form;
#[derive(Debug, Clone, Readable, Writable)]
pub struct Macro {
    parameters: Vec<Symbol>, 
    body: Box<Form>
}