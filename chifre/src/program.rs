use pest::iterators::Pair;
use pest::Parser;
use vaca_core::build::error::LexError;

use crate::{Lexer, Rule};

/// Receives some input source code and generates tokens for an entire program
/// The output Ok variant has the imports in the .0, and the forms in the .1
pub fn lex_program<'a>(source: &'a str) -> Result<(Pair<'a, Rule>, Pair<'a, Rule>), LexError> {
    match Lexer::parse(Rule::program, source) {
        Ok(mut tokens) => {
            let mut program = tokens.next().unwrap().into_inner();
            let imports = program.next().unwrap();
            let forms = program.next().unwrap();

            Ok((imports, forms))
        },
        Err(err) => {
            Err(LexError(err.to_string()))
        },
    }
}