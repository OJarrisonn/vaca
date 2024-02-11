use std::{collections::LinkedList, sync::{Arc, RwLock}};

use rustc_hash::FxHashMap;

use crate::build::symbol::Symbol;

use super::{error::RunErrorStack, result::RunResult, value::Value, valueref::ValueRef};

#[derive(Debug)]
pub struct SymbolTableTree {
    parent: Option<Arc<RwLock<Self>>>,
    scope: FxHashMap<Symbol, SymbolTableEntry>,
    mutables: FxHashMap<Symbol, ValueRef>
}

#[derive(Debug)] 
pub struct SymbolTableStack {
    stack: LinkedList<FxHashMap<Symbol, SymbolTableEntry>>
}

#[derive(Debug)]
pub struct SymbolTableEntry {
    value: ValueRef,
    is_action: bool
}

impl SymbolTableTree {
    pub fn root() -> Arc<RwLock<Self>> {
        let root = Self {
            parent: None,
            scope: FxHashMap::default(),
            mutables: FxHashMap::default(),
        };

        Arc::new(RwLock::new(root))
    }

    pub fn from_parent(parent: &Arc<RwLock<Self>>) -> Arc<RwLock<Self>> {
        let child = Self {
            parent: Some(Arc::clone(parent)),
            scope: FxHashMap::default(),
            mutables: FxHashMap::default()
        };

        Arc::new(RwLock::new(child))
    }

    pub fn lookup(&self, symbol: &Symbol) -> RunResult<ValueRef> {
        if symbol.is_mutable() {
            match self.mutables.get(symbol) {
                Some(value) => Ok(ValueRef::clone(value)),
                None => Err(RunErrorStack::Top { src: Some(symbol.to_string()), msg: format!("use of undefined mutable symbol `{}`. Mutable symbols are only accessible in the scope they were created", symbol) }),
            }
        } else {
            match self.scope.get(symbol) {
                Some(value) => Ok(ValueRef::clone(&value.value)),
                None => match &self.parent {
                    Some(parent) => parent.read()
                        .map_err(|err| RunErrorStack::Stream { 
                            src: Some(symbol.to_string()), 
                            from: Box::new(RunErrorStack::Top { src: None, msg: err.to_string() }), 
                            note: Some(format!("failed to access parent scope while tring to get the value of `{}`", symbol)) 
                        })?.lookup(symbol),
                    None => Err(RunErrorStack::Top { src: Some(symbol.to_string()), msg: format!("use of undefined symbol `{}`", symbol) }),
                },
            }
        }
    }

    pub fn is_action(&self, symbol: &Symbol) -> RunResult<bool> {
        if symbol.is_mutable() {
            Ok(true)
        } else {
            match self.scope.get(symbol) {
                Some(value) => Ok(value.is_action),
                None => match &self.parent {
                    Some(parent) => parent.read()
                        .map_err(|err| RunErrorStack::Stream { 
                            src: Some(symbol.to_string()), 
                            from: Box::new(RunErrorStack::Top { src: None, msg: err.to_string() }), 
                            note: Some(format!("failed to access parent scope while tring to get the value of `{}`", symbol)) 
                        })?.is_action(symbol),
                    None => Err(RunErrorStack::Top { src: Some(symbol.to_string()), msg: format!("use of undefined symbol `{}`", symbol) }),
                },
            }
        }
    }

    pub fn assign(&mut self, symbol: Symbol, value: Value, is_action: bool) -> RunResult<()> {
        if symbol.is_mutable() {
            self.mutables.insert(symbol, ValueRef::new(value));
            Ok(())
        } else {
            match self.scope.get(&symbol) {
                Some(_) => Err(RunErrorStack::Top { src: Some(symbol.to_string()), msg: format!("atempt to mutate immutable symbol `{}`. If you need mutation, try creating `{}'`", symbol, symbol) }),
                _ => {
                    self.scope.insert(symbol, SymbolTableEntry { value: ValueRef::new(value), is_action });
                    Ok(())
                }
            }
        }
    }
}


impl SymbolTableStack {
    pub fn new() -> Self {
        Self { stack: LinkedList::new() }
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
            self.stack.front_mut().unwrap().insert(symbol, SymbolTableEntry { value: ValueRef::clone(&value), is_action: true });
            Ok(())
        } else {
            match self.stack.front().unwrap().get(&symbol) {
                Some(_) => Err(RunErrorStack::Top { src: Some(symbol.to_string()), msg: format!("atempt to mutate immutable symbol `{}`. If you need mutation, try creating `{}'`", symbol, symbol) }),
                _ => {
                    self.stack.front_mut().unwrap().insert(symbol, SymbolTableEntry { value: ValueRef::clone(&value), is_action });
                    Ok(())
                }
            }
        }
    }
}