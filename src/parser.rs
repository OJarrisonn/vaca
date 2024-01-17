use pest::{Parser, iterators::Pair};
use pest_derive::Parser;

use crate::{Expr, Literal, Symbol};

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
        Rule::keyword => unreachable!(),
        Rule::operators => unreachable!(),
        Rule::WHITESPACE => unreachable!(),
        Rule::COMMENT => unreachable!(),
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
        
        Rule::string_content => Ok(Expr::Literal(Literal::String(pair.as_str().to_string().replace("\\n", "\n").replace("\\r", "\r")))),

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
        Rule::array_list => {
            let res =  pair.into_inner()
                .map(|pair| pair_walk(pair))
                .fold(Ok(vec![]), |acc, r| {
                    match acc {
                        Ok(mut acc) => match r {
                            Ok(r) => {acc.push(r); Ok(acc)},
                            Err(e) => Err(e),
                        },
                        Err(e) => Err(e),
                    }
                });
            
            match res {
                Ok(list) => Ok(Expr::Array(list)),
                Err(e) => Err(e),
            }
        },
        Rule::function => {
            let mut params = vec![];
            let mut body = vec![];

            for p in pair.into_inner() {
                match p.as_rule() {
                    Rule::symbol => params.push(p.as_str().into()),
                    Rule::form => body.push(pair_walk(p)?),
                    _ => panic!("No other rule should be inside a function rule")
                }
            }

            let body = Expr::CodeBlock(body);

            Ok(Expr::Function(params, Box::new(body)))
        },
        Rule::call => {
            let res = pair.into_inner()
                .map(|pair| pair_walk(pair));

            // TODO: Remove find
            if let Some(err) = res.clone().into_iter().find(|r| r.is_err()) {
                err
            } else {
                let mut iter = res.map(|e| e.unwrap());
                let func = iter.next().expect("There should be a first expression in the function");

                match &func {
                    Expr::Function(_, _) | Expr::CodeBlock(_) | Expr::Literal(Literal::Symbol(_)) | Expr::Call(_, _) => Ok(Expr::Call(Box::new(func), iter.collect())),
                    e => Err(format!("Passing {e:?} as a callable function, but it isn't"))
                }
            }
        },
        Rule::code_block => {
            let res = pair.into_inner()
                .map(|pair| pair_walk(pair));
            
                // TODO: Remove find
                if let Some(err) = res.clone().into_iter().find(|r| r.is_err()) {
                    err
                } else {
                    Ok(Expr::CodeBlock(res.map(|e| e.unwrap()).collect()))
                }
        },
    }
}

#[cfg(tests)]
mod tests {
    use std::fs;

    use super::parse;

    #[test]
    fn crude_parse() {
        let _ = dbg!(parse(format!("{{{}}}", fs::read_to_string("./tests/hello_world.vaca").unwrap())));
    }
}