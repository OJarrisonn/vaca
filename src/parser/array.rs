use edn_format::Value;

use super::{form::Form, Parseable};

/// `stl.macro/Array`
#[derive(Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd, Default)]
pub struct Array {
    pub forms: Vec<Form>,
}

impl Parseable for Array {
    type Error = String;

    fn parse(value: Value) -> Result<Self, Self::Error> {
        if let Value::Vector(values) = value {
            let forms = values.into_iter().map(Form::parse).collect::<Result<Vec<_>, _>>()?;

            Ok(Array { forms })
        } else {
            Err("Expected an array".to_string())
        }
    }

    fn accept(value: &Value) -> bool {
        matches!(value, Value::Vector(_))
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::Parseable;

    #[test]
    fn test_parse() {
        use super::Array;

        let value = edn_format::parse_str("[1 2 3]").unwrap();

        let array = Array::parse(value).unwrap();

        assert!(array.forms.len() == 3);
    }
}