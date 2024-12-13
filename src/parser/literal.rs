use ordered_float::OrderedFloat;

use super::Parseable;

/// `stl.macro/StringLiteral`
pub struct StringLiteral {
    pub value: String,
}

impl Parseable for StringLiteral {
    type Error = String;

    fn parse(value: edn_format::Value) -> Result<Self, Self::Error> {
        let edn_format::Value::String(string) = value else {
            return Err("Expected a string".to_string())
        };

        Ok(StringLiteral { value: string })
    }

    fn accept(value: &edn_format::Value) -> bool {
        matches!(value, edn_format::Value::String(_))
    }
}

/// `stl.macro/CharLiteral`
pub struct CharLiteral {
    pub value: char,
}

impl Parseable for CharLiteral {
    type Error = String;

    fn parse(value: edn_format::Value) -> Result<Self, Self::Error> {
        let edn_format::Value::Character(value) = value else {
            return Err("Expected a character".to_string())
        };

        Ok(CharLiteral { value })
    }

    fn accept(value: &edn_format::Value) -> bool {
        matches!(value, edn_format::Value::String(_))
    }
}

/// `stl.macro/BoolLiteral`
pub struct BoolLiteral {
    pub value: bool,
}

impl Parseable for BoolLiteral {
    type Error = String;

    fn parse(value: edn_format::Value) -> Result<Self, Self::Error> {
        let value = match value {
            edn_format::Value::Boolean(value) => value,
            edn_format::Value::Keyword(keyword) => if keyword.namespace().is_none() {
                match keyword.name() {
                    "true" => true,
                    "false" => false,
                    _ => return Err("Expected a boolean".to_string()),
                }
            } else {
                return Err("Expected a boolean".to_string());
            },
            _ => return Err("Expected a boolean".to_string()),
        };

        Ok(BoolLiteral { value })
    }

    fn accept(value: &edn_format::Value) -> bool {
        if let edn_format::Value::Keyword(keyword) = value {
            if keyword.namespace().is_none() {
                return keyword.name() == "true" || keyword.name() == "false";
            }
        }
        
        matches!(value, edn_format::Value::Boolean(_))
    }
}

/// `stl.macro/IntLiteral`
pub struct IntLiteral {
    pub value: i64,
}

impl Parseable for IntLiteral {
    type Error = String;

    fn parse(value: edn_format::Value) -> Result<Self, Self::Error> {
        let edn_format::Value::Integer(value) = value else {
            return Err("Expected an integer".to_string())
        };

        Ok(IntLiteral { value })
    }

    fn accept(value: &edn_format::Value) -> bool {
        matches!(value, edn_format::Value::Integer(_))
    }
}

/// `stl.macro/FloatLiteral`
pub struct FloatLiteral {
    pub value: OrderedFloat<f64>,
}

impl Parseable for FloatLiteral {
    type Error = String;

    fn parse(value: edn_format::Value) -> Result<Self, Self::Error> {
        let edn_format::Value::Float(value) = value else {
            return Err("Expected a float".to_string())
        };

        Ok(FloatLiteral { value })
    }

    fn accept(value: &edn_format::Value) -> bool {
        matches!(value, edn_format::Value::Float(_))
    }
}

/// `stl.macro/NilLiteral`
pub struct NilLiteral;

impl Parseable for NilLiteral {
    type Error = String;

    fn parse(value: edn_format::Value) -> Result<Self, Self::Error> {
        if let edn_format::Value::Nil = value {
            Ok(NilLiteral)
        } else if let edn_format::Value::Keyword(keyword) = value {
            if keyword.namespace().is_none() && keyword.name() == "nil" {
                Ok(NilLiteral)
            } else {
                Err("Expected nil".to_string())
            }
        } else {
            Err("Expected nil".to_string())
        }
    }

    fn accept(value: &edn_format::Value) -> bool {
        matches!(value, edn_format::Value::Nil)
    }
}

