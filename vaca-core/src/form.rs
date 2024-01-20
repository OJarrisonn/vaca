use std::{rc::Rc, fmt::Display};
use speedy::{Readable, Writable};

use crate::{Value, Symbol, value::function::Function, SymbolTable};


#[derive(Debug, Clone, Readable, Writable)]
pub enum Form {
    AssingmentList(Vec<(Symbol, Form)>),
    Assingment(Symbol, Box<Form>),
    CodeBlock(Vec<Form>),
    Function(Vec<Symbol>, Box<Form>),
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
    pub fn eval(&self, table: &mut SymbolTable) -> Result<Rc<Value>, String> {
        match self {
            Form::AssingmentList(pairs) => {
                for (s, e) in pairs {
                    let v = e.eval(table)?;
                    table.register(s.clone(), v);
                }

                Ok(Rc::new(Value::Nil))
            },
            Form::Assingment(_, _) => {
                //{ table.insert(symbol.clone(), expr.eval(table)?); }
                panic!("Shouldn't eval over a single assingment");

                //Ok(owner.insert(Value::Nil))
            }

            Form::CodeBlock(b) => { 
                table.create_scope();

                let mut res = b.iter()
                .map(|e| e.eval(table))
                .reduce(|acc, r| if acc.is_err() { acc } else { r })
                .unwrap_or(Ok(Rc::new(Value::Nil)));
            
                if let Ok(d) = res {
                    res = Ok(d);
                }
                
                table.drop_scope();

                res
            },

            Form::Function(params, body) => Ok(
                Rc::new(
                    Value::Function(
                        Function::new(params.clone(), 
                                      (**body).clone())))
            ),

            Form::Call(func, args) => {
                let func = func.eval(table);

                match func {
                    Err(e) => Err(e),
                    Ok(func) => match func.as_ref() {
                        Value::Function(f) => {
                            let args = Form::Array(args.clone()).eval(table);

                            match args {
                                Err(e) => Err(e),
                                Ok(args) => f.exec(args.as_vec(), table)
                            }
                        },
                        Value::Macro(m) => {
                            m(table, args)
                        }
                        d => Err(format!("Trying call over on functional value {}", d))
                    },
                }
            },

            // Evaluate each expression and put back into an array
            Form::Array(a) => { 
                let res = a.iter()
                    .map(|e| e.eval(table))
                    .fold(Ok(vec![]), |acc, e| match acc {
                        Err(e) => Err(e),
                        Ok(mut v) => match e {
                            Err(e) => Err(e),
                            Ok(d) => { v.push(d); Ok(v) },
                        }
                    });

                match res {
                    Err(e) => Err(e),
                    Ok(d) => Ok(Rc::new(Value::Array(d))),
                }
            },
            Form::Literal(l) => l.eval(table),
        }
    }
}

impl Literal {
    pub fn eval(&self, table: &mut SymbolTable) -> Result<Rc<Value>, String> {
        let value = match self {
            Literal::Nil => Rc::new(Value::Nil),
            Literal::Integer(i) => Rc::new(Value::Integer(*i)),
            Literal::Float(f) => Rc::new(Value::Float(*f)),
            Literal::Char(c) => Rc::new(Value::Char(*c)),
            Literal::String(s) => Rc::new(Value::String(s.clone())),
            Literal::Bool(b) => Rc::new(Value::Bool(*b)),
            Literal::Symbol(s) => match table.lookup(s) {
                Some(v) => v,
                None => return Err(format!("Tried to evaluate undefined symbol {s}"))
            },
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
            Self::Char(c) => c.to_string(),
            Self::String(s) => s.to_string(),
            Self::Bool(b) => b.to_string(),
            Self::Symbol(s) => s.to_string(),
        })
    }
}