use chifre::Rule;
use pest::iterators::Pair;
use rustc_hash::FxHashMap;
use vaca_core::build::program::Program;

use crate::{form::parse_forms, BuildResult};

/// Takes a [`Rule::program`] as input and tries to build a Program 
/// This output isn't runtime-ready
/// Imports are completly ignored
pub fn parse_program((_imports, forms): (Pair<Rule>, Pair<Rule>)) -> BuildResult<Program> {
    Ok(Program::new(FxHashMap::default(), parse_forms(forms)))
}

#[cfg(test)]
mod tests {
    use chifre::program::lex_program;

    use crate::program::parse_program;

    #[test]
    pub fn hello_world() {
        let source = r#"#(message "Hello World") (pritln message)"#;
        let program = lex_program(source);
        assert!(program.is_ok());
        let program = parse_program(program.unwrap());
        assert!(program.is_ok());
    }
}