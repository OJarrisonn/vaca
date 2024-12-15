use super::Parseable;


/// `stl.macro/Keyword`
#[derive(Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd, Default)]
pub struct Keyword {
    pub namespace: Option<String>,
    pub name: String,
}

impl From<edn_format::Keyword> for Keyword {
    fn from(keyword: edn_format::Keyword) -> Self {
        Keyword {
            namespace: keyword.namespace().map(|s| s.into()),
            name: keyword.name().into(),
        }
    }
}

impl Parseable for Keyword {
    type Error = String;

    fn parse(value: edn_format::Value) -> Result<Self, Self::Error> {
        let keyword = match value {
            edn_format::Value::Keyword(keyword) => keyword,
            _ => return Err("Expected a keyword".to_string()),
        };

        Ok(Keyword::from(keyword))
    }

    fn accept(value: &edn_format::Value) -> bool {
        matches!(value, edn_format::Value::Keyword(_))
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::Parseable;

    #[test]
    fn test_parse() {
        use super::Keyword;

        let edn_format::Value::Keyword(keyword) = edn_format::parse_str(":foo").unwrap() else {
            panic!("Expected a keyword");
        };

        let value = edn_format::Value::Keyword(keyword);
        
        let keyword = Keyword::parse(value).unwrap();

        assert_eq!(keyword.namespace, None);
        assert_eq!(keyword.name, "foo");
    }
}