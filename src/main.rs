mod stl;
mod parser;
mod runtime;
#[macro_use]
mod macros;

pub use crate::runtime::{data::{Data, owner::Owner, function::Function, symbol_table::SymbolTable}, expr::{Expr, Literal}, symbol::Symbol};

fn main() {
    let mut owner = Owner::new();
    let mut table = SymbolTable::new();

    owner.create_scope();
    table.create_scope();

    register!(owner, table, "pi", Data::Float(3.1415926));
    register!(owner, table, "+", function!(stl::math::sum, "a", "b"));
    register!(owner, table, "-", function!(stl::math::sub, "a", "b"));
    register!(owner, table, "*", function!(stl::math::mul, "a", "b"));
    register!(owner, table, "/", function!(stl::math::div, "a", "b"));

    let call = Expr::Call(Box::new(Expr::Literal(Literal::Symbol(symbol!("=")))), vec![Expr::Literal(Literal::Integer(1)), Expr::Literal(Literal::Symbol(symbol!("pi")))]);

    dbg!(call.eval(&mut owner, &mut table).unwrap().upgrade());

    dbg!(table.lookup(&symbol!("pi")).unwrap().upgrade());

    table.drop_scope();
    owner.drop_scope();
}
