use crate::parser::object::Object;

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum InterpreterError {
    #[error("Runtime error: {0}")]
    RuntimeError(String),

    #[error("Undefined variable: '{0}'.")]
    UndefinedVariable(String),

    #[error("Early return: '{0}'.")]
    Return(Object),
}
