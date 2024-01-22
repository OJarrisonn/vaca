use std::{error::Error, fmt::Display};

use crate::Form;

#[derive(Debug)]
pub struct GenericError(pub String);

#[derive(Debug)]
pub enum ErrorStack<'a> {
    Top{src: Option<&'a Form>, msg: String},
    Stream{src: Option<&'a Form>, from: Box<dyn std::error::Error>, note: Option<String>}
}

impl Error for GenericError {}

impl Display for GenericError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error: {}", self.0)
    }
}

impl Error for ErrorStack<'_> {}

impl Display for ErrorStack<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorStack::Top { src, msg } => { 
                match src {
                    Some(src) => {
                        writeln!(f, "Caused by: {}", src)?;
                        write!(f, "Due to {}", msg)
                    },
                    None => write!(f, "Caused by: {}", msg)
                }
            },
            ErrorStack::Stream { src, from, note } => {
                writeln!(f, "{}", from)?;

                match src {
                    Some(src) => writeln!(f, "Happened at: {}", src)?,
                    None => {}
                }
                
                if let Some(note) = note {
                    write!(f, "Note: {}", note)
                } else {
                    Ok(())
                }
            },
        }
    }
}