use vaca_core::{build::form::Form, run::{result::RunResult, valueref::ValueRef}};

#[derive(Debug)]
/// Native objects are simple vaca functions/macros implmented in Rust and imported by the VM on the fly
pub enum NativeObject {
    Function(),
    Macro()
}


/// Type for functions defined in Rust code
pub type NativeFunction = fn(Vec<ValueRef>) -> RunResult<ValueRef>;

/// Type for macros defined in Rust code
pub type NativeMacro = fn(Vec<Form>) -> RunResult<ValueRef>;