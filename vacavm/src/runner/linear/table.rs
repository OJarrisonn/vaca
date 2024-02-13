use std::collections::LinkedList;

use rustc_hash::FxHashMap;
use vaca_core::{build::symbol::Symbol, run::{error::RunErrorStack, external::ExternalTable, result::RunResult, valueref::ValueRef}};

use super::native::NativeObject;

#[derive(Debug)] 
pub struct SymbolTableStack {
    stack: LinkedList<FxHashMap<Symbol, Entry>>,
    externals: FxHashMap<String, ExternalTable<NativeObject>>
}

#[derive(Debug)]
struct Entry {
    value: ValueRef,
    is_action: bool
}

impl SymbolTableStack {
    pub fn new() -> Self {
        Self { stack: LinkedList::new(), externals: FxHashMap::default() }
    }

    pub fn create_scope(&mut self) {
        self.stack.push_front(FxHashMap::default())
    }

    pub fn drop_scope(&mut self) {
        self.stack.pop_front();
    }

    pub fn lookup(&self, symbol: &Symbol) -> RunResult<ValueRef> {
        if symbol.is_mutable() {
            match self.stack.front().unwrap().get(symbol) {
                Some(value) => Ok(ValueRef::clone(&value.value)),
                None => Err(RunErrorStack::Top { src: Some(symbol.to_string()), msg: format!("use of undefined mutable symbol `{}`. Mutable symbols are only accessible in the scope they were created", symbol) }),
            }
        } else {
            match self.stack.iter().find_map(|scope| scope.get(symbol)) {
                Some(entry) => Ok(ValueRef::clone(&entry.value)),
                None => Err(RunErrorStack::Top { src: Some(symbol.to_string()), msg: format!("use of undefined symbol `{}`", symbol)}),
            }
        }
    }

    pub fn is_action(&self, symbol: &Symbol) -> RunResult<bool> {
        if symbol.is_mutable() {
            Ok(true)
        } else {
            match self.stack.iter().find_map(|scope| scope.get(symbol)) {
                Some(entry) => Ok(entry.is_action),
                None => Err(RunErrorStack::Top { src: Some(symbol.to_string()), msg: format!("use of undefined mutable symbol `{}`", symbol)}),
            }
        }
    }

    pub fn assign(&mut self, symbol: Symbol, value: ValueRef, is_action: bool) -> RunResult<()> {
        if symbol.is_mutable() {
            self.stack.front_mut().unwrap().insert(symbol, Entry { value: ValueRef::clone(&value), is_action: true });
            Ok(())
        } else {
            match self.stack.front().unwrap().get(&symbol) {
                Some(_) => Err(RunErrorStack::Top { src: Some(symbol.to_string()), msg: format!("atempt to mutate immutable symbol `{}`. If you need mutation, try creating `{}'`", symbol, symbol) }),
                _ => {
                    self.stack.front_mut().unwrap().insert(symbol, Entry { value: ValueRef::clone(&value), is_action });
                    Ok(())
                }
            }
        }
    }
}