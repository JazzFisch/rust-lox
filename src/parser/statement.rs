use crate::{
    interpreter::interpreter_error::InterpreterError, token::Token,
    visitor::statement_visitor::StatementVisitor,
};

use super::{expression::Expression, object::Object};

#[derive(Clone, Debug)]
pub enum Statement {
    Block(Vec<Statement>),
    Expression(Expression),
    Function(Token, Vec<Token>, Vec<Statement>),
    If(Expression, Box<Statement>, Option<Box<Statement>>),
    Print(Expression),
    Return(Option<Expression>),
    Variable(Token, Option<Expression>),
    While(Expression, Box<Statement>),
}

impl Statement {
    pub fn accept(
        &self,
        visitor: &mut dyn StatementVisitor,
    ) -> Result<Option<Object>, InterpreterError> {
        match self {
            Statement::Block(statements) => visitor.visit_block_statement(statements),
            Statement::Expression(expr) => visitor.visit_expression_statement(expr),
            Statement::Function(name, params, body) => {
                visitor.visit_function_statement(name, params, body)
            }
            Statement::If(condition, then_branch, else_branch) => {
                visitor.visit_if_statement(condition, then_branch, else_branch)
            }
            Statement::Print(expr) => visitor.visit_print_statement(expr),
            Statement::Return(expr) => visitor.visit_return_statement(expr),
            Statement::Variable(name, expr) => visitor.visit_variable_statement(name, expr),
            Statement::While(condition, body) => visitor.visit_while_statement(condition, body),
        }
    }
}
