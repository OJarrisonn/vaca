use edn_format::Value;

pub mod keyword;
pub mod literal;
pub mod module;
pub mod symbol;

pub trait Parseable: Sized {
    type Error;

    fn parse(value: Value) -> Result<Self, Self::Error>;

    fn accept(value: &Value) -> bool;
}