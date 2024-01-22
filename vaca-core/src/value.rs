use std::{fmt::Display, iter::zip};

use self::{function::Function, macros::NativeMacro, array::Array};

pub mod function;
pub mod macros;
pub mod valueref;
pub mod result;
pub mod array;

#[derive(Debug, Clone)]
pub enum Value {
    Nil,
    NotANumber,
    Bool(bool),
    Integer(i64),
    Float(f64),
    Char(char),
    String(String),
    Array(Array),
    Function(Function),
    Macro(NativeMacro)
}

impl Into<Array> for Value {
    fn into(self) -> Array {
        match self {
            Self::Array(a) => a,
            d => panic!("Can't turn a {} into an array like", d)
        }
    }
}

impl Value {
    /// Copies the Value and unwraps an array
    pub fn to_array(&self) -> Array {
        match self {
            Self::Array(a) => a.clone(),
            d => panic!("Can't turn a {} into an array like", d)
        }
    }

    pub fn as_boolean(&self) -> bool {
        match self {
            Value::Nil => false,
            Value::NotANumber => false,
            Value::Bool(b) => *b,
            Value::Integer(i) => *i != 0,
            Value::Float(f) => *f != 0.,
            Value::Char(c) => *c != '\0',
            Value::String(s) => s.len() != 0,
            Value::Array(a) => a.len() != 0,
            Value::Function(_) => false,
            Value::Macro(_) => false,
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::Nil => format!("'nil"),
            Self::NotANumber => format!("'nan"),
            Self::Bool(b) => format!("{b}"),
            Self::Char(c) => format!("{c}"),
            Self::Integer(i) => format!("{i}"),
            Self::Float(f) => format!("{f}"),
            Self::Array(a) => format!("[ {}]", a.iter()
                .map(|w| format!("{} ", w))
                .reduce(|acc, f| format!("{acc}{f}"))
                .unwrap_or(String::from(""))
            ),
            Self::Function(f) => format!("'func\\{}", f.arity()),
            Self::Macro(_) => format!("'macro"),
            Self::String(s) => s.clone()
        })
    }
}


impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Self::Integer(l0), Self::Integer(r0)) => l0.partial_cmp(r0),
            (Self::Integer(l0), Self::Float(r0)) => (*l0 as f64).partial_cmp(r0),
            (Self::Float(l0), Self::Float(r0)) => l0.partial_cmp(r0),
            (Self::Float(l0), Self::Integer(r0)) => l0.partial_cmp(&(*r0 as f64)),
            (Self::Char(l0), Self::Char(r0)) => l0.partial_cmp(r0),
            (Self::String(l0), Self::String(r0)) => l0.partial_cmp(r0),
            _ => None
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Nil, Self::Nil) => true,
            (Self::NotANumber, Self::NotANumber) => true,
            (Self::Bool(l0), Self::Bool(r0)) => l0 == r0,
            (Self::Integer(l0), Self::Integer(r0)) => l0 == r0,
            (Self::Integer(l0), Self::Float(r0)) => *l0 as f64 == *r0,
            (Self::Float(l0), Self::Float(r0)) => l0 == r0,
            (Self::Float(l0), Self::Integer(r0)) => *l0 == *r0 as f64,
            (Self::Char(l0), Self::Char(r0)) => l0 == r0,
            (Self::String(l0), Self::String(r0)) => l0 == r0,
            (Self::Array(l0), Self::Array(r0)) => {
                if l0.len() != r0.len() {
                    false
                } else {
                    zip(l0, r0).all(|(l, r)| l == r)
                }
            },
            _ => false//core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}