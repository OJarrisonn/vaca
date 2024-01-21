use std::iter::zip;

use crate::{Symbol, Value, SymbolTable, Form, ValueRef};

#[derive(Debug, Clone)]
pub struct Function {
    arity: usize,
    params: Vec<Symbol>,
    partials: Vec<ValueRef>,
    body: Option<Form>,
    native: Option<NativeFunction>
}

/// The function call takes care of passing the return value to the previous ownership scope
pub type NativeFunction = fn(&mut SymbolTable) -> Result<ValueRef, String>;

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

    pub fn exec(&self, source_args: Vec<ValueRef>, table: &mut SymbolTable) -> Result<ValueRef, String> {
        if self.arity < source_args.len() {
            return Err(format!("Missmatch on argument count, expected {}, got {}", self.arity, source_args.len()));
        } else if self.arity > source_args.len() {
            return Ok(ValueRef::Owned(Value::Function(Function::partial(&self, source_args))));
        }

        let mut args = self.partials.clone();
        args.extend(source_args.into_iter());
        
        table.create_scope();

        zip(&self.params, args).for_each(|(s, v)| table.register(s.clone(), v.unwrap()));

        let res = if self.body.is_none() {
            self.native.unwrap()(table)
        } else {
            self.body.as_ref().unwrap().eval(table)
        };

        table.drop_scope();

        return res;
    }

    pub fn partial(source: &Self, args: Vec<ValueRef>) -> Self {
        let mut source = source.clone();
        source.arity -= args.len();
        source.partials.extend(args.into_iter());

        source
    }
}