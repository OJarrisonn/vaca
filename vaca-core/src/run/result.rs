use super::error::RunErrorStack;

pub type RunResult<T> = Result<T, RunErrorStack>;