use std::error::Error;

use chifre::Rule;
use pest::iterators::Pair;
use vaca_core::build::{error::BuildErrorStack, form::{Form, Span}};

mod assignment;
mod collections;
mod callables;
mod symbols;

/// Receives a [`Rule::form`] or any of its possible inners and creates a Form
pub fn parse_form(form: Pair<Rule>) -> Form, BuildErrorStack> {
    let span = Span::from(form.as_span());

    let expr = match form.as_rule() {
        Rule::assingment_list | Rule::mut_assingment_list => assignment::parse_assignment_list(form.into_inner().next().unwrap()),
        Rule::array_list => collections::parse_array(form),
        Rule::scope => collections::parse_scope(form),
        Rule::function => callables::parse_function(form),
        Rule::macrodef => todo!(),
        Rule::call => todo!(),
        Rule::infix_call => todo!(),
        Rule::literal => todo!(),
        Rule::dontcare => todo!(),
        Rule::form => return parse_form(form),
        _ => unreachable!()
    };

    match expr {
        Ok(expr) => Ok(Form::new(span, expr)),
        Err(err) => Err(BuildErrorStack::Stream { from: Box::new(err), src: span, note: None }),
    }

}

pub fn parse_forms(forms: Pair<Rule>) -> Result<Vec<Form>, BuildErrorStack> {
    let span = forms.as_span().into();
    
    let (oks, mut errs): (Vec<_>, Vec<_>) = forms.into_inner()
        .map(|form| parse_form(form.into_inner().next().unwrap()))
        .partition(|form| form.is_ok());

    if errs.is_empty() {
        let forms = oks.into_iter().map(Result::unwrap).collect::<Vec<Form>>();
        Ok(forms)
    } else if errs.len() == 1 {
        let err = BuildErrorStack::Stream { from: Box::new(errs.pop().unwrap().unwrap_err()), src: span, note: Some("A single form parsing failed in the current scope".into()) };
        Err(err)
    } else {
        let count = errs.len();
        let err = BuildErrorStack::MultiStream { 
            from: errs.into_iter().map(|err| Box::new(err.unwrap_err()) as Box<dyn Error>).collect(), 
            src: span, 
            note: Some(format!("{} forms parsing failed in the current scope", count)) };
        Err(err)
    }
}