use crate::parser::{
    binary_expression::BinaryExpression, grouping_expression::GroupingExpression,
    literal_expression::LiteralExpression, unary_expression::UnaryExpression,
    variable_expression::VariableExpression,
};

pub trait ExpressionVisitor<T, E> {
    fn visit_binary(&self, binary: &BinaryExpression) -> Result<T, E>;
    fn visit_grouping(&self, grouping: &GroupingExpression) -> Result<T, E>;
    fn visit_literal(&self, literal: &LiteralExpression) -> Result<T, E>;
    fn visit_unary(&self, unary: &UnaryExpression) -> Result<T, E>;
    fn visit_variable(&self, variable: &VariableExpression) -> Result<T, E>;
}
