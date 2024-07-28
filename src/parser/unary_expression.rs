use crate::token::Token;

use super::expression::Expression;

#[derive(Debug, PartialEq)]
pub struct UnaryExpression {
    operator: Token,
    right: Box<Expression>,
}

impl UnaryExpression {
    pub fn new(operator: Token, right: Expression) -> Self {
        UnaryExpression {
            operator,
            right: Box::new(right),
        }
    }

    pub fn operator(&self) -> &Token {
        &self.operator
    }

    pub fn right(&self) -> &Expression {
        &self.right
    }
}
