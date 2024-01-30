use chifre::Rule;
use pest::iterators::Pair;
use vaca_core::build::{atom::Atom, form::Expr, symbol::Symbol};

/// Receive any rule who inner is a symbol* and generates a vector of symbols
pub fn parse_symbols(symbols: Pair<Rule>) -> Vec<Symbol> {
    symbols.into_inner().map(|symbol| Symbol::from(symbol.as_span().as_str())).collect()
}

pub fn parse_literal(literal: Pair<Rule>) -> Expr {
    let src = literal.as_span().as_str();

    match literal.as_rule() {
        Rule::float => Expr::Float(src.parse().unwrap()),
        Rule::integer => Expr::Integer(src.parse().unwrap()),
        Rule::string => Expr::String(literal.into_inner().next().unwrap().as_span().as_str().into()),
        Rule::bool => Expr::Bool(src.parse().unwrap()),
        Rule::nil => Expr::Nil,
        Rule::atom => Expr::Atom(Atom::from(src)),
        Rule::symbol => Expr::Symbol(Symbol::from(src)),
        _ => unreachable!(),
    }
}