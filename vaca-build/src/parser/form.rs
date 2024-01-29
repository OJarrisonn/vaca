use pest::iterators::Pair;
use vaca_core::{form::Literal, Form, Symbol};

use self::{assignments::parse_associations, functional::{parse_call, parse_function, parse_macro}};

use super::Rule;

mod assignments;
mod functional;

/// Takes a [`Pair`] and generates a Form from it, may receive a pair that is a `Form` or any of the Rules inside a Form
pub fn parse_form(pair: Pair<Rule>) -> Form {
    // The input is for sure one of these
    match pair.as_rule() {
        Rule::form => parse_form(pair.into_inner().next().unwrap()),
        // assignment_list > associations > association*
        Rule::assingment_list => Form::AssingmentList(parse_associations(pair.into_inner().next().unwrap().into_inner())),
        Rule::final_assingment_list => Form::FinalAssingmentList(parse_associations(pair.into_inner().next().unwrap().into_inner())),
        Rule::function => parse_function(pair.into_inner()),
        Rule::macrodef => parse_macro(pair.into_inner()),
        Rule::array_list => Form::Array(pair.into_inner().map(parse_form).collect()),
        Rule::code_block => Form::CodeBlock(pair.into_inner().map(parse_form).collect()),
        Rule::call => parse_call(pair.into_inner()),
        Rule::literal => Form::Literal(parse_literal(pair.into_inner().next().unwrap())),
        Rule::infix_call => todo!(),
        Rule::dontcare => todo!(),
        _ => {dbg!(pair); unreachable!()}
    }
}

fn parse_literal(pair: Pair<Rule>) -> Literal {
    let lit = match pair.as_rule() {
        Rule::nil => Literal::Nil,
        Rule::bool => Literal::Float(pair.as_span().as_str().parse().unwrap()),
        Rule::integer => Literal::Integer(pair.as_span().as_str().parse().unwrap()),
        Rule::float => Literal::Float(pair.as_span().as_str().parse().unwrap()),
        Rule::string => Literal::String(pair.into_inner().next().unwrap().as_span().as_str().into()),
        Rule::atom => todo!(),
        Rule::symbol => Literal::Symbol(Symbol::from(pair.as_span().as_str())),
        _ => {dbg!(pair); unreachable!()}
    };

    lit
}