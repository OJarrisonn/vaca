mod data;
mod symbol;
mod expr;

use std::rc::Weak;

pub use crate::{data::{Data, owner::Owner, owner::OwnScope, function::Function, symbol_table::SymbolScope, symbol_table::SymbolTable}, expr::{Expr, Literal}, symbol::Symbol};

fn sum(owner: &mut Owner, table: &mut SymbolTable) -> Result<Weak<Data>, String> {
    let a = table.lookup(&Symbol::from("a")).unwrap().upgrade().unwrap();
    let b = table.lookup(&Symbol::from("b")).unwrap().upgrade().unwrap();

    let sum = match (a.as_ref(), b.as_ref()) {
        (Data::Integer(a), Data::Integer(b)) => Data::Integer(a + b),
        (Data::Float(a), Data::Integer(b)) => Data::Float(a + *b as f64),
        (Data::Integer(a), Data::Float(b)) => Data::Float(*a as f64 + b),
        (Data::Float(a), Data::Float(b)) => Data::Float(a + b),
        (Data::Integer(_), _) | (Data::Float(_), _) => return Err(format!("Argument `b` should be a numeric value not {b}")),
        (_, Data::Integer(_)) | (_, Data::Float(_)) => return Err(format!("Argument `a` should be a numeric value not {a}")),
        (_, _) => return Err(format!("Arguments for `a` and `b` should be numeric values not {a} and {b}"))
    };

    Ok(owner.insert(sum))
}

fn main() {
    let mut owner = Owner::new();
    let mut table = SymbolTable::new();

    owner.create_scope();
    table.create_scope();

    let pi = (Symbol::from("PI"), owner.insert(Data::Float(3.1415)));
    table.insert(pi.0.clone(), pi.1);

    let sum = (
        Symbol::from("+"), 
        owner.insert(Data::Function(
            Function::native(vec![Symbol::from("a"), Symbol::from("b")], sum)
        ))
    );

    table.insert(sum.0.clone(), sum.1);

    let call = Expr::Call(Box::new(Expr::Literal(Literal::Symbol(sum.0))), vec![Expr::Literal(Literal::Integer(1)), Expr::Literal(Literal::Symbol(pi.0.clone()))]);

    dbg!(call.eval(&mut owner, &mut table).unwrap().upgrade());

    dbg!(table.lookup(&pi.0).unwrap().upgrade());

    table.drop_scope();
    owner.drop_scope();
}
