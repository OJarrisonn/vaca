use edn_format::Value;

pub mod array;
pub mod form;
pub mod keyword;
pub mod literal;
pub mod list;
pub mod map;
pub mod symbol;

pub trait Parseable: Sized {
    type Error;

    fn parse(value: Value) -> Result<Self, Self::Error>;

    fn accept(value: &Value) -> bool;
}