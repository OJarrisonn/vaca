use std::{collections::LinkedList, sync::{Arc, RwLock}};

use crate::{build::{form::Form, symbol::Symbol}, run::{error::RunErrorStack, table::SymbolTableTree, valueref::ValueRef}};

#[derive(Debug, Clone)]
pub struct Macro {
    arity: usize,
    params: Option<Vec<Symbol>>,
    partials: Vec<Form>,
    body: MacroBody
}

#[derive(Debug, Clone)]
enum MacroBody {
    Defined(Form),
    Native(NativeMacro)
}

pub type NativeMacro = fn(Arc<RwLock<SymbolTableTree>>, Vec<Form>) -> Result<ValueRef, RunErrorStack>;

impl Macro {
    pub fn new(params: Vec<Symbol>, body: Form) -> Self {
        Self {
            arity: params.len(), 
            params: Some(params),
            partials: vec![],
            body: MacroBody::Defined(body)
        }
    }

    pub fn native(arity: usize, native: NativeMacro) -> Self {
        Self { 
            arity, 
            params: None, 
            partials: vec![],
            body: MacroBody::Native(native) 
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