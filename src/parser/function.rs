use std::fmt::{Display, Formatter};

use crate::{
    interpreter::{interpreter_error::InterpreterError, Interpreter},
    token::{token_value::TokenValue, Token},
};

use super::{callable::Callable, object::Object, statement::Statement};

#[derive(Debug, Clone)]
pub struct Function {
    name: String,
    params: Vec<Token>,
    body: Vec<Statement>,
}

impl Function {
    pub fn new(name: String, params: Vec<Token>, body: Vec<Statement>) -> Self {
        return Self { name, params, body };
    }
}

impl Callable for Function {
    fn arity(&self) -> usize {
        self.params.len()
    }

    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<Object>,
    ) -> Result<Object, InterpreterError> {
        interpreter.environment.push_scope();

        for (param, argument) in self.params.iter().zip(arguments.iter()) {
            if let TokenValue::Identifier(name) = &param.value {
                interpreter
                    .environment
                    .define(name.as_str(), argument.clone());
            }
        }

        interpreter.execute_block(&self.body)?;

        interpreter.environment.pop_scope();
        Ok(Object::Nil)
    }
}

impl Display for Function {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<fn {}>", self.name)
    }
}
