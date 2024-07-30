#[derive(thiserror::Error, Debug, PartialEq)]
pub enum InterpreterError {
    #[error("Runtime error: {0}")]
    RuntimeError(String),

    #[error("Undefined variable: '{0}'.")]
    UndefinedVariable(String),
}
