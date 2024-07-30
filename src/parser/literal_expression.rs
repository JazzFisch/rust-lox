use crate::token::{token_type::TokenType, token_value::TokenValue, Token};

use super::expression_value::ExpressionValue;

#[derive(Debug, PartialEq)]
pub struct LiteralExpression {
    value: ExpressionValue,
}

impl LiteralExpression {
    pub fn new(token: &Token) -> Self {
        let value = match (&token.token_type, &token.value) {
            (TokenType::Nil, _) => ExpressionValue::Nil,
            (TokenType::False, _) => ExpressionValue::Boolean(false),
            (TokenType::True, _) => ExpressionValue::Boolean(true),
            (_, TokenValue::Number(num)) => ExpressionValue::Number(*num),
            (_, TokenValue::String(str)) => ExpressionValue::String(str.clone()),
            _ => unreachable!("Invalid token value for literal expression {:?}", token),
        };

        Self { value }
    }

    pub fn value(&self) -> &ExpressionValue {
        &self.value
    }
}
