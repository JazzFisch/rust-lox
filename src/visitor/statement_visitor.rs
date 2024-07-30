use crate::{
    interpreter::interpreter_error::InterpreterError, parser::expression::Expression, token::Token,
};

pub trait StatementVisitor {
    fn visit_expression_statement(&self, expr: &Expression) -> Result<(), InterpreterError>;
    fn visit_print_statement(&self, print: &Expression) -> Result<(), InterpreterError>;
    fn visit_variable_statement(
        &mut self,
        name: &Token,
        initializer: &Option<Expression>,
    ) -> Result<(), InterpreterError>;
}
