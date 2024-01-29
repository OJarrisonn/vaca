use pest::iterators::{Pair, Pairs};
use vaca_core::{Form, Symbol};

use crate::parser::Rule;

use super::parse_form;

pub fn parse_associations(pairs: Pairs<Rule>) -> Vec<(Symbol, Form)> {
    pairs.map(|pair| parse_association(pair))
        .collect()
}

fn parse_association(pair: Pair<Rule>) -> (Symbol, Form) {
    let mut iter = pair.into_inner();

    // TODO: Collect the description
    (Symbol::from(iter.next().unwrap().as_span().as_str()), parse_form(iter.next().unwrap()))
}