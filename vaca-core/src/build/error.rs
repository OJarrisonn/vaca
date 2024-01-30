use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct LexError(pub String);

#[derive(Debug)]
pub struct BuildErrorStack {
    
}

impl Error for LexError {}

impl Display for LexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for BuildErrorStack {}

impl Display for BuildErrorStack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LexError")
    }
}