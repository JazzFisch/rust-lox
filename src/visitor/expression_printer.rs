use crate::parser::{
    assignment_expression::AssignmentExpression, binary_expression::BinaryExpression,
    expression::Expression, grouping_expression::GroupingExpression,
    literal_expression::LiteralExpression, logical_expression::LogicalExpression,
    unary_expression::UnaryExpression,
};

use super::expression_visitor::ExpressionVisitor;

pub struct ExpressionPrinter;

impl ExpressionPrinter {
    pub fn print(&mut self, expression: &Expression) -> String {
        expression.accept(self).unwrap()
    }
}

impl ExpressionVisitor<String, String> for ExpressionPrinter {
    fn visit_assignment(&mut self, assignment: &AssignmentExpression) -> Result<String, String> {
        let name = assignment.name();
        let expr = assignment.expression().accept(self)?;
        Ok(format!("(set {} {})", name.value, expr))
    }

    fn visit_binary(&mut self, binary: &BinaryExpression) -> Result<String, String> {
        let left = binary.left().accept(self)?;
        let operator = binary.operator();
        let right = binary.right().accept(self)?;
        Ok(format!("({} {} {})", operator.token_type, left, right))
    }

    fn visit_grouping(&mut self, grouping: &GroupingExpression) -> Result<String, String> {
        let expr = grouping.expression().accept(self)?;
        Ok(format!("(group {})", expr))
    }

    fn visit_literal(&mut self, literal: &LiteralExpression) -> Result<String, String> {
        let value = literal.value();
        Ok(format!("{}", value))
    }

    fn visit_logical(&mut self, logical: &LogicalExpression) -> Result<String, String> {
        let left = logical.left().accept(self)?;
        let operator = logical.operator();
        let right = logical.right().accept(self)?;
        Ok(format!("({} {} {})", operator.token_type, left, right))
    }

    fn visit_unary(&mut self, unary: &UnaryExpression) -> Result<String, String> {
        let operator = unary.operator();
        let right = unary.right().accept(self)?;
        Ok(format!("({} {})", operator.token_type, right))
    }

    fn visit_variable(
        &mut self,
        variable: &crate::parser::variable_expression::VariableExpression,
    ) -> Result<String, String> {
        let name = variable.name();
        Ok(format!("{}", name.value))
    }
}
