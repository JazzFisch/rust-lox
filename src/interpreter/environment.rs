use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::parser::object::Object;

use super::interpreter_error::InterpreterError;

#[derive(Debug, Clone)]
pub struct Environment {
    parent: Option<Rc<RefCell<Environment>>>,
    values: HashMap<String, Object>,
}

impl Environment {
    pub fn new(parent: Option<Rc<RefCell<Environment>>>) -> Self {
        Environment {
            parent,
            values: HashMap::new(),
        }
    }

    pub fn new_with_parent(parent: Rc<RefCell<Environment>>) -> Self {
        Environment {
            parent: Some(parent),
            values: HashMap::new(),
        }
    }

    pub fn assign(&mut self, name: &str, value: Object) -> Result<(), InterpreterError> {
        if self.values.contains_key(name) {
            self.values.insert(name.to_owned(), value);
            return Ok(());
        }

        if let Some(parent) = &self.parent {
            parent.borrow_mut().assign(name, value)
        } else {
            Err(InterpreterError::RuntimeError(format!(
                "Undefined variable '{}'.",
                name
            )))
        }
    }

    pub fn define(&mut self, name: &str, value: Object) {
        self.values.insert(name.to_owned(), value);
    }

    pub fn get(&self, name: &str) -> Result<Object, InterpreterError> {
        if let Some(value) = self.values.get(name) {
            return Ok(value.clone());
        }

        if let Some(parent) = &self.parent {
            parent.borrow().get(name)
        } else {
            Err(InterpreterError::UndefinedVariable(name.to_owned()))
        }
    }

    pub fn print(&self) {
        self.print_internal(0);
    }

    fn print_internal(&self, depth: usize) {
        if let Some(parent) = &self.parent {
            parent.borrow().print_internal(depth + 1);
        }

        for (key, value) in &self.values {
            println!("{:indent$}{}: {}", "", key, value, indent = depth * 2);
        }
    }
}

impl Default for Environment {
    fn default() -> Self {
        Self::new(None)
    }
}
