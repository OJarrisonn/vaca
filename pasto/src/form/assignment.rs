use std::error::Error;

use chifre::Rule;
use pest::iterators::Pair;
use vaca_core::build::{error::BuildErrorStack, form::{Expr, Form, Span}, symbol::Symbol};

use super::parse_form;

/// Receives a [`Rule::assignments`] and turn it into an AssignmentList expression
pub fn parse_assignment_list(assignments: Pair<Rule>) -> Expr {
    let span = Span::from(assignments.as_span());
    let list = assignments.into_inner();

    let assignments = list.map(parse_assignment).collect();
    Expr::AssignmentList(assignments)
}

/// Receives an [`Rule::assignment`] and Extracts the Symbol and Form from it
fn parse_assignment(token: Pair<Rule>) -> (Symbol, Form) {
    let span = Span::from(token.as_span());
    let mut iter = token.into_inner();
    let symbol = iter.next().unwrap().as_span().as_str().into();
    let form = parse_form(iter.next().unwrap().into_inner().next().unwrap());

    (symbol, form)
}