use std::collections::{HashMap, VecDeque};

use crate::{
    parser::object::Object,
    token::{token_value::TokenValue, Token},
};

use super::interpreter_error::InterpreterError;

#[derive(Debug, Default)]
pub struct Environment {
    stack: VecDeque<HashMap<String, Object>>,
}

impl Environment {
    pub fn new() -> Self {
        let first: HashMap<String, Object> = HashMap::new();

        Self {
            stack: VecDeque::from(vec![first]),
        }
    }

    pub fn assign(&mut self, name: &Token, value: Object) -> Result<(), InterpreterError> {
        if let TokenValue::Identifier(name) = &name.value {
            for values in self.stack.iter_mut() {
                if values.contains_key(name) {
                    values.insert(name.clone(), value);
                    return Ok(());
                }
            }
        }

        Err(InterpreterError::RuntimeError(format!(
            "Undefined variable '{}'.",
            name.value
        )))
    }

    pub fn define(&mut self, name: String, value: Object) {
        self.stack.front_mut().unwrap().insert(name, value);
    }

    pub fn get(&self, name: &str) -> Result<Object, InterpreterError> {
        for values in self.stack.iter() {
            if let Some(value) = values.get(name) {
                return Ok(value.clone());
            }
        }

        Err(InterpreterError::UndefinedVariable(name.to_string()))
    }

    pub fn pop_child(&mut self) {
        if self.stack.len() == 1 {
            unreachable!("Cannot pop the global environment");
        }

        self.stack.pop_front();
    }

    pub fn push_child(&mut self) {
        let child: HashMap<String, Object> = HashMap::new();
        self.stack.push_front(child);
    }
}
