use pest::{Parser, iterators::Pair};
use pest_derive::Parser;
use vaca_core::{Form, Symbol, form::Literal, ErrorStack};

#[derive(Parser)]
#[grammar = "./parser/grammar.pest"]
pub struct VacaParser;

pub fn parse_program(form: String) -> Result<Form, ErrorStack>{
    let res = VacaParser::parse(Rule::program, &form);

    match res {
        Ok(mut pairs) => pair_walk(pairs.next().unwrap()),
        Err(e) => Err(format!("{}", e).into()),
    }
}

pub fn parse_form(form: String) -> Result<Form, ErrorStack>{
    let res = VacaParser::parse(Rule::form, &form);

    match res {
        Ok(mut pairs) => pair_walk(pairs.next().unwrap()),
        Err(e) => Err(format!("{}", e).into()),
    }
}

fn pair_walk(pair: Pair<'_, Rule>) -> Result<Form, ErrorStack>{
    let src = Some(format!("[L {} C {}]: {}", pair.line_col().0, pair.line_col().1, pair.as_span().as_str()));
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
        Rule::symbol => Ok(Form::Literal(Literal::Symbol(Symbol::from(pair.as_str())))),
        Rule::float => Ok(Form::Literal(Literal::Float(pair.as_str().parse().unwrap()))),
        Rule::integer => Ok(Form::Literal(Literal::Integer(pair.as_str().parse().unwrap()))),
        
        Rule::string_content => Ok(Form::Literal(Literal::String(pair.as_str().to_string().replace("\\n", "\n").replace("\\r", "\r")))),

        Rule::char_content => Ok(Form::Literal(Literal::Char(pair.as_str().chars().next().unwrap_or('\0')))),
        Rule::bool => Ok(Form::Literal(Literal::Bool(pair.as_str() == "true"))),
        Rule::nil => Ok(Form::Literal(Literal::Nil)),
        // Collection rules
        Rule::assingment_list => {
            let res = pair.into_inner()
                .map(|pair| pair_walk(pair))
                .fold(Ok(vec![]), |acc, a| {
                    match acc {
                        Ok(mut acc) => match a {
                            Ok(Form::Assingment(symbol, form)) => {
                                acc.push((symbol, *form));
                                Ok(acc)
                            },
                            Ok(_) => panic!("This must be an assignment"),
                            Err(e) => Err(e)
                        },
                        e => e
                    }
                });

            match res {
                Ok(list) => Ok(Form::AssingmentList(list)),
                Err(e) => Err(e),
            }
        },
        Rule::export_list => todo!(),
        Rule::assingment => {
            let mut iter = pair.into_inner();
            let symbol: Symbol = iter.next().unwrap().as_str().into();
            let form = pair_walk(iter.next().unwrap());

            match form {
                Err(e) => Err(e),
                Ok(form) => Ok(Form::Assingment(symbol, Box::new(form))),
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
                Ok(list) => Ok(Form::Array(list)),
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

            let body = Form::CodeBlock(body);

            Ok(Form::Function(params, Box::new(body)))
        },
        Rule::call => {
            let res: Result<Vec<Form>, ErrorStack> = pair.clone().into_inner()
                .map(|pair| pair_walk(pair))
                .collect();

            match res {
                Err(e) => Err(ErrorStack::Stream { src, from: Box::new(e), note: Some("During parsing of a `Form` in a `Call`".into()) }),
                Ok(vec) => {
                    let mut iter = vec.into_iter();

                    match iter.next() {
                        Some(func) => match func {
                            Form::Function(_, _) | Form::CodeBlock(_) | Form::Literal(Literal::Symbol(_)) | Form::Call(_, _) => Ok(Form::Call(Box::new(func), iter.collect())),
                            e => Err(ErrorStack::Top{ src, msg: format!("Passing {e:?} as a callable, but it isn't") })
                        },
                        None => Err(ErrorStack::Top { src, msg: "Empty call not allowed".into() }),
                    }
                }
            }
        },
        Rule::code_block | Rule::program => {
            let res: Result<Vec<Form>, ErrorStack> = pair.into_inner()
                .map(|pair| pair_walk(pair))
                .collect();
            
            res.map(|ok| Form::CodeBlock(ok)).map_err(|err| ErrorStack::Stream { src: src, from: Box::new(err), note: Some("During parsing of a `Form` of a `Block`".into()) })
        },
        Rule::lib => Err(ErrorStack::Top { src, msg: "Libraries aren't implemented yet".into() }),
        Rule::dontcare => Err(ErrorStack::Top { src, msg: "Don't Cares aren't implemented yet".into() }),
        Rule::macrodef => todo!(),
        Rule::vacaimport => todo!(),
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