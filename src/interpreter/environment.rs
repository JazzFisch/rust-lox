use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    parser::object::Object,
    token::{token_value::TokenValue, Token},
};

use super::interpreter_error::InterpreterError;

#[derive(Clone, Debug)]
pub struct Environment {
    pub parent: Option<Rc<RefCell<Environment>>>,
    values: HashMap<String, Object>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            parent: None,
            values: HashMap::new(),
        }
    }

    pub fn new_with_parent(parent: Rc<RefCell<Environment>>) -> Self {
        Self {
            parent: Some(parent),
            values: HashMap::new(),
        }
    }

    pub fn assign(&mut self, name_token: &Token, value: Object) -> Result<(), InterpreterError> {
        if let TokenValue::Identifier(name) = &name_token.value {
            if self.values.contains_key(name) {
                self.values.insert(name.clone(), value);
                return Ok(());
            } else if let Some(parent) = &mut self.parent {
                return parent.borrow_mut().assign(name_token, value);
            }
        }

        Err(InterpreterError::RuntimeError(format!(
            "Undefined variable '{}'.",
            name_token.value
        )))
    }

    pub fn define(&mut self, name: String, value: Object) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &str) -> Result<Object, InterpreterError> {
        if let Some(value) = self.values.get(name) {
            return Ok(value.clone());
        } else if let Some(parent) = &self.parent {
            return parent.borrow().get(name);
        }

        Err(InterpreterError::UndefinedVariable(name.to_string()))
    }
}
