use crate::{build::{form::Form, symbol::Symbol}, run::{error::RunErrorStack, table::SymbolTableStack, valueref::ValueRef}};

use super::array::Array;

#[derive(Debug, Clone)]
pub struct Function {
    pub arity: usize,
    pub params: Vec<Symbol>,
    pub partials: Vec<ValueRef>,
    pub body: Option<Form>,
    pub native: Option<NativeFunction>
}

/// The function call takes care of passing the return value to the previous ownership scope
pub type NativeFunction = fn(&mut SymbolTableStack) -> Result<ValueRef, RunErrorStack>;

impl Function {
    pub fn new(params: Vec<Symbol>, body: Form) -> Self {
        Self {
            arity: params.len(), 
            params,
            partials: vec![],
            body: Some(body),
            native: None
        }
    }

    pub fn native(params: Vec<Symbol>, native: NativeFunction) -> Self {
        Self { 
            arity: params.len(), 
            params, 
            partials: vec![],
            body: None, 
            native: Some(native) 
        }
    }

    pub fn arity(&self) -> usize {
        self.arity
    }

    pub fn partial(source: &Self, args: Array) -> Self {
        let mut source = source.clone();
        source.arity -= args.len();
        source.partials.extend(args.into_iter());

        source
    }

    pub fn span_from_args(&self) -> String {
        self.params.iter().map(|symbol| symbol.to_string()).collect::<Vec<String>>().join(" ")
    }
}