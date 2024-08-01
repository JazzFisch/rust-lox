use crate::token::Token;

use super::expression::Expression;

#[derive(Debug, PartialEq)]
pub struct LogicalExpression {
    left: Box<Expression>,
    operator: Token,
    right: Box<Expression>,
}

impl LogicalExpression {
    pub fn new(left: Expression, operator: Token, right: Expression) -> Self {
        Self {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }
    }

    pub fn left(&self) -> &Expression {
        &self.left
    }

    pub fn operator(&self) -> &Token {
        &self.operator
    }

    pub fn right(&self) -> &Expression {
        &self.right
    }
}
