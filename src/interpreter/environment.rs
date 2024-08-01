use std::collections::{HashMap, VecDeque};

use crate::{
    parser::expression_value::ExpressionValue,
    token::{token_value::TokenValue, Token},
};

use super::interpreter_error::InterpreterError;

#[derive(Debug, Default)]
pub struct Environment {
    stack: VecDeque<HashMap<String, ExpressionValue>>,
}

impl Environment {
    pub fn new() -> Self {
        let first: HashMap<String, ExpressionValue> = HashMap::new();

        Self {
            stack: VecDeque::from(vec![first]),
        }
    }

    pub fn assign(&mut self, name: &Token, value: ExpressionValue) -> Result<(), InterpreterError> {
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

    pub fn define(&mut self, name: String, value: ExpressionValue) {
        self.stack.front_mut().unwrap().insert(name, value);
    }

    pub fn get(&self, name: &str) -> Result<ExpressionValue, InterpreterError> {
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
        let child: HashMap<String, ExpressionValue> = HashMap::new();
        self.stack.push_front(child);
    }
}
