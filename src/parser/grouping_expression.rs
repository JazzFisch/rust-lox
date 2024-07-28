use super::expression::Expression;

pub struct GroupingExpression {
    expression: Box<Expression>,
}

impl GroupingExpression {
    pub fn new(expression: Expression) -> Self {
        Self {
            expression : Box::new(expression),
        }
    }

    pub fn expression(&self) -> &Expression {
        &self.expression
    }
}
