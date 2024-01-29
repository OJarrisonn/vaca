use speedy::{Readable, Writable};

#[derive(Debug, Clone, Readable, Writable)]
pub struct Atom(String);

impl From<&'_ str> for Atom {
    fn from(value: &'_ str) -> Self {
        Self(value.into())
    }
}

impl From<String> for Atom {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl PartialEq for Atom {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}