use std::collections::LinkedList;

use rustc_hash::FxHashSet;
use vaca_core::build::{error::BuildErrorStack, symbol::Symbol};

use crate::BuildResult;

#[derive(Debug)]
pub struct TrackTable {
    scopes: LinkedList<FxHashSet<Symbol>>
}

impl TrackTable {
    pub fn new() -> Self {
        Self { scopes: LinkedList::new() }
    }

    /// Pushes a new scope to the TrackTable
    pub fn create_scope(&mut self) {
        self.scopes.push_front(FxHashSet::default());
    }

    /// Drops the most recent pushed scope from the TrackTable
    pub fn drop_scope(&mut self) {
        self.scopes.pop_front();
    }

    /// Checks if the symbol is already assigned
    /// 
    /// For mutable symbols, checks only the current scope.
    /// For immutables checks the whole table
    fn is_assigned(&self, symbol: &Symbol) -> bool {
        if symbol.is_mutable() {
            if let Some(scope) = self.scopes.front() {
                scope.contains(symbol)
            } else {
                false
            }
        } else {
            self.scopes.iter().any(|scope| scope.contains(symbol))
        }
    }

    /// Checks if the symbol can be assigned
    /// 
    /// For mutable symbols is always true.
    /// For immutables checks if it isn't already assigned in the current scope
    fn can_assign(&self, symbol: &Symbol) -> bool {
        if symbol.is_mutable() {
            true
        } else {
            if let Some(scope) = self.scopes.front() {
                !scope.contains(symbol)
            } else {
                false
            }
        }
    }

    /// Checks if the symbol is assignable, if so assigns it, otherwise throws an error
    pub fn assign(&mut self, symbol: &Symbol) -> BuildResult<()> {
        if self.can_assign(symbol) {
            self.scopes.front_mut().unwrap().insert(symbol.clone());
            Ok(())
        } else {
            Err(BuildErrorStack::Top { src: symbol.to_string(), msg: format!("attempt to reassign immutable symbol {}", symbol) })
        }
    }

    /// Checks if the symbol is assigned, if don't throws an error
    /// TODO: Check if the symbol has an Action associated
    pub fn validate(&mut self, symbol: &Symbol) -> BuildResult<()> {
        if self.is_assigned(symbol) {
            Ok(())
        } else {
            Err(BuildErrorStack::Top { src: symbol.to_string(), msg: if symbol.is_mutable() {
                    format!("attempt to use undefined mutable symbol {symbol}. Remember that mutable symbols aren't accessible in inner scopes.")
                } else {
                    format!("attemp to use undefined symbol {symbol}.")
                }})
        }
    }
}