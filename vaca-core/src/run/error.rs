use std::{error::Error, fmt::Display};

use super::result::RunResult;

//#[derive(Debug)]
//pub struct GenericError(pub String);

#[derive(Debug)]
/// The ErrorStack is used to help debugging the callstack to see exactly what went wrong and where
/// It may contain the source form where the error happened, if you can't provide the source form, a good explanation message/note MUST be provided
/// When not at the top of the stack you may reference the Error, which can be anything that impl std::error::Error
pub enum RunErrorStack {
    Top{src: Option<String>, msg: String},
    Stream{from: Box<dyn std::error::Error>, src: Option<String>, note: Option<String>},
    MultiStream{from: Vec<Box<dyn Error>>, src: String, note: Option<String>}
}

//impl Error for GenericError {}

// impl Display for GenericError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "Error: {}", self.0)
//     }

impl RunErrorStack {
    pub fn into_result<T>(mut errs: Vec<Self>, ok: T) -> RunResult<T> {
        if errs.is_empty() {
            Ok(ok)
        } else if errs.len() == 1 {
            Err(errs.pop().unwrap())
        } else {
            Err(Self::MultiStream { from: errs.into_iter().map(|err| Box::new(err) as Box<dyn Error>).collect(), src: "program".into(), note: None })
        }
    }
}

impl From<String> for RunErrorStack {
    fn from(value: String) -> Self {
        Self::Top { src: None, msg: value }
    }
}

impl Error for RunErrorStack{}

impl Display for RunErrorStack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RunErrorStack::Top { src, msg } => { 
                match src {
                    Some(src) => {
                        writeln!(f, "Caused by: {}", src)?;
                        write!(f, "Due to {}", msg)
                    },
                    None => write!(f, "Caused by: {}", msg)
                }
            },
            RunErrorStack::Stream { src, from, note } => {
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
            RunErrorStack::MultiStream { from, src, note } => {
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