use super::{
    binary_expression::BinaryExpression, grouping_expression::GroupingExpression,
    literal_expression::LiteralExpression, unary_expression::UnaryExpression,
};

pub trait AstVisitor<T> {
    fn visit_binary(&self, binary: &BinaryExpression) -> T;
    fn visit_grouping(&self, grouping: &GroupingExpression) -> T;
    fn visit_literal(&self, literal: &LiteralExpression) -> T;
    fn visit_unary(&self, unary: &UnaryExpression) -> T;
}
