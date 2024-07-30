use crate::{interpreter::interpreter_error::InterpreterError, parser::expression::Expression};

pub trait StatementVisitor {
    fn visit_expression_statement(&self, expr: &Expression) -> Result<(), InterpreterError>;
    fn visit_print_statement(&self, expr: &Expression) -> Result<(), InterpreterError>;
}
