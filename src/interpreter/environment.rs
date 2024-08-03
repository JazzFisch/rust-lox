use std::collections::{HashMap, VecDeque};

use crate::{
    parser::object::Object,
    token::{token_value::TokenValue, Token},
};

use super::interpreter_error::InterpreterError;

#[derive(Clone, Debug)]
pub struct Environment {
    stack: VecDeque<HashMap<String, Object>>,
}

impl Environment {
    pub fn new() -> Self {
        let stack = VecDeque::from(vec![HashMap::new()]);
        Environment::new_from_parent(&mut Self { stack })
    }

    pub fn new_from_parent(parent: &mut Environment) -> Self {
        let mut stack = VecDeque::new();
        stack.append(&mut parent.stack);
        Self { stack }
    }

    pub fn assign(&mut self, name_token: &Token, value: Object) -> Result<(), InterpreterError> {
        if let TokenValue::Identifier(name) = &name_token.value {
            for scope in self.stack.iter_mut() {
                if scope.contains_key(name) {
                    scope.insert(name.clone(), value);
                    return Ok(());
                }
            }
        }

        Err(InterpreterError::RuntimeError(format!(
            "Undefined variable '{}'.",
            name_token.value
        )))
    }

    pub fn define(&mut self, name: &str, value: Object) {
        let front = self.stack.front_mut();
        let front = front.unwrap();
        front.insert(name.to_owned(), value);
    }

    pub fn get(&self, name: &str) -> Result<Object, InterpreterError> {
        for scope in self.stack.iter() {
            if let Some(value) = scope.get(name) {
                return Ok(value.clone());
            }
        }

        Err(InterpreterError::UndefinedVariable(name.to_string()))
    }

    pub fn pop_scope(&mut self) {
        if self.stack.len() == 1 {
            panic!("Cannot pop the global scope.");
        }

        self.stack.pop_front();
    }

    pub fn push_scope(&mut self) {
        self.stack.push_front(HashMap::new());
    }
}
