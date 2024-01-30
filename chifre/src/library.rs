use pest::iterators::Pairs;
use pest::Parser;
use vaca_core::build::error::LexError;

use crate::{Lexer, Rule};

/// Receives some input source code and generates tokens for an entire program
/// The output Ok variant has the imports in the .0, and the exports in the .1 and the assignments in the .2
/// The later ends with the EOF Rule
pub fn lex_library<'a>(source: &'a str) -> Result<(Pairs<'a, Rule>, Pairs<'a, Rule>, Pairs<'a, Rule>), LexError> {
    match Lexer::parse(Rule::library, source) {
        Ok(mut tokens) => {
            let mut library = tokens.next().unwrap().into_inner();
            let imports = library.next().unwrap().into_inner();
            let exports = library.next().unwrap().into_inner();
            let forms = library;

            Ok((imports, exports, forms))
        },
        Err(err) => {
            Err(LexError(err.to_string()))
        },
    }
}