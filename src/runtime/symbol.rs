use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Symbol(String);

impl From<&'static str> for Symbol {
    fn from(value: &'static str) -> Self {
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