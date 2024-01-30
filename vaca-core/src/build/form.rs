use std::fmt::Display;

use speedy::{Readable, Writable};

use self::{assignment::Assignment, call::Call, function::Function, macros::Macro};

use super::{atom::Atom, symbol::Symbol};

pub mod assignment;
pub mod function;
pub mod macros;
pub mod call;

#[derive(Debug, Clone, Readable, Writable)]
pub struct Form {
    expr: Expr,
    span: Span
}

#[derive(Debug, Clone, Readable, Writable)]
pub struct Span {
    src: String,
    pos: (usize, usize)
}

#[derive(Debug, Clone, Readable, Writable)]
pub enum Expr {
    Nil,
    Integer(i64),
    Float(f64),
    String(String),
    Bool(bool),
    Symbol(Symbol),
    Atom(Atom),
    AssignmentList(Vec<Assignment>),
    Scope(Vec<Form>),
    Function(Function),
    Macro(Macro),
    Call(Call),
    Array(Vec<Form>),
}

impl Form {
    pub fn new(span: Span, expr: Expr) -> Self {
        Self {span, expr}
    }

    pub fn span(&self) -> &Span {
        &self.span
    }
    pub fn expr(&self) -> &Expr {
        &self.expr
    }
}

impl Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}:{}) => {}", self.pos.0, self.pos.1, &self.src)
    }
}

impl From<pest::Span<'_>> for Span {
    fn from(value: pest::Span<'_>) -> Self {
        Span {
            pos: value.start_pos().line_col(),
            src: value.as_str().into()
        }
    }
}

impl Span {
    pub fn pos(&self) -> &(usize, usize) {
        &self.pos
    }
}