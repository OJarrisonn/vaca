use chifre::Rule;
use pest::iterators::Pair;
use vaca_core::build::form::Expr;

use super::parse_forms;

/// Receives a [`Rule::array_list`] and turns it into an [`Expr::Array`]
pub fn parse_array(array: Pair<Rule>) -> Expr {
    Expr::Array(parse_forms(array.into_inner().next().unwrap())) // The inner of array is forms
}

/// Receives a [`Rule::scope`] and turns it into an [`Expr::Array`]
pub fn parse_scope(scope: Pair<Rule>) -> Expr {
    Expr::Scope(parse_forms(scope.into_inner().next().unwrap())) // The inner of array is forms
}