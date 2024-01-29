use speedy::{Readable, Writable};

use crate::build::symbol::Symbol;

use super::Form;

pub type Assignment = (Symbol, Form);

#[derive(Debug, Clone, Readable, Writable)]
pub enum AssignmentKind {
    Immutable,
    Mutable,
    Final
}