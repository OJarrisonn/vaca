use std::ops::ControlFlow;


use super::valueref::ValueRef;

type Success = ValueRef;
type Failure = String;
pub enum ExecResult {
    Success(Success),
    Failure(Failure)
}

impl ExecResult {
    pub fn unwrap(self) -> ControlFlow<Failure, Success> {
        match self {
            ExecResult::Success(s) => ControlFlow::Continue(s),
            ExecResult::Failure(f) => ControlFlow::Break(f),
        }
    }
}

impl Into<Result<Success, Failure>> for ExecResult {
    fn into(self) -> Result<Success, Failure> {
        match self {
            ExecResult::Success(s) => Ok(s),
            ExecResult::Failure(f) => Err(f),
        }
    }
}

impl From<Result<Success, Failure>> for ExecResult {
    fn from(value: Result<Success, Failure>) -> Self {
        match value {
            Ok(s) => ExecResult::Success(s),
            Err(f) => ExecResult::Failure(f),
        }
    }
}