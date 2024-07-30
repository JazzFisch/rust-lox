use crate::{
    interpreter::interpreter_error::InterpreterError, visitor::statement_visitor::StatementVisitor,
};

use super::expression::Expression;

pub enum Statement {
    Expression(Expression),
    Print(Expression),
}

impl Statement {
    pub fn accept(&self, visitor: &dyn StatementVisitor) -> Result<(), InterpreterError> {
        match self {
            Statement::Expression(expr) => visitor.visit_expression_statement(expr),
            Statement::Print(expr) => visitor.visit_print_statement(expr),
        }
    }
}
