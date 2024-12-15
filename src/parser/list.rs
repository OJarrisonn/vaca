use super::{form::Form, Parseable};

/// `stl.macro/List`
#[derive(Debug)]
pub struct List {
    pub forms: Vec<Form>,
}

impl Parseable for List {
    type Error = String;

    fn parse(value: edn_format::Value) -> Result<Self, Self::Error> {
        let edn_format::Value::List(list) = value else {
            return Err("Expected a list".to_string())
        };

        let forms = list.into_iter().map(Form::parse).collect::<Result<Vec<_>, _>>()?;

        Ok(List { forms })
    }

    fn accept(value: &edn_format::Value) -> bool {
        matches!(value, edn_format::Value::List(_))
    }
}