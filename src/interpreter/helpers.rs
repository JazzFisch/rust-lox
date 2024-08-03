use crate::{parser::object::Object, token::token_type::TokenType};

use super::interpreter_error::InterpreterError;

pub(crate) fn check_number_operand(
    operator: &TokenType,
    operand: &Object,
) -> Result<(), InterpreterError> {
    if let Object::Number(_) = operand {
        Ok(())
    } else {
        Err(InterpreterError::RuntimeError(format!(
            "Operand must be a number for operator ({} {})",
            operator, operand
        )))
    }
}

pub(crate) fn check_number_operands<'a>(
    left: &'a Object,
    operator: &TokenType,
    right: &'a Object,
) -> Result<(f64, f64), InterpreterError> {
    if let (Object::Number(left), Object::Number(right)) = (left, right) {
        Ok((*left, *right))
    } else {
        Err(InterpreterError::RuntimeError(format!(
            "Operands must be numbers for operator ({} {} {})",
            left, operator, right
        )))
    }
}
