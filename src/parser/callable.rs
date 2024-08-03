use dyn_clone::DynClone;
use std::fmt::{Debug, Display};

use crate::interpreter::{interpreter_error::InterpreterError, Interpreter};

use super::object::Object;

dyn_clone::clone_trait_object!(Callable);

pub trait Callable: Debug + Display + DynClone {
    fn arity(&self) -> usize;
    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<Object>,
    ) -> Result<Object, InterpreterError>;
}

#[derive(Debug, Clone)]
pub struct Clock {}

impl Callable for Clock {
    fn arity(&self) -> usize {
        0
    }

    fn call(
        &self,
        _interpreter: &mut Interpreter,
        _arguments: Vec<Object>,
    ) -> Result<Object, InterpreterError> {
        Ok(Object::Number(
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs_f64(),
        ))
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<fn clock>")
    }
}
