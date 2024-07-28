use std::fmt::Display;

use crate::token::Token;

use super::{
    ast_visitor::AstVisitor, binary_expression::BinaryExpression,
    grouping_expression::GroupingExpression, literal_expression::LiteralExpression,
    unary_expression::UnaryExpression,
};

#[derive(Debug, PartialEq)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    Binary(Box<BinaryExpression>),
    Grouping(Box<GroupingExpression>),
    Literal(Box<LiteralExpression>),
    Unary(Box<UnaryExpression>),
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
}

impl Expression {
    pub fn accept<T>(&self, visitor: &dyn AstVisitor<T>) -> T {
        match self {
            Expression::Binary(expr) => visitor.visit_binary(expr),
            Expression::Grouping(expr) => visitor.visit_grouping(expr),
            Expression::Literal(expr) => visitor.visit_literal(expr),
            Expression::Unary(expr) => visitor.visit_unary(expr),
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::String(str) => write!(f, "{}", str),
            Value::Number(num) => {
                if f64::trunc(*num) == *num {
                    write!(f, "{:.1}", num)
                } else {
                    write!(f, "{}", num)
                }
            }
            Value::Boolean(bool) => write!(f, "{}", bool),
            Value::Nil => write!(f, "nil"),
        }
    }
}
