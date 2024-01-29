use std::fmt::Display;

use speedy::{Writable, Readable};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Readable, Writable)]
/// Symbols are the name units of Vaca, all data association is made using symbols
/// 
/// `External` symbols can be obtained when renaming symbols imported by other libraries
/// `Local` are the symbols created in the current Vaca scope
pub enum Symbol {
    External{ns: String, symbol: String},
    Local(String)
}

impl Symbol {
    /// Sets the namespace of a [`Symbol`], if it's `Local`, turns it into an `External`.
    /// If already is an `External`, just replaces the `ns` field
    pub fn export(self, namespace: String) -> Self {
        match self {
            Symbol::External { ns: _, symbol } => Symbol::External { ns: namespace, symbol },
            Symbol::Local(symbol) => Symbol::External { ns: namespace, symbol },
        }
    }

    /// Checks if the [`Symbol`] has the mutability signal `'` at the end
    pub fn is_mutable(&self) -> bool {
        match self {
            Symbol::External { ns: _, symbol } | Symbol::Local(symbol) => symbol.ends_with("'"),
        }
    }
}

impl From<&'_ str> for Symbol {
    fn from(value: &'_ str) -> Self {
        if value.chars().next().unwrap().is_alphabetic() && value.contains("/") {
            let value: Vec<&str> = value.splitn(2, "/").collect();
            Self::External { ns: value[0].into(), symbol: value[1].into() }
        } else {
            Self::Local(value.into())
        }
    }
}

impl From<String> for Symbol {
    fn from(value: String) -> Self {
        if value.chars().next().unwrap().is_alphabetic() && value.contains("/") {
            let value: Vec<&str> = value.splitn(2, "/").collect();
            Self::External { ns: value[0].into(), symbol: value[1].into() }
        } else {
            Self::Local(value)
        }
    }
}

impl Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Symbol::External { ns, symbol } => write!(f, "{}/{}", ns, symbol),
            Symbol::Local(symbol) => write!(f, "{}", symbol),
        }
    }
}