use pest::iterators::Pair;
use pest::Parser;
use vaca_core::build::error::LexError;

use crate::{Lexer, Rule};

/// Receives some input source code and evaluates a single form from it
pub fn lex_form<'a>(source: &'a str) -> Result<Pair<'a, Rule>, LexError> {
    match Lexer::parse(Rule::repl_form, source) {
        Ok(mut tokens) => Ok(tokens.next().unwrap().into_inner().next().unwrap().into_inner().next().unwrap()),
        Err(err) => {
            Err(LexError(err.to_string()))
        },
    }
}