use edn_format::{Symbol, Value};

use super::Parseable;

pub struct Module {
    pub name: Symbol,
    pub imports: Vec<Symbol>,
    pub functions: Vec<Symbol>,
}

impl Module {
    pub fn symbol() -> Symbol {
        Symbol::from_name("defmod")
    }
}

impl Parseable for Module {
    type Error = String;

    fn parse(value: edn_format::Value) -> Result<Self, Self::Error> {
        let Value::List(list) = value else {
            return Err("Expected a list".to_string());
        };

        let mut list = list.into_iter().skip(1);

        let name = match list.next() {
            Some(Value::Symbol(symbol)) => symbol,
            _ => return Err("Expected a symbol".to_string()),
        };

        let imports = Vec::new();
        let functions = Vec::new();

        Ok(Module {
            name,
            imports,
            functions,
        })
    }

    fn accept(value: &edn_format::Value) -> bool {
        if let Value::List(list) = value {
            if let Some(Value::Symbol(symbol)) = list.first() {
                return symbol == &Self::symbol();
            }
        } 
        false
    }
}
