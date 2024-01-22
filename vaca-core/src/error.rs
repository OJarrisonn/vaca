use std::{error::Error, fmt::Display};

//#[derive(Debug)]
//pub struct GenericError(pub String);

#[derive(Debug)]
/// The ErrorStack is used to help debugging the callstack to see exactly what went wrong and where
/// It may contain the source form where the error happened, if you can't provide the source form, a good explanation message/note MUST be provided
/// When not at the top of the stack you may reference the Error, which can be anything that impl std::error::Error
pub enum ErrorStack {
    Top{src: Option<String>, msg: String},
    Stream{src: Option<String>, from: Box<dyn std::error::Error>, note: Option<String>}
}

//impl Error for GenericError {}

// impl Display for GenericError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "Error: {}", self.0)
//     }
//}

impl From<String> for ErrorStack {
    fn from(value: String) -> Self {
        Self::Top { src: None, msg: value }
    }
}

impl Error for ErrorStack{}

impl Display for ErrorStack {
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