use vaca_core::build::error::BuildErrorStack;

pub mod form;
pub mod program;
pub mod library;
pub mod build;

pub type BuildResult<T> = Result<T, BuildErrorStack>;