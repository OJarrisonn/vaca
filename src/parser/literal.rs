use ordered_float::OrderedFloat;

use super::Parseable;

/// `stl.macro/Literal`
#[derive(Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum Literal {
    String(StringLiteral),
    Char(CharLiteral),
    Bool(BoolLiteral),
    Int(IntLiteral),
    Float(FloatLiteral),
    Nil(NilLiteral),
}

impl Default for Literal {
    fn default() -> Self {
        Literal::Nil(NilLiteral)
    }
}

impl Parseable for Literal {
    type Error = String;

    fn parse(value: edn_format::Value) -> Result<Self, Self::Error> {
        if StringLiteral::accept(&value) {
            Ok(Literal::String(StringLiteral::parse(value)?))
        } else if CharLiteral::accept(&value) {
            Ok(Literal::Char(CharLiteral::parse(value)?))
        } else if BoolLiteral::accept(&value) {
            Ok(Literal::Bool(BoolLiteral::parse(value)?))
        } else if IntLiteral::accept(&value) {
            Ok(Literal::Int(IntLiteral::parse(value)?))
        } else if FloatLiteral::accept(&value) {
            Ok(Literal::Float(FloatLiteral::parse(value)?))
        } else if NilLiteral::accept(&value) {
            Ok(Literal::Nil(NilLiteral::parse(value)?))
        } else {
            Err("Expected a literal".to_string())
        }
    }

    fn accept(value: &edn_format::Value) -> bool {
        StringLiteral::accept(value) || CharLiteral::accept(value) || BoolLiteral::accept(value) || IntLiteral::accept(value) || FloatLiteral::accept(value) || NilLiteral::accept(value)
    }
}

/// `stl.macro/StringLiteral`
#[derive(Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd, Default)]
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
#[derive(Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd, Default)]
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
#[derive(Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd, Default)]
pub struct BoolLiteral {
    pub value: bool,
}

impl Parseable for BoolLiteral {
    type Error = String;

    fn parse(value: edn_format::Value) -> Result<Self, Self::Error> {
        let edn_format::Value::Boolean(value) = value else {
            return Err("Expected a boolean".to_string())
        }; 

        Ok(BoolLiteral { value })
    }

    fn accept(value: &edn_format::Value) -> bool {
        matches!(value, edn_format::Value::Boolean(_))
    }
}

/// `stl.macro/IntLiteral`
#[derive(Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd, Default)]
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
#[derive(Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd, Default)]
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
#[derive(Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd, Default)]
pub struct NilLiteral;

impl Parseable for NilLiteral {
    type Error = String;

    fn parse(value: edn_format::Value) -> Result<Self, Self::Error> {
        if let edn_format::Value::Nil = value {
            Ok(NilLiteral)
        } else {
            Err("Expected nil".to_string())
        }
    }

    fn accept(value: &edn_format::Value) -> bool {
        matches!(value, edn_format::Value::Nil)
    }
}

