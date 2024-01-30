use rustc_hash::FxHashMap;

use crate::build::symbol::Symbol;

use self::table::LibTable;

mod table;

pub struct Library {
    externals: LibraryCollection,
    locals: LibTable
}

pub type LibraryCollection = FxHashMap<Symbol, Library>;