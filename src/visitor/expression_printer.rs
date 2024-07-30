use crate::parser::{
    binary_expression::BinaryExpression, expression::Expression,
    grouping_expression::GroupingExpression, literal_expression::LiteralExpression,
    unary_expression::UnaryExpression,
};

use super::expression_visitor::ExpressionVisitor;

pub struct ExpressionPrinter;

impl ExpressionPrinter {
    pub fn print(&self, expression: &Expression) -> String {
        expression.accept(self).unwrap()
    }
}

impl ExpressionVisitor<String, String> for ExpressionPrinter {
    fn visit_binary(&self, binary: &BinaryExpression) -> Result<String, String> {
        let left = binary.left().accept(self)?;
        let operator = binary.operator();
        let right = binary.right().accept(self)?;
        Ok(format!("({} {} {})", operator.token_type, left, right))
    }

    fn visit_grouping(&self, grouping: &GroupingExpression) -> Result<String, String> {
        let expr = grouping.expression().accept(self)?;
        Ok(format!("(group {})", expr))
    }

    fn visit_literal(&self, literal: &LiteralExpression) -> Result<String, String> {
        let value = literal.value();
        Ok(format!("{}", value))
    }

    fn visit_unary(&self, unary: &UnaryExpression) -> Result<String, String> {
        let operator = unary.operator();
        let right = unary.right().accept(self)?;
        Ok(format!("({} {})", operator.token_type, right))
    }
}
