use std::collections::LinkedList;

use crate::build::{form::Form, symbol::Symbol};

#[derive(Debug, Clone)]
pub struct Macro {
    arity: usize,
    params: Option<Vec<Symbol>>,
    partials: Vec<Form>,
    body: Form
}

impl Macro {
    pub fn new(params: Vec<Symbol>, body: Form) -> Self {
        Self {
            arity: params.len(), 
            params: Some(params),
            partials: vec![],
            body
        }
    }

    pub fn arity(&self) -> usize {
        self.arity
    }

    fn partial(&self, forms: LinkedList<Form>) -> Self {
        let mut applied = self.clone();
        applied.arity -= forms.len();
        applied.partials.extend(forms.into_iter());

        applied
    }
}