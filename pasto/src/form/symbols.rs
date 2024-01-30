use chifre::Rule;
use pest::iterators::Pair;
use vaca_core::build::symbol::Symbol;

/// Receive any rule who inner is a symbol* and generates a vector of symbols
pub fn parse_symbols(symbols: Pair<Rule>) -> Vec<Symbol> {
    symbols.into_inner().map(|symbol| Symbol::from(symbol.as_span().as_str())).collect()
}