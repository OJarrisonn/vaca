#[macro_export]
/// Shortcut for creating a Symbol
macro_rules! sym {
    ($s:expr) => {
        Symbol::from($s)
    };
}

#[macro_export]
/// Shortcut for doing a lookup in a SymbolTable 
macro_rules! lookup {
    ($table:expr, $symbol:expr) => {
        $table.lookup(&Symbol::from($symbol))
    };
}

#[macro_export]
/// Shortcut for registering entries in the SymbolTable
macro_rules! register {
    ($table:expr, $symbol:expr, $value:expr) => {
        $table.register(
            sym!($symbol),
            ValueRef::Owned($value)
        )
    };
}

#[macro_export]
/// Shortcut for creating new native functions
macro_rules! function {
    ($f:expr $(,$args:expr)*) => {
        Value::Function(Function::native(vec![$($args.into(),)*], $f))
    };
}