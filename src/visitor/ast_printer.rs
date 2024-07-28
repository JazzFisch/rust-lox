use crate::parser::{
    ast_visitor::AstVisitor,
    binary_expression::BinaryExpression,
    expression::Expression,
    grouping_expression::GroupingExpression,
    literal_expression::LiteralExpression,
    unary_expression::UnaryExpression
};

pub struct AstPrinter;

impl AstPrinter {
    pub fn print(&self, expression: &Expression) -> String {
        expression.accept(self)
    }
}

impl AstVisitor<String> for AstPrinter {
    fn visit_binary(&self, binary: &BinaryExpression) -> String {
        let left = binary.left().accept(self);
        let operator = binary.operator();
        let right = binary.right().accept(self);
        format!("({} {} {})", operator.token_type, left, right)
    }

    fn visit_grouping(&self, grouping: &GroupingExpression) -> String {
        let expr = grouping.expression().accept(self);
        format!("(group {})", expr)
    }

    fn visit_literal(&self, literal: &LiteralExpression) -> String {
        let value = literal.value();
        format!("{}", value)
    }

    fn visit_unary(&self, unary: &UnaryExpression) -> String {
        let operator = unary.operator();
        let right = unary.right().accept(self);
        format!("({} {})", operator.token_type, right)
    }
}