#[macro_export]
macro_rules! symbol {
    ($s:expr) => {
        Symbol::from($s)
    };
}

#[macro_export]
/// Receives a Weak<Data> and returns a &Data
macro_rules! extract {
    ($weak:expr) => {
        ($weak).upgrade().unwrap()
    };
}

#[macro_export]
macro_rules! lookup {
    ($table:expr, $symbol:expr) => {
        extract!($table.lookup(&Symbol::from($symbol))?)
    };
}

#[macro_export]
macro_rules! register {
    ($owner:expr, $table:expr, $symbol:expr, $data:expr) => {
        $table.insert(
            symbol!($symbol),
            $owner.insert($data)
        )
    };
}

#[macro_export]
macro_rules! function {
    ($f:expr $(,$args:expr)*) => {
        Data::Function(Function::native(vec![$($args.into(),)*], $f))
    };
}