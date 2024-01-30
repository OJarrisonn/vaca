pub mod form;
pub mod program;
pub mod library;

use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "./grammar.pest"]
struct Lexer;