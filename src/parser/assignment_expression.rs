use crate::token::Token;

use super::expression::Expression;

#[derive(Debug, PartialEq)]
pub struct AssignmentExpression {
    name: Token,
    expression: Box<Expression>,
}

impl AssignmentExpression {
    pub fn new(name: Token, expression: Expression) -> Self {
        Self {
            name,
            expression: Box::new(expression),
        }
    }

    pub fn name(&self) -> &Token {
        &self.name
    }

    pub fn expression(&self) -> &Expression {
        &self.expression
    }
}
