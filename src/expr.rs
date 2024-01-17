use std::rc::Weak;

use crate::{symbol::Symbol, Data, Owner, SymbolTable, Function};

#[derive(Debug, Clone)]
pub enum Expr {
    Assingment(Vec<(Symbol, Expr)>),
    CodeBlock(Vec<Expr>),
    Function(Vec<Symbol>, Box<Expr>),
    Call(Box<Expr>, Vec<Expr>),
    Array(Vec<Expr>),
    Literal(Literal)
}

#[derive(Debug, Clone)]
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
            Expr::Assingment(pairs) => {
                for (s, e) in pairs {
                    let v = e.eval(owner, table)?;
                    table.insert(s.clone(), v);
                }

                Ok(owner.insert(Data::Nil))
            },

            Expr::CodeBlock(b) => { 
                owner.create_scope();
                table.create_scope();

                let mut res = b.iter()
                .map(|e| e.eval(owner, table))
                .reduce(|acc, r| if acc.is_err() { acc } else { r })
                .unwrap_or(Ok(owner.insert(Data::Nil)));
            
                if let Ok(d) = res {
                    res = Ok(owner.insert_return(d));
                }
                
                table.drop_scope();
                owner.drop_scope();

                res
            },

            Expr::Function(params, body) => Ok(
                owner.insert(
                    Data::Function(
                        Function::new(params.clone(), 
                                      (**body).clone())))
            ),

            Expr::Call(func, args) => {
                let func = func.eval(owner, table);

                match func {
                    Err(e) => Err(e),
                    Ok(func) => match func.upgrade().unwrap().as_ref() {
                        Data::Function(f) => {
                            let args = Expr::Array(args.clone()).eval(owner, table);

                            match args {
                                Err(e) => Err(e),
                                Ok(args) => f.exec(args.upgrade().unwrap().as_ref().as_vec(), owner, table)
                            }
                        },
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
                    Ok(d) => Ok(owner.insert(Data::Array(d))),
                }
            },
            Expr::Literal(l) => l.eval(owner, table),
        }
    }
}

impl Literal {
    pub fn eval(&self, owner: &mut Owner, table: &mut SymbolTable) -> Result<Weak<Data>, String> {
        let data = match self {
            Literal::Nil => owner.insert(Data::Nil),
            Literal::Integer(i) => owner.insert(Data::Integer(*i)),
            Literal::Float(f) => owner.insert(Data::Float(*f)),
            Literal::Char(c) => owner.insert(Data::Char(*c)),
            Literal::String(s) => owner.insert(Data::String(s.clone())),
            Literal::Bool(b) => owner.insert(Data::Bool(*b)),
            Literal::Symbol(s) => {
                let v = table.lookup(s);
                if let None = v {
                    return Err(format!("Undefined symbol {}", s));
                } else {
                    v.unwrap()
                }
            },
        };

        Ok(data)
    }
}