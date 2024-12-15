use edn_format::Value;

use super::{array::Array, keyword::Keyword, list::List, literal::Literal, symbol::Symbol, Parseable};

#[derive(Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum Form {
    Literal(Literal),
    Keyword(Keyword),
    List(List),
    Array(Array),
    Symbol(Symbol),
}

impl Default for Form {
    fn default() -> Self {
        Form::List(List::default())
    }
}

impl Parseable for Form {
    type Error = String;

    fn parse(value: Value) -> Result<Self, Self::Error> {
        if Literal::accept(&value) {
            Ok(Form::Literal(Literal::parse(value)?))
        } else if Keyword::accept(&value) {
            Ok(Form::Keyword(Keyword::parse(value)?))
        } else if List::accept(&value) {
            Ok(Form::List(List::parse(value)?))
        } else if Array::accept(&value) {
            Ok(Form::Array(Array::parse(value)?))
        } else if Symbol::accept(&value) {
            Ok(Form::Symbol(Symbol::parse(value)?))
        } else {
            Err("Expected a form".to_string())
        }
    }

    fn accept(value: &Value) -> bool {
        Literal::accept(value) || Keyword::accept(value) || List::accept(value) || Array::accept(value) || Symbol::accept(value)
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::Parseable;

    #[test]
    fn test_parse() {
        use super::Form;

        let value = edn_format::parse_str(include_str!("../samples/hello_world.vaca")).unwrap();

        let form = Form::parse(value).unwrap();

        dbg!(&form);
    }
}