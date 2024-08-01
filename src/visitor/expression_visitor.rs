use crate::{
    parser::{expression::Expression, expression_value::ExpressionValue},
    token::Token,
};

pub trait ExpressionVisitor<T, E> {
    fn visit_assignment(&mut self, name: &Token, expression: &Expression) -> Result<T, E>;
    fn visit_binary(
        &mut self,
        left: &Expression,
        operator: &Token,
        right: &Expression,
    ) -> Result<T, E>;
    fn visit_grouping(&mut self, expression: &Expression) -> Result<T, E>;
    fn visit_literal(&mut self, value: &ExpressionValue) -> Result<T, E>;
    fn visit_logical(
        &mut self,
        left: &Expression,
        operator: &Token,
        right: &Expression,
    ) -> Result<T, E>;
    fn visit_unary(&mut self, operator: &Token, right: &Expression) -> Result<T, E>;
    fn visit_variable(&mut self, name: &Token) -> Result<T, E>;
}
