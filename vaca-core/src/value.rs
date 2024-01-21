use std::{fmt::Display, iter::zip};

use self::{function::Function, macros::NativeMacro};

pub mod function;
pub mod macros;

#[derive(Debug, Clone)]
pub enum ValueRef {
    Owned(Value),
    Pointed(*const Value)
}

#[derive(Debug, Clone)]
pub enum Value {
    Nil,
    Bool(bool),
    Integer(i64),
    Float(f64),
    Char(char),
    String(String),
    Array(Vec<ValueRef>),
    Function(Function),
    Macro(NativeMacro)
}

impl Into<Vec<ValueRef>> for Value {
    fn into(self) -> Vec<ValueRef> {
        match self {
            Self::Array(a) => a,
            d => panic!("Can't turn a {} into an array like", d)
        }
    }
}

impl ValueRef {
    pub fn unwrap(self) -> Value {
        match self {
            ValueRef::Owned(v) => v,
            ValueRef::Pointed(v) => unsafe { std::ptr::read(v) },
        }
    }
}

impl Value {
    pub fn as_vec(&self) -> Vec<ValueRef> {
        match self {
            Self::Array(a) => a.clone(),
            d => panic!("Can't turn a {} into an array like", d)
        }
    }

    pub fn as_boolean(&self) -> bool {
        match self {
            Value::Nil => false,
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

impl Display for ValueRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res = match &self {
            Self::Owned(v) => v,
            Self::Pointed(v) => unsafe { std::ptr::read(v) }
        };

        write!(f, "{}", &unsafe { std::ptr::read(res) })
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::Nil => format!("'nil"),
            Self::Bool(b) => format!("{b}"),
            Self::Char(c) => format!("{c}"),
            Self::Integer(i) => format!("{i}"),
            Self::Float(f) => format!("{f}"),
            Self::Array(a) => format!("[ {}]", a.iter()
                .map(|w| format!("{} ", w))
                .reduce(|acc, f| format!("{acc}{f}"))
                .unwrap_or(String::from(""))
            ),
            Self::Function(f) => format!("'function\\{}", f.arity()),
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

impl PartialEq for ValueRef {
    fn eq(&self, other: &Self) -> bool {
        self.clone().unwrap() == other.clone().unwrap()
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Nil, Self::Nil) => true,
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