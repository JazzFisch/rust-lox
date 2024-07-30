use std::fmt::Display;

use crate::{token::Token, visitor::expression_visitor::ExpressionVisitor};

use super::{
    binary_expression::BinaryExpression, expression_value::ExpressionValue,
    grouping_expression::GroupingExpression, literal_expression::LiteralExpression,
    unary_expression::UnaryExpression, variable_expression::VariableExpression,
};

#[derive(Debug, PartialEq)]
pub enum Expression {
    Binary(Box<BinaryExpression>),
    Grouping(Box<GroupingExpression>),
    Literal(Box<LiteralExpression>),
    Unary(Box<UnaryExpression>),
    Variable(Box<VariableExpression>),
}

impl Expression {
    pub fn new_binary(left: Expression, operand: Token, right: Expression) -> Self {
        let expr = BinaryExpression::new(left, operand, right);
        Expression::Binary(Box::new(expr))
    }

    pub fn new_grouping(expression: Expression) -> Self {
        let expr = GroupingExpression::new(expression);
        Expression::Grouping(Box::new(expr))
    }

    pub fn new_literal(token: &Token) -> Self {
        let expr = LiteralExpression::new(token);
        Expression::Literal(Box::new(expr))
    }

    pub fn new_unary(operator: Token, right: Expression) -> Self {
        let expr = UnaryExpression::new(operator, right);
        Expression::Unary(Box::new(expr))
    }

    pub fn new_variable(name: Token) -> Self {
        let expr = VariableExpression::new(name);
        Expression::Variable(Box::new(expr))
    }
}

impl Expression {
    pub fn accept<T, E>(&self, visitor: &dyn ExpressionVisitor<T, E>) -> Result<T, E> {
        match self {
            Expression::Binary(expr) => visitor.visit_binary(expr),
            Expression::Grouping(expr) => visitor.visit_grouping(expr),
            Expression::Literal(expr) => visitor.visit_literal(expr),
            Expression::Unary(expr) => visitor.visit_unary(expr),
            Expression::Variable(expr) => visitor.visit_variable(expr),
        }
    }
}

impl Display for ExpressionValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExpressionValue::String(str) => write!(f, "{}", str),
            ExpressionValue::Number(num) => {
                if f64::trunc(*num) == *num {
                    write!(f, "{:.1}", num)
                } else {
                    write!(f, "{}", num)
                }
            }
            ExpressionValue::Boolean(bool) => write!(f, "{}", bool),
            ExpressionValue::Nil => write!(f, "nil"),
        }
    }
}
