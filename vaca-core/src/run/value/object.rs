use rustc_hash::FxHashMap;

use crate::{build::symbol::Symbol, run::valueref::ValueRef};

pub type Object = FxHashMap<Symbol, ValueRef>;