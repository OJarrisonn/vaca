use chifre::Rule;
use pest::iterators::Pair;
use vaca_core::build::form::{call::Call, function::Function, macros::Macro, Expr};

use super::{literals::parse_symbols, parse_form, parse_forms};

pub fn parse_function(function: Pair<Rule>) -> Expr {
    let parts = function.into_inner();

    let mut captures = None;
    let mut parameters = None;

    for part in parts {
        match part.as_rule() {
            Rule::captures => captures = Some(parse_symbols(part)),
            Rule::parameters => parameters = Some(parse_symbols(part)),
            Rule::form => return Expr::Function(Function::new(captures, parameters, parse_form(part.into_inner().next().unwrap()))),
            _ => unreachable!()
        }
    }

    unreachable!()
}

/// Receives a [`Rule::macrodef`] and generates a Macro Expression
pub fn parse_macro(macrodef: Pair<Rule>) -> Expr {
    let parts = macrodef.into_inner();

    let mut parameters = None;

    for part in parts {
        match part.as_rule() {
            Rule::parameters => parameters = Some(parse_symbols(part)),
            Rule::form => return Expr::Macro(Macro::new(parameters, parse_form(part.into_inner().next().unwrap()))),
            _ => unreachable!()
        }
    }

    unreachable!()
}

pub fn parse_call(function: Pair<Rule>) -> Expr {
    let mut parts = function.into_inner();

    let callable = parse_form(parts.next().unwrap().into_inner().next().unwrap());
    let forms = parse_forms(parts.next().unwrap());
    
    Expr::Call(Call::new(callable, forms))
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
    }

    #[test]
    pub fn no_capture() {
        let source = r#"<(x y -> {(a x y) (b y)})"#;
        let form = chifre::form::lex_form(source);
        assert!(form.is_ok());
        let function = parse_function(form.unwrap());
        dbg!(&function);
    }

    #[test]
    pub fn no_parameters() {
        let source = r#"<(a b c : {(a x y) (b y)})"#;
        let form = chifre::form::lex_form(source);
        assert!(form.is_ok());
        let function = parse_function(form.unwrap());
        dbg!(&function);
    }

    #[test]
    pub fn just_form() {
        let source = r#"<({(a x y) (b y)})"#;
        let form = chifre::form::lex_form(source);
        assert!(form.is_ok());
        let function = parse_function(form.unwrap());
        dbg!(&function);
    }
}