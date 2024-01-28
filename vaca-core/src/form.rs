use std::{collections::LinkedList, fmt::Display};
use speedy::{Readable, Writable};

use crate::{Value, Symbol, value::{array::Array, function::Function, macros::Macro, valueref::ValueRef}, SymbolTable, ErrorStack};


#[derive(Debug, Clone, Readable, Writable)]
pub enum Form {
    AssingmentList(Vec<(Symbol, Form)>),
    Assingment(Symbol, Box<Form>),
    CodeBlock(Vec<Form>),
    Function(Vec<Symbol>, Box<Form>),
    Macro(Vec<Symbol>, Box<Form>),
    Call(Box<Form>, Vec<Form>),
    Array(Vec<Form>),
    Literal(Literal)
}

#[derive(Debug, Clone, Readable, Writable)]
pub enum Literal {
    Nil,
    Integer(i64),
    Float(f64),
    Char(char),
    String(String),
    Bool(bool),
    Symbol(Symbol)
}

impl Form {
    pub fn eval(&self, table: &mut SymbolTable) -> Result<ValueRef, ErrorStack> {
        match self {
            Form::AssingmentList(pairs) => {
                for (s, e) in pairs {
                    let v = e.eval(table)?;
                    table.register(s.clone(), v.take());
                }

                Ok(ValueRef::own(Value::Nil))
            },
            Form::Assingment(_, _) => {
                panic!("Shouldn't eval over a single assingment");
            }

            Form::CodeBlock(b) => { 
                table.create_scope();

                let mut res = b.iter()
                .map(|e| e.eval(table))
                .reduce(|acc, r| if acc.is_err() { acc } else { r })
                .unwrap_or(Ok(ValueRef::own(Value::Nil)));
            
                if let Ok(d) = res {
                    res = Ok(d.to_owned());
                }
                
                table.drop_scope();

                res
            },

            Form::Function(params, body) => Ok(
                ValueRef::own(
                    Value::Function(
                        Function::new(params.clone(), 
                                      (**body).clone())))
            ),
            Form::Macro(params, body) => Ok(
                ValueRef::own(
                    Value::Macro(
                        Macro::defined(params.clone(), 
                                      (**body).clone())))
            ),

            Form::Call(func, args) => {
                let func = func.eval(table);

                match func {
                    Err(err) => Err(ErrorStack::Stream { src: Some(self.to_string()), from: Box::new(err), note: Some("Error happened while trying to evaluate the callable part of the current call".into()) }),
                    Ok(func) => match func.as_ref() {
                        Value::Function(f) => {
                            let args = Form::Array(args.clone()).eval(table);

                            match args {
                                Err(err) => Err(ErrorStack::Stream { src: Some(self.to_string()), from: Box::new(err), note: Some("Error happened while evaluating an argument of the current call".into()) }),
                                Ok(args) => f.exec(args.to_array(), table)
                                    .map_err(|err| ErrorStack::Stream { src: Some(self.to_string()), from: Box::new(err), note: None })
                            }
                        },
                        Value::Macro(m) => {
                            let mut forms = LinkedList::new();
                            for arg in args.iter() {
                                forms.push_back(arg.clone())
                            }

                            m.exec(table, forms).map_err(|err| ErrorStack::Stream { src: Some(self.to_string()), from: Box::new(err), note: None })
                        }
                        d => if args.len() == 0 {
                            Ok(func)
                        } else {
                            Err(ErrorStack::Top { 
                                src: Some(self.to_string()), 
                                msg: format!("Trying call a non function value `{}`", d) 
                            })
                        }
                    },
                }
            },

            // Evaluate each expression and put back into an array
            Form::Array(a) => { 
                let res = a.iter()
                    .map(|e| e.eval(table))
                    .fold(Ok(Array::new()), |acc, item| match acc {
                        Err(e) => Err(e),
                        Ok(mut v) => match item {
                            Err(e) => Err(e),
                            Ok(d) => { v.push_back(d); Ok(v) },
                        }
                    });

                match res {
                    Err(err) => Err(ErrorStack::Stream { src: Some(self.to_string()), from: Box::new(err), note: Some("Error happened while evaluating an item of the current array".into()) }),
                    Ok(d) => Ok(ValueRef::own(Value::Array(d))),
                }
            },
            Form::Literal(l) => l.eval(table).map_err(|err| ErrorStack::Stream { src: Some(self.to_string()), from: Box::new(err), note: None }),
        }
    }

    pub fn replace_symbol(self, symbol: &Symbol, form: &Self) -> Self {
        match self {
            Form::AssingmentList(list) => Self::AssingmentList(list.into_iter()
                .map(|(s, f)| 
                    (s, f.replace_symbol(symbol, form)))
                .collect()),
            Form::Assingment(_, _) => unreachable!(),
            Form::CodeBlock(block) => Form::CodeBlock(block.into_iter()
                .map(move  |f| 
                    f.replace_symbol(symbol, form))
                .collect()),
            Form::Function(params, body) => {
                let body = if !params.contains(symbol) {
                    body.replace_symbol(symbol, form)
                } else { 
                    *body
                };
                Form::Function(params, Box::new(body))
            },
            Form::Macro(params, body) => {
                let body = if !params.contains(symbol) {
                    body.replace_symbol(symbol, form)
                } else { 
                    *body
                };
                Form::Macro(params, Box::new(body))
            },
            Form::Call(call, args) => Form::Call(
                Box::new(call.replace_symbol(symbol, form)), 
                args.into_iter().map(|f| f.replace_symbol(symbol, form)).collect()),
            Form::Array(array) => Form::Array(array.into_iter().map(|f| f.replace_symbol(symbol, form)).collect()),
            Form::Literal(l) => match l {
                Literal::Symbol(s) => if &s == symbol { form.clone() } else { Form::Literal(Literal::Symbol(s)) },
                l => Form::Literal(l)
            },
        }
    }
}

impl Literal {
    pub fn eval(&self, table: &mut SymbolTable) -> Result<ValueRef, ErrorStack> {
        let value = match self {
            Literal::Nil => ValueRef::own(Value::Nil),
            Literal::Integer(i) => ValueRef::own(Value::Integer(*i)),
            Literal::Float(f) => ValueRef::own(Value::Float(*f)),
            Literal::Char(c) => ValueRef::own(Value::Char(*c)),
            Literal::String(s) => ValueRef::own(Value::String(s.clone())),
            Literal::Bool(b) => ValueRef::own(Value::Bool(*b)),
            Literal::Symbol(s) => table.lookup(s)?//.map_err(|err| ErrorStack::Stream { src: Some(self.to_string()), from: Box::new(err), note: None })?,
        };

        Ok(value)
    }
}

impl Display for Form {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Form::AssingmentList(list) => {
                write!(f, "#( ")?;
                for (s, e) in list.iter() {
                    write!(f, "{} {} ", s, e)?
                }
                write!(f, ")")
            },
            Form::Assingment(_, _) => unreachable!(),
            Form::CodeBlock(block) => {
                writeln!(f, "{{")?;
                for e in block.iter() {
                    writeln!(f, "\t{}", e)?
                }
                writeln!(f, "}}")
            },
            Form::Function(args, body) => {
                write!(f, "<( ")?;
                
                for s in args.iter() {
                    write!(f, "{} ", s)?
                }

                if args.len() > 0 {
                    write!(f, "-> ")?;
                }

                write!(f, "{} )", body)
            },
            Form::Macro(args, body) => {
                write!(f, "[( ")?;
                
                for s in args.iter() {
                    write!(f, "{} ", s)?
                }

                if args.len() > 0 {
                    write!(f, "-> ")?;
                }

                write!(f, "{} )", body)
            },
            Form::Call(func, args) => {
                write!(f, "( {func} ")?;
                
                for arg in args.iter() {
                    write!(f, "{} ", arg)?;
                }

                write!(f, ")")
            },
            Form::Array(arr) => {
                write!(f, "[ ")?;
                for e in arr.iter() {
                    write!(f, "{} ", e)?
                }
                write!(f, "]")
            },
            Form::Literal(l) => write!(f, "{}", l),
        }
    }
}

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::Nil => "nil".to_string(),
            Self::Integer(i) => format!("{i}"),
            Self::Float(f) => format!("{f}"),
            Self::Char(c) => format!("'{c}'"),
            Self::String(s) => format!("\"{s}\""),
            Self::Bool(b) => b.to_string(),
            Self::Symbol(s) => s.to_string(),
        })
    }
}