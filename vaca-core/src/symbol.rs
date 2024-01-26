use std::fmt::Display;

use speedy::{Writable, Readable};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Readable, Writable)]
pub struct Symbol(String);

impl Symbol {
    pub fn add_namespace(self, ns: &str) -> Self {
        Self(format!("{}/{}", ns, self.0))
    }

    /// Takes a symbol and splits it into it's namespace and symbol
    pub fn split_namespace(self) -> (Self, Self) {
        let mut parts = self.0.splitn(2, "/");
        let ns = parts.next().unwrap_or("");
        let symbol = parts.next().unwrap_or("");
        (Self::from(ns), Self::from(symbol))
    }
}

impl From<&'_ str> for Symbol {
    fn from(value: &'_ str) -> Self {
        Self(String::from(value))
    }
}

impl From<String> for Symbol {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.0)
    }
}