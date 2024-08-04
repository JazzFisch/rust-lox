use std::{
    cell::RefCell,
    fmt::{Display, Formatter},
    rc::Rc,
};

use crate::{
    interpreter::{environment, interpreter_error::InterpreterError, Interpreter},
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
        Self { name, params, body }
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
        let environment = Rc::new(RefCell::new(environment::Environment::new(Some(
            Rc::clone(&interpreter.environment),
        ))));

        for (param, argument) in self.params.iter().zip(arguments.iter()) {
            if let TokenValue::Identifier(name) = &param.value {
                environment
                    .borrow_mut()
                    .define(name.as_str(), argument.clone());
            }
        }

        if let Err(execute_return) = interpreter.execute_block(&self.body, environment) {
            if let InterpreterError::Return(value) = execute_return {
                return Ok(value);
            }
            return Err(execute_return);
        }

        Ok(Object::Nil)
    }
}

impl Display for Function {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<fn {}>", self.name)
    }
}
