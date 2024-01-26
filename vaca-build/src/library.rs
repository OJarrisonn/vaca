use rustc_hash::{FxHashSet, FxHashMap};
use vaca_core::{value::valueref::ValueRef, Symbol, SymbolTable};

pub type LibraryCollection = FxHashMap<Symbol, Library>;

#[derive(Debug)]
pub struct Library {
    libraries: LibraryCollection,
    exports: FxHashSet<Symbol>, 
    table: SymbolTable
}

impl Library {
    pub fn build(libraries: Vec<(Symbol, Library)>, exports: Vec<Symbol>, assignments: Vec<Symbol, ValueRef>) -> Self {
        let mut libs = FxHashMap::default();

        for (alias, lib) in libraries {
            libs.insert(alias, lib);
        } 

        let mut exp = FxHashSet::default();
        for symbol in exports {
            exp.insert(symbol);
        }



        Self {
            libraries: libs,
            exports: exp,
            table
        }
    }
}