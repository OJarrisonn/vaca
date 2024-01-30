use speedy::{Readable, Writable};
use rustc_hash::{FxHashMap, FxHashSet};

use super::{form::Form, symbol::Symbol};

#[derive(Debug, Readable, Writable)]
pub struct Library {
    externals: LibraryCollection,
    exports: FxHashSet<Symbol>,
    locals: FxHashMap<Symbol, Form>
}

pub type LibraryCollection = FxHashMap<Symbol, Library>;