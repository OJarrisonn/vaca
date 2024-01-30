use std::error::Error;

use chifre::Rule;
use pest::iterators::Pair;
use vaca_core::build::{error::BuildErrorStack, form::{Expr, Form, Span}, symbol::Symbol};

use super::parse_form;

/// Receives a [`Rule::assignments`] and turn it into an AssignmentList expression
pub fn parse_assignment_list(assignments: Pair<Rule>) -> Result<Expr, BuildErrorStack> {
    let span = Span::from(assignments.as_span());
    let list = assignments.into_inner();

    let (oks, mut errs): (Vec<_>, Vec<_>) = list.map(parse_assignment).partition(|res| res.is_ok());

    // Checks for errors and create the proper BuildErrorStack if needed
    if errs.is_empty() {
        let assignments = oks.into_iter().map(Result::unwrap).collect::<Vec<(Symbol, Form)>>();
        Ok(Expr::AssignmentList(assignments))
    } else if errs.len() == 1 {
        let err = BuildErrorStack::Stream { from: Box::new(errs.pop().unwrap().unwrap_err()), src: span, note: Some("A single assignment failed in the current assignment list".into()) };
        Err(err)
    } else {
        let count = errs.len();
        let err = BuildErrorStack::MultiStream { 
            from: errs.into_iter().map(|err| Box::new(err.unwrap_err()) as Box<dyn Error>).collect(), 
            src: span, 
            note: Some(format!("{} assignments failed", count)) };
        Err(err)
    }
}

/// Receives an [`Rule::assignment`] and Extracts the Symbol and Form from it
fn parse_assignment(token: Pair<Rule>) -> Result<(Symbol, Form), BuildErrorStack> {
    let span = Span::from(token.as_span());
    let mut iter = token.into_inner();
    let symbol = iter.next().unwrap().as_span().as_str().into();
    let form = parse_form(iter.next().unwrap().into_inner().next().unwrap()).map_err(|err| BuildErrorStack::Stream { from: Box::new(err), src: span, note: Some("The parsing of the form passed to the current assignment failed".into()) })?;

    Ok((symbol, form))
}