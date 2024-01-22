use std::ops::Deref;

use crate::Value;

/// ValueRef is the type that Vaca uses to share memory and values across the program using ownership
/// Values can be owned in the scope or owned by the SymbolTable, which returns ValueRef as a *const Value
pub enum ValueRef {
    Owned(Value),
    Pointed(*const Value)
}

impl ValueRef {
    /// Takes the ownership of a Value
    pub fn own(other: Value) -> Self {
        Self::Owned(other)
    }

    /// Takes the reference, converts it to a *const Value and stores it
    pub fn point(other: &Value) -> Self {
        Self::Pointed(other as *const Value)
    }

    pub fn is_valid(&self) -> bool {
        match self {
            ValueRef::Owned(_) => true,
            ValueRef::Pointed(v) => !v.is_null(),
        }
    }
}

impl Deref for ValueRef {
    type Target = Value;

    /// Uses unsafe to access a value that is a *const Value
    fn deref(&self) -> &Self::Target {
        match self {
            ValueRef::Owned(v) => v,
            ValueRef::Pointed(v) => unsafe { v.as_ref().unwrap() },
        }
    }
}