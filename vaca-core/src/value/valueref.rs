use std::ops::Deref;

use crate::Value;

#[derive(Debug)]
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
    pub fn point(other: &Self) -> Self {
        let ptr = match other {
            ValueRef::Owned(own) => own as *const Value,
            ValueRef::Pointed(ptr) => *ptr,
        };

        Self::Pointed(ptr)
    }

    pub fn take(self) -> Value {
        match self {
            ValueRef::Owned(own) => own,
            ValueRef::Pointed(ptr) => unsafe {
                (*ptr).clone()
            },
        }
    }

    pub fn is_valid(&self) -> bool {
        match self {
            ValueRef::Owned(_) => true,
            ValueRef::Pointed(v) => !v.is_null(),
        }
    }

    pub fn is_owned(&self) -> bool {
        if let Self::Owned(_) = &self {
            true
        } else {
            false
        }
    }

    pub fn is_pointed(&self) -> bool {
        !self.is_owned()
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

impl Clone for ValueRef {
    fn clone(&self) -> Self {
        match self {
            Self::Owned(arg0) => Self::Owned(arg0.clone()),
            Self::Pointed(arg0) => unsafe { Self::Owned((**arg0).clone()) },
        }
    }
}

impl AsRef<Value> for ValueRef {
    fn as_ref(&self) -> &Value {
        match self {
            ValueRef::Owned(own) => own,
            ValueRef::Pointed(ptr) => unsafe {
                ptr.as_ref().unwrap()
            },
        }
    }
}