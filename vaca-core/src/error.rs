use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct GenericError(String);

impl Error for GenericError {}

impl Display for GenericError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error: {}", self.0)
    }
}