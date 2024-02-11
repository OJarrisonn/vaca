use std::fmt::Display;

use crate::run::valueref::ValueRef;

use self::{assignment::Assignment, call::Call, function::Function, macros::Macro};

use super::{atom::Atom, symbol::Symbol};

pub mod assignment;
pub mod function;
pub mod macros;
pub mod call;

// TODO: Made them serializable
#[derive(Debug, Clone)]
pub struct Form {
    pub expr: Expr,
    pub span: Span
}

#[derive(Debug, Clone)]
pub struct Span {
    src: String,
    pos: (usize, usize)
}

#[derive(Debug, Clone)]
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
    Capture(ValueRef)
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

    pub fn into_expr(self) -> Expr {
        self.expr
    }
    pub fn into_span(self) -> Span {
        self.span
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

impl From<&'_ str> for Span {
    fn from(value: &'_ str) -> Self {
        Span { src: value.into(), pos: (0, 0) }
    }
}

impl Span {
    pub fn pos(&self) -> &(usize, usize) {
        &self.pos
    }
}