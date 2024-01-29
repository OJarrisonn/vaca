use pest::iterators::Pairs;
use vaca_core::{Form, Symbol};

use crate::parser::Rule;

use super::parse_form;

pub fn parse_function(pairs: Pairs<Rule>) -> Form {
    let mut captures = None;
    let mut params = Vec::new();

    for pair in pairs {
        match pair.as_rule() {
            Rule::captures => captures = Some(pair.into_inner().map(|pair| Symbol::from(pair.as_span().as_str())).collect()),
            Rule::parameters => params = pair.into_inner().map(|pair| Symbol::from(pair.as_span().as_str())).collect(),
            Rule::form => return Form::Function(captures, params, Box::new(parse_form(pair))),
            _ => unreachable!()
        }
    }
    
    unreachable!()
}

pub fn parse_macro(pairs: Pairs<Rule>) -> Form {
    let mut params = Vec::new();

    for pair in pairs {
        match pair.as_rule() {
            Rule::parameters => params = pair.into_inner().map(|pair| Symbol::from(pair.as_span().as_str())).collect(),
            Rule::form => return Form::Macro(params, Box::new(parse_form(pair))),
            _ => unreachable!()
        }
    }
    
    unreachable!()
}

pub fn parse_call(mut pairs: Pairs<Rule>) -> Form {
    let callable = pairs.next().unwrap();
    let args = pairs.next().unwrap();

    Form::Call(Box::new(parse_form(callable.into_inner().next().unwrap())), args.into_inner().map(parse_form).collect())
}