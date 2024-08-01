use crate::parser::expression::Expression;

use super::expression_visitor::ExpressionVisitor;

pub struct ExpressionPrinter;

impl ExpressionPrinter {
    pub fn print(&mut self, expression: &Expression) -> String {
        expression.accept(self).unwrap()
    }
}

impl ExpressionVisitor<String, String> for ExpressionPrinter {
    fn visit_assignment(
        &mut self,
        name: &crate::token::Token,
        expression: &Expression,
    ) -> Result<String, String> {
        let expr = expression.accept(self)?;
        Ok(format!("(set {} {})", name.value, expr))
    }

    fn visit_binary(
        &mut self,
        left: &Expression,
        operator: &crate::token::Token,
        right: &Expression,
    ) -> Result<String, String> {
        let left = left.accept(self)?;
        let right = right.accept(self)?;
        Ok(format!("({} {} {})", operator.token_type, left, right))
    }

    fn visit_grouping(&mut self, expression: &Expression) -> Result<String, String> {
        let expr = expression.accept(self)?;
        Ok(format!("(group {})", expr))
    }

    fn visit_literal(
        &mut self,
        value: &crate::parser::expression_value::ExpressionValue,
    ) -> Result<String, String> {
        Ok(format!("{}", value))
    }

    fn visit_logical(
        &mut self,
        left: &Expression,
        operator: &crate::token::Token,
        right: &Expression,
    ) -> Result<String, String> {
        let left = left.accept(self)?;
        let right = right.accept(self)?;
        Ok(format!("({} {} {})", operator.token_type, left, right))
    }

    fn visit_unary(
        &mut self,
        operator: &crate::token::Token,
        right: &Expression,
    ) -> Result<String, String> {
        let right = right.accept(self)?;
        Ok(format!("({} {})", operator.token_type, right))
    }

    fn visit_variable(&mut self, name: &crate::token::Token) -> Result<String, String> {
        Ok(format!("{}", name.value))
    }
}
