use crate::parser::{
    assignment_expression::AssignmentExpression, binary_expression::BinaryExpression,
    grouping_expression::GroupingExpression, literal_expression::LiteralExpression,
    unary_expression::UnaryExpression, variable_expression::VariableExpression,
};

pub trait ExpressionVisitor<T, E> {
    fn visit_assignment(&mut self, assignment: &AssignmentExpression) -> Result<T, E>;
    fn visit_binary(&mut self, binary: &BinaryExpression) -> Result<T, E>;
    fn visit_grouping(&mut self, grouping: &GroupingExpression) -> Result<T, E>;
    fn visit_literal(&mut self, literal: &LiteralExpression) -> Result<T, E>;
    fn visit_unary(&mut self, unary: &UnaryExpression) -> Result<T, E>;
    fn visit_variable(&mut self, variable: &VariableExpression) -> Result<T, E>;
}
