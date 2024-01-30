use chifre::Rule;
use pest::iterators::Pair;
use vaca_core::build::form::{Expr, Span};

use super::symbols::parse_symbols;

pub fn parse_function(function: Pair<Rule>) -> Expr {
    let span = Span::from(function.as_span());

    let parts = function.into_inner();

    let mut captures = None;
    let mut parameters = None;

    for part in parts {
        match part.as_rule() {
            Rule::captures => captures = Some(parse_symbols(part)),
            Rule::parameters => parameters = Some(parse_symbols(part)),
            Rule::form => ,
            _ => unreachable!()
        }
    }

    Expr::Nil
}

#[cfg(test)]
mod tests {
    use chifre;

    use crate::form::callables::parse_function;

    #[test]
    pub fn full_signature() {
        let source = r#"<(a b c : x y -> {(a x y) (b y)})"#;
        let form = chifre::form::lex_form(source);
        assert!(form.is_ok());
        let function = parse_function(form.unwrap());
        dbg!(&function);
        assert!(function.is_ok());
    }

    #[test]
    pub fn no_capture() {
        let source = r#"<(x y -> {(a x y) (b y)})"#;
        let form = chifre::form::lex_form(source);
        assert!(form.is_ok());
        let function = parse_function(form.unwrap());
        dbg!(&function);
        assert!(function.is_ok());
    }

    #[test]
    pub fn no_parameters() {
        let source = r#"<(a b c : {(a x y) (b y)})"#;
        let form = chifre::form::lex_form(source);
        assert!(form.is_ok());
        let function = parse_function(form.unwrap());
        dbg!(&function);
        assert!(function.is_ok());
    }

    #[test]
    pub fn just_form() {
        let source = r#"<({(a x y) (b y)})"#;
        let form = chifre::form::lex_form(source);
        assert!(form.is_ok());
        let function = parse_function(form.unwrap());
        dbg!(&function);
        assert!(function.is_ok());
    }
}