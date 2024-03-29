use std::collections::LinkedList;

use rustc_hash::{FxHashMap, FxHashSet};
use crate::{value::valueref::ValueRef, ErrorStack, Symbol, Value};

/// The structure that register our definitions using the `#( ... )` syntax
/// A symbol table is a stack of scopes, with each level containing it's associations
/// This allows name shadowing by overriding a Value in a inner scope, but recovering it when exiting the scope
#[derive(Debug)]
pub struct SymbolTable {
    tables: LinkedList<FxHashMap<Symbol, ValueRef>>
}

impl SymbolTable {
    /// Creates an empty SymbolTable
    pub fn new() -> Self {
        Self {
            tables: LinkedList::new()
        }
    }

    /// Pushes a new empty scope to the top of the scope stack
    pub fn create_scope(&mut self) {
        self.tables.push_back(FxHashMap::default());
    }

    /// Drops the last scope in the stack
    /// No assertion that a last scope exists is done, but nothing bad happens if it doesn't exists
    pub fn drop_scope(&mut self) {
        let _ = self.tables.pop_back();
    }

    /// Associates a symbol to a new value in the current top scope
    pub fn register(&mut self, symbol: Symbol, value: Value) {
        self.tables.back_mut().unwrap().insert(symbol, ValueRef::own(value));
    }

    /// Tries to return a Rc to a value stored in the table if the value do exists
    pub fn lookup(&mut self, symbol: &Symbol) -> Result<ValueRef, ErrorStack> {
        match self.tables.iter().rev().find_map(|table| table.get(symbol).map(|value| ValueRef::point(value))) {
            Some(value) => Ok(value),
            None => Err(ErrorStack::Top { src: None, msg: format!("Use of undefined symbol `{}`. Maybe you misspeled or the symbol's scope got dropped", symbol) }
            ),
        }
    }

    pub fn env(&self) -> Vec<(String, String)> {
        let mut seen = FxHashSet::default();
        let mut result = Vec::new();

        for table in self.tables.iter().rev() {
            for (key, value) in table {
                if seen.contains(key) {
                    continue;
                }
                seen.insert(key);
                result.push((key.to_string(), value.to_string()));
            }
        }

        result
    }

}

#[cfg(test)]
mod tests {

    use crate::{Symbol, Value};

    use crate::SymbolTable;

    #[test]
    fn test_table() {
        let mut table = SymbolTable::new();

        table.create_scope();

        table.register(Symbol::from("a"), Value::Bool(false));

        table.create_scope();

        table.register(Symbol::from("b"), Value::Char('j'));

        table.drop_scope();

        dbg!(&table);

        table.drop_scope();
    }
}