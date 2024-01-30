use chifre::Rule;
use pest::iterators::Pair;
use vaca_core::build::{error::BuildErrorStack, form::Expr};

use super::parse_forms;

/// Receives a [`Rule::array_list`] and turns it into an [`Expr::Array`]
pub fn parse_array(array: Pair<Rule>) -> Result<Expr, BuildErrorStack> {
    let span = array.as_span().into();

    parse_forms(array.into_inner().next().unwrap()) // The inner of array is forms
        .map(|ok| Expr::Array(ok))
        .map_err(|err| BuildErrorStack::Stream { from: Box::new(err), src: span, note: None })
}

/// Receives a [`Rule::scope`] and turns it into an [`Expr::Array`]
pub fn parse_array(scope: Pair<Rule>) -> Result<Expr, BuildErrorStack> {
    let span = scope.as_span().into();

    parse_forms(array.into_inner().next().unwrap()) // The inner of array is forms
        .map(|ok| Expr::Array(ok))
        .map_err(|err| BuildErrorStack::Stream { from: Box::new(err), src: span, note: None })
}