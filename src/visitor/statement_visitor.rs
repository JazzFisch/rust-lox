use crate::{
    interpreter::interpreter_error::InterpreterError, parser::expression::Expression, token::Token,
};

pub trait StatementVisitor {
    fn visit_expression_statement(&mut self, expr: &Expression) -> Result<(), InterpreterError>;
    fn visit_print_statement(&mut self, print: &Expression) -> Result<(), InterpreterError>;
    fn visit_variable_statement(
        &mut self,
        name: &Token,
        initializer: &Option<Expression>,
    ) -> Result<(), InterpreterError>;
}
