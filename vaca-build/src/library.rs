use rustc_hash::{FxHashSet, FxHashMap};
use vaca_core::{ErrorStack, Symbol, SymbolTable, Value};
use vaca_stl as stl;

pub type LibraryCollection = FxHashMap<Symbol, Library>;

#[derive(Debug)]
pub struct Library {
    libraries: LibraryCollection,
    exports: FxHashSet<Symbol>, 
    table: SymbolTable
}

impl Library {
    pub fn build() -> Result<Self, ErrorStack> {
        
    }

    fn new(libraries: Vec<(Symbol, Library)>, exports: Vec<Symbol>, assignments: Vec<(Symbol, Value)>) -> Self {
        let mut libs = FxHashMap::default();

        for (alias, lib) in libraries {
            libs.insert(alias, lib);
        } 

        let mut exp = FxHashSet::default();
        for symbol in exports {
            exp.insert(symbol);
        }

        let mut table = stl::create_table();
        for (symbol, value) in assignments.into_iter() {
            table.register(symbol, value);
        }

        Self {
            libraries: libs,
            exports: exp,
            table
        }
    }
}