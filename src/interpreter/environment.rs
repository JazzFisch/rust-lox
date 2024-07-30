use std::collections::HashMap;

use crate::parser::expression_value::ExpressionValue;

use super::interpreter_error::InterpreterError;

pub struct Environment {
    values: HashMap<String, ExpressionValue>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: String, value: ExpressionValue) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &str) -> Result<&ExpressionValue, InterpreterError> {
        if let Some(value) = self.values.get(name) {
            Ok(value)
        } else {
            Err(InterpreterError::UndefinedVariable(name.to_string()))
        }
    }
}
