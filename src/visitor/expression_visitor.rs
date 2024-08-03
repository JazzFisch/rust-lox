use crate::{
    parser::{expression::Expression, object::Object},
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
    fn visit_call(
        &mut self,
        callee: &Expression,
        paren: &Token,
        arguments: &[Expression],
    ) -> Result<T, E>;
    fn visit_grouping(&mut self, expression: &Expression) -> Result<T, E>;
    fn visit_literal(&mut self, value: &Object) -> Result<T, E>;
    fn visit_logical(
        &mut self,
        left: &Expression,
        operator: &Token,
        right: &Expression,
    ) -> Result<T, E>;
    fn visit_unary(&mut self, operator: &Token, right: &Expression) -> Result<T, E>;
    fn visit_variable(&mut self, name: &Token) -> Result<T, E>;
}
