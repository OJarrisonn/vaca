use rustc_hash::FxHashMap;

use crate::{build::symbol::Symbol, run::valueref::ValueRef};

/// The Library equivalent of a SymbolTable
/// 
/// Constains the registry of all the exported symbols
///
/// All the non-exported symbols should be evaluated and replaced during build
pub struct LibTable {
    assignments: FxHashMap<Symbol, ValueRef>
}

impl LibTable {
    /// Creates a new LibTable with only the public assignments
    /// 
    /// All the local assignments should be resolved during build
    pub fn new(entries: Vec<(Symbol, ValueRef)>) -> Self {
        let mut assignments = FxHashMap::default();
        for (k, v) in entries {
            assignments.insert(k, v);
        }

        Self { assignments }
    }

    /// Retrieves an exported symbol from the table
    /// 
    /// It doesn't checks if the symbol exists, since undefined symbols are treated during Build
    pub fn lookup(&self, symbol: Symbol) -> ValueRef {
        ValueRef::clone(self.assignments.get(&symbol).expect("Undefined symbols aren't being treated"))
    }
}