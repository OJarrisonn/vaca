use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct LexError(pub String);

#[derive(Debug)]
pub enum BuildErrorStack {
    Top{src: String, msg: String},
    Stream{from: Box<dyn Error>, src: String, note: Option<String>},
    MultiStream{from: Vec<Box<dyn Error>>, src: String, note: Option<String>}
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
        match self {
            BuildErrorStack::Top { src, msg } => {
                writeln!(f, "Caused at: {src}")?;
                write!(f, "Due to {msg}")
            },
            BuildErrorStack::Stream { from, src, note } => {
                writeln!(f, "{from}")?;
                write!(f, "Caused at: {src}")?;
                match note {
                    Some(note) => write!(f, "\n{note}"),
                    None => write!(f, "")
                }
            },
            BuildErrorStack::MultiStream { from, src, note } => {
                writeln!(f, "Multiple Error Stream caused by: {src}")?;
                for err in from.iter() {
                    writeln!(f, "{err}")?;
                }
                match note {
                    Some(note) => writeln!(f, "{note}")?,
                    None => (),
                }
                write!(f, "End of Error Stream caused")
            }
        }
    }
}