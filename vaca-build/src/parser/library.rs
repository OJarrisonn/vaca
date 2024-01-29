use rustc_hash::{FxHashMap, FxHashSet};
use speedy::{Readable, Writable};
use vaca_core::{ErrorStack, Symbol};

pub type LibraryCollection = FxHashMap<Symbol, Library>;

#[derive(Debug, Readable, Writable)]
pub struct Library {
    exports: FxHashSet<Symbol>,
}

impl Library {
}