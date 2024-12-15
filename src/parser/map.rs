use super::{form::Form, Parseable};

#[derive(Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd, Default)]
pub struct Map {
    pub entries: Vec<(Form, Form)>,
}

impl Parseable for Map {
    type Error = String;

    fn parse(value: edn_format::Value) -> Result<Self, Self::Error> {
        let edn_format::Value::Map(map) = value else {
            return Err("Expected a map".to_string())
        };

        let entries = map.into_iter().map(|(key, value)| {
            Ok::<_, Self::Error>((Form::parse(key)?, Form::parse(value)?))
        }).collect::<Result<Vec<_>, _>>()?;

        Ok(Map { entries })
    }

    fn accept(value: &edn_format::Value) -> bool {
        matches!(value, edn_format::Value::Map(_))
    }
}