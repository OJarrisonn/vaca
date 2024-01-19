use std::{rc::Weak, iter::zip};

use crate::{Symbol, Expr, Data, Owner, SymbolTable};

#[derive(Debug, Clone)]
pub struct Function {
    arity: usize,
    params: Vec<Symbol>,
    partials: Vec<Weak<Data>>,
    body: Option<Expr>,
    native: Option<NativeFunction>
}

/// The function call takes care of passing the return value to the previous ownership scope
pub type NativeFunction = fn(&mut Owner, &mut SymbolTable) -> Result<Weak<Data>, String>;

impl Function {
    pub fn new(params: Vec<Symbol>, body: Expr) -> Self {
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

    pub fn exec(&self, source_args: Vec<Weak<Data>>, owner: &mut Owner, table: &mut SymbolTable) -> Result<Weak<Data>, String> {
        if self.arity < source_args.len() {
            return Err(format!("Missmatch on argument count, expected {}, got {}", self.arity, source_args.len()));
        } else if self.arity > source_args.len() {
            return Ok(owner.allocate(Data::Function(Function::partial(&self, source_args))));
        }

        let mut args = self.partials.clone();
        args.extend(source_args.into_iter());
        
        owner.create_scope();
        table.create_scope();

        zip(&self.params, args).for_each(|(s, v)| table.insert(s.clone(), v));

        let res = if self.body.is_none() {
            self.native.unwrap()(owner, table)
        } else {
            self.body.as_ref().unwrap().eval(owner, table)
        };

        let res = match res {
            Err(e) => Err(e),
            Ok(d) => Ok(owner.allocate_return(d)),
        };

        table.drop_scope();
        owner.drop_scope();

        return res;
    }

    pub fn partial(source: &Self, args: Vec<Weak<Data>>) -> Self {
        let mut source = source.clone();
        source.arity -= args.len();
        source.partials.extend(args.into_iter());

        source
    }
}