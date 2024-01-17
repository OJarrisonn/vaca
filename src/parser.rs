use pest::{Parser, iterators::Pair};
use pest_derive::Parser;

use crate::{Expr, Literal, Symbol, runtime::symbol};

#[derive(Parser)]
#[grammar = "./parser/grammar.pest"]
pub struct VacaParser;

pub fn parse(form: String) -> Result<Expr, String>{
    let res = VacaParser::parse(Rule::form, &form);

    match res {
        Ok(mut pairs) => pair_walk(pairs.next().unwrap()),
        Err(e) => Err(format!("{}", e)),
    }
}

fn pair_walk(pair: Pair<'_, Rule>) -> Result<Expr, String>{
    match pair.as_rule() {
        // Unreachble
        Rule::keyword => todo!(),
        Rule::operators => todo!(),
        Rule::WHITESPACE => todo!(),
        Rule::COMMENT => todo!(),
        Rule::atom => todo!(),
        // Direct recursion
        Rule::form => pair_walk(pair.into_inner().next().unwrap()),
        Rule::literal => pair_walk(pair.into_inner().next().unwrap()),
        Rule::string => pair_walk(pair.into_inner().next().unwrap()),
        Rule::char => pair_walk(pair.into_inner().next().unwrap()),
        // Get the text and wraps it into a Symbol
        Rule::symbol => Ok(Expr::Literal(Literal::Symbol(Symbol::from(pair.as_str())))),
        Rule::float => Ok(Expr::Literal(Literal::Float(pair.as_str().parse().unwrap()))),
        Rule::integer => Ok(Expr::Literal(Literal::Integer(pair.as_str().parse().unwrap()))),
        Rule::string_content => Ok(Expr::Literal(Literal::String(pair.as_str().to_string()))),
        Rule::char_content => Ok(Expr::Literal(Literal::Char(pair.as_str().chars().next().unwrap_or('\0')))),
        Rule::bool => Ok(Expr::Literal(Literal::Bool(pair.as_str() == "true"))),
        Rule::nil => Ok(Expr::Literal(Literal::Nil)),
        // Collection rules
        Rule::assingment_list => {
            let res = pair.into_inner()
                .map(|pair| pair_walk(pair))
                .fold(Ok(vec![]), |acc, a| {
                    match acc {
                        Ok(mut acc) => match a {
                            Ok(Expr::Assingment(symbol, expr)) => {
                                acc.push((symbol, *expr));
                                Ok(acc)
                            },
                            Ok(_) => panic!("This must be an assinment"),
                            Err(e) => Err(e)
                        },
                        e => e
                    }
                });

            match res {
                Ok(list) => Ok(Expr::AssingmentList(list)),
                Err(e) => Err(e),
            }
        },
        Rule::export_list => todo!(),
        Rule::assingment => {
            let mut iter = pair.into_inner();
            let symbol: Symbol = iter.next().unwrap().as_str().into();
            let expr = pair_walk(iter.next().unwrap());

            match expr {
                Err(e) => Err(e),
                Ok(expr) => Ok(Expr::Assingment(symbol, Box::new(expr))),
            }
        },
        // TODO: complete this
        Rule::array_list => todo!(),
        Rule::function => todo!(),
        Rule::call => todo!(),
        Rule::code_block => todo!(),
    }
}

mod tests {
    use std::fs;

    use pest::Parser;

    use super::{parse, VacaParser, Rule};

    #[test]
    fn crude_parse() {
        let _ = dbg!(VacaParser::parse(Rule::form, &(format!("{{{}}}", fs::read_to_string("./tests/assingments.vaca").unwrap()))));
    }
}