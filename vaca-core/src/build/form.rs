use speedy::{Readable, Writable};

use self::{assignment::{Assignment, AssignmentKind}, call::Call, function::Function, macros::Macro};

use super::{atom::Atom, symbol::Symbol};

pub mod assignment;
pub mod function;
pub mod macros;
pub mod call;

#[derive(Debug, Clone, Readable, Writable)]
pub struct Form {
    expr: FormExpr,
    span: FormSpan
}

#[derive(Debug, Clone, Readable, Writable)]
pub struct FormSpan {
    src: String,
    pos: (usize, usize)
}

#[derive(Debug, Clone, Readable, Writable)]
pub enum FormExpr {
    Nil,
    Integer(i64),
    Float(f64),
    String(String),
    Bool(bool),
    Symbol(Symbol),
    Atom(Atom),
    AssignmentList(Vec<Assignment>, AssignmentKind),
    Scope(Vec<Form>),
    Function(Function),
    Macro(Macro),
    Call(Call),
    Array(Vec<Form>),
}