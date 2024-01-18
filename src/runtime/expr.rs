use std::{rc::Weak, fmt::Display};
use speedy::{Readable, Writable};
use crate::{extract, runtime::{data::{Data, owner::Owner, function::Function, symbol_table::SymbolTable}, symbol::Symbol}};


#[derive(Debug, Clone, Readable, Writable)]
pub enum Expr {
    AssingmentList(Vec<(Symbol, Expr)>),
    Assingment(Symbol, Box<Expr>),
    CodeBlock(Vec<Expr>),
    Function(Vec<Symbol>, Box<Expr>),
    Call(Box<Expr>, Vec<Expr>),
    Array(Vec<Expr>),
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

impl Expr {
    pub fn eval(&self, owner: &mut Owner, table: &mut SymbolTable) -> Result<Weak<Data>, String> {
        match self {
            Expr::AssingmentList(pairs) => {
                for (s, e) in pairs {
                    let v = e.eval(owner, table)?;
                    table.insert(s.clone(), v);
                }

                Ok(owner.allocate(Data::Nil))
            },
            Expr::Assingment(_, _) => {
                //{ table.insert(symbol.clone(), expr.eval(owner, table)?); }
                panic!("Shouldn't eval over a single assingment");

                //Ok(owner.insert(Data::Nil))
            }

            Expr::CodeBlock(b) => { 
                owner.create_scope();
                table.create_scope();

                let mut res = b.iter()
                .map(|e| e.eval(owner, table))
                .reduce(|acc, r| if acc.is_err() { acc } else { r })
                .unwrap_or(Ok(owner.allocate(Data::Nil)));
            
                if let Ok(d) = res {
                    res = Ok(owner.allocate_return(d));
                }
                
                table.drop_scope();
                owner.drop_scope();

                res
            },

            Expr::Function(params, body) => Ok(
                owner.allocate(
                    Data::Function(
                        Function::new(params.clone(), 
                                      (**body).clone())))
            ),

            Expr::Call(func, args) => {
                let func = func.eval(owner, table);

                match func {
                    Err(e) => Err(e),
                    Ok(func) => match extract!(func).as_ref() {
                        Data::Function(f) => {
                            let args = Expr::Array(args.clone()).eval(owner, table);

                            match args {
                                Err(e) => Err(e),
                                Ok(args) => f.exec(extract!(args).as_vec(), owner, table)
                            }
                        },
                        Data::Macro(m) => {
                            m(owner, table, args)
                        }
                        d => Err(format!("Trying call over on functional value {}", d))
                    },
                }
            },

            // Evaluate each expression and put back into an array
            Expr::Array(a) => { 
                let res = a.iter()
                    .map(|e| e.eval(owner, table))
                    .fold(Ok(vec![]), |acc, e| match acc {
                        Err(e) => Err(e),
                        Ok(mut v) => match e {
                            Err(e) => Err(e),
                            Ok(d) => { v.push(d); Ok(v) },
                        }
                    });

                match res {
                    Err(e) => Err(e),
                    Ok(d) => Ok(owner.allocate(Data::Array(d))),
                }
            },
            Expr::Literal(l) => l.eval(owner, table),
        }
    }
}

impl Literal {
    pub fn eval(&self, owner: &mut Owner, table: &mut SymbolTable) -> Result<Weak<Data>, String> {
        let data = match self {
            Literal::Nil => owner.allocate(Data::Nil),
            Literal::Integer(i) => owner.allocate(Data::Integer(*i)),
            Literal::Float(f) => owner.allocate(Data::Float(*f)),
            Literal::Char(c) => owner.allocate(Data::Char(*c)),
            Literal::String(s) => owner.allocate(Data::String(s.clone())),
            Literal::Bool(b) => owner.allocate(Data::Bool(*b)),
            Literal::Symbol(s) => table.lookup(s)?,
        };

        Ok(data)
    }
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::AssingmentList(list) => {
                write!(f, "#( ")?;
                for (s, e) in list.iter() {
                    write!(f, "{} {} ", s, e)?
                }
                write!(f, ")")
            },
            Expr::Assingment(_, _) => unreachable!(),
            Expr::CodeBlock(block) => {
                writeln!(f, "{{")?;
                for e in block.iter() {
                    writeln!(f, "\t{}", e)?
                }
                writeln!(f, "}}")
            },
            Expr::Function(args, body) => {
                write!(f, "<( ")?;
                
                for s in args.iter() {
                    write!(f, "{} ", s)?
                }

                if args.len() > 0 {
                    write!(f, "-> ")?;
                }

                write!(f, "{} )", body)
            },
            Expr::Call(func, args) => {
                write!(f, "( {func} ")?;
                
                for arg in args.iter() {
                    write!(f, "{} ", arg)?;
                }

                write!(f, ")")
            },
            Expr::Array(arr) => {
                write!(f, "[ ")?;
                for e in arr.iter() {
                    write!(f, "{} ", e)?
                }
                write!(f, "]")
            },
            Expr::Literal(l) => write!(f, "{}", l),
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