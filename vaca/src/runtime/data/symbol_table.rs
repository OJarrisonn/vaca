use std::{rc::Weak, collections::{HashMap, LinkedList}};

use crate::{Data, Symbol};

#[derive(Debug)]
pub struct SymbolScope(HashMap<Symbol, Weak<Data>>);

pub struct SymbolTable {
    scopes: LinkedList<SymbolScope>
}

impl SymbolScope {
    pub fn new() -> Self {
        Self(HashMap::new())
    }
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            scopes: LinkedList::new()
        }
    }

    pub fn create_scope(&mut self) {
        self.scopes.push_back(SymbolScope::new());
    }

    pub fn drop_scope(&mut self) {
        self.scopes.pop_back();
    }

    pub fn insert(&mut self, symbol: Symbol, value: Weak<Data>) {
        self.scopes.back_mut().unwrap().0.insert(symbol, value);
    }

    pub fn lookup(&self, symbol: &Symbol) -> Result<Weak<Data>,String> {
        match self.scopes.iter()
            .rfind(|scope| scope.0.contains_key(symbol)) {
                Some(scope) => Ok(Weak::clone(scope.0.get(symbol).unwrap())),
                None => Err(format!("Tried to lookup for undefined symbol `{symbol}`")),
            }
    }

    pub fn env(&self) -> Vec<(String, String)> {
        let mut symbols = self.scopes.iter()
            .map(|scope| scope.0.keys().collect::<Vec<&Symbol>>())
            .reduce(|mut acc, keys| { acc.extend(keys.iter()); acc })
            .unwrap_or(vec![]);

        symbols.dedup();

        symbols.iter().map(|symbol| (symbol.to_string(), self.lookup(symbol).unwrap().upgrade().unwrap().to_string())).collect()

    }
}