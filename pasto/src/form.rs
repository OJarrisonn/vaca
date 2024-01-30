use chifre::Rule;
use pest::iterators::Pair;
use vaca_core::build::form::{Form, Span};

mod assignment;
mod collections;
mod callables;
mod literals;

/// Receives a [`Rule::form`] or any of its possible inners and creates a Form
pub fn parse_form(form: Pair<Rule>) -> Form {
    let span = Span::from(form.as_span());

    let expr = match form.as_rule() {
        Rule::assingment_list | Rule::mut_assingment_list => assignment::parse_assignment_list(form.into_inner().next().unwrap()),
        Rule::array_list => collections::parse_array(form),
        Rule::scope => collections::parse_scope(form),
        Rule::function => callables::parse_function(form),
        Rule::macrodef => callables::parse_macro(form),
        Rule::call => callables::parse_call(form),
        Rule::infix_call => todo!(),//callables::parse_infix(form),
        Rule::literal => literals::parse_literal(form),
        Rule::dontcare => todo!(),
        Rule::form => return parse_form(form),
        _ => unreachable!()
    };

    Form::new(span, expr)

}

/// Receive a [`Rule::forms`] and generate a vector of each form parsed
pub fn parse_forms(forms: Pair<Rule>) -> Vec<Form> {
    forms.into_inner()
        .map(|form| parse_form(form.into_inner().next().unwrap()))
        .collect()
}