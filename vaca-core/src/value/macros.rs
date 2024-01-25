use std::{collections::LinkedList, iter::zip};

use crate::{ErrorStack, Form, Symbol, SymbolTable, Value};

use super::valueref::ValueRef;

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

pub type NativeMacro = fn(&mut SymbolTable, Vec<Form>) -> Result<ValueRef, ErrorStack>;

impl Macro {
    pub fn defined(params: Vec<Symbol>, body: Form) -> Self {
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

    pub fn exec(&self, table: &mut SymbolTable, source_forms: LinkedList<Form>) -> Result<ValueRef, ErrorStack> {
        if self.arity != 0 && self.arity < source_forms.len() {
            return Err(ErrorStack::Top { 
                src: None, 
                msg: format!("Too many arguments passed to macro call, expected {}, but got {}", self.arity, source_forms.len()) 
            });
        } else if self.arity > source_forms.len() {
            return Ok(ValueRef::own(Value::Macro(self.partial(source_forms))));
        }

        let mut forms = self.partials.clone();
        forms.extend(source_forms.into_iter());
        
        table.create_scope();

        let res = match &self.body {
            MacroBody::Defined(def) => {
                let mut def = def.clone();
                for (s, f) in zip(self.params.as_ref().unwrap(), forms) {
                    def = def.replace_symbol(s, &f);
                }
                
                def.eval(table)
            },
            MacroBody::Native(nat) => nat(table, forms),
        };

        table.drop_scope();

        return res;
    }

    fn partial(&self, forms: LinkedList<Form>) -> Self {
        let mut applied = self.clone();
        applied.arity -= forms.len();
        applied.partials.extend(forms.into_iter());

        applied
    }
}