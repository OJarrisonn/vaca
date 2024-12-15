use super::Parseable;

/// `stl.macro/Symbol`
#[derive(Debug)]
pub struct Symbol {
    pub namespace: Option<String>,
    pub name: String,
}

impl From<edn_format::Symbol> for Symbol {
    fn from(symbol: edn_format::Symbol) -> Self {
        Symbol {
            namespace: symbol.namespace().map(|s| s.into()),
            name: symbol.name().into(),
        }
    }
}

impl Parseable for Symbol {
    type Error = String;

    fn parse(value: edn_format::Value) -> Result<Self, Self::Error> {
        let symbol = match value {
            edn_format::Value::Symbol(symbol) => symbol,
            _ => return Err("Expected a symbol".to_string()),
        };

        Ok(Symbol::from(symbol))
    }

    fn accept(value: &edn_format::Value) -> bool {
        matches!(value, edn_format::Value::Symbol(_))
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::Parseable;

    #[test]
    fn test_parse() {
        use super::Symbol;

        let edn_format::Value::Symbol(symbol) = edn_format::parse_str("foo").unwrap() else {
            panic!("Expected a symbol");
        };

        let value = edn_format::Value::Symbol(symbol);
        
        let symbol = Symbol::parse(value).unwrap();

        assert_eq!(symbol.namespace, None);
        assert_eq!(symbol.name, "foo");
    }
}