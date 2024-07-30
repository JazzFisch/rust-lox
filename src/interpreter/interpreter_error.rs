#[derive(thiserror::Error, Debug, PartialEq)]
pub enum InterpreterError {
    #[error("Runtime error: {0}")]
    RuntimeError(String),
}
