use crate::{
    interpreter::interpreter_error::InterpreterError, token::Token,
    visitor::statement_visitor::StatementVisitor,
};

use super::expression::Expression;

pub enum Statement {
    Expression(Expression),
    Print(Expression),
    Variable(Token, Option<Expression>),
}

impl Statement {
    pub fn accept(&self, visitor: &mut dyn StatementVisitor) -> Result<(), InterpreterError> {
        match self {
            Statement::Expression(expr) => visitor.visit_expression_statement(expr),
            Statement::Print(expr) => visitor.visit_print_statement(expr),
            Statement::Variable(name, expr) => visitor.visit_variable_statement(name, expr),
        }
    }
}
