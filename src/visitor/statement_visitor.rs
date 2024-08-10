use crate::{
    interpreter::interpreter_error::InterpreterError,
    parser::{expression::Expression, object::Object, statement::Statement},
    token::Token,
};

pub trait StatementVisitor {
    fn visit_block_statement(
        &mut self,
        statements: &[Statement],
    ) -> Result<Option<Object>, InterpreterError>;
    fn visit_expression_statement(
        &mut self,
        expr: &Expression,
    ) -> Result<Option<Object>, InterpreterError>;
    fn visit_function_statement(
        &mut self,
        name: &Token,
        params: &[Token],
        body: &[Statement],
    ) -> Result<Option<Object>, InterpreterError>;
    fn visit_if_statement(
        &mut self,
        condition: &Expression,
        then_branch: &Statement,
        else_branch: &Option<Box<Statement>>,
    ) -> Result<Option<Object>, InterpreterError>;
    fn visit_print_statement(
        &mut self,
        print: &Expression,
    ) -> Result<Option<Object>, InterpreterError>;
    fn visit_return_statement(
        &mut self,
        value: &Option<Expression>,
    ) -> Result<Option<Object>, InterpreterError>;
    fn visit_variable_statement(
        &mut self,
        name: &Token,
        initializer: &Option<Expression>,
    ) -> Result<Option<Object>, InterpreterError>;
    fn visit_while_statement(
        &mut self,
        condition: &Expression,
        body: &Statement,
    ) -> Result<Option<Object>, InterpreterError>;
}
