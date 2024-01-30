use std::rc::Rc;

use super::value::Value;

/// ValueRef is the type that Vaca uses to share memory and values across the program using ownership
/// Values can be owned in the scope or owned by the SymbolTable, which returns ValueRef as a *const Value
pub type ValueRef = Rc<Value>;