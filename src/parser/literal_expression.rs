use crate::token::{token_type::TokenType, Token, TokenValue};

use super::expression::Value;

#[derive(Debug, PartialEq)]
pub struct LiteralExpression {
    value: Value,
}

impl LiteralExpression {
    pub fn new(token: &Token) -> Self {
        let value = match (&token.token_type, &token.value) {
            (TokenType::Nil, _) => Value::Nil,
            (TokenType::False, _) => Value::Boolean(false),
            (TokenType::True, _) => Value::Boolean(true),
            (_, TokenValue::Number(num)) => Value::Number(*num),
            (_, TokenValue::String(str)) => Value::String(str.clone()),
            _ => unreachable!("Invalid token value for literal expression {:?}", token),
        };

        Self { value }
    }

    pub fn value(&self) -> &Value {
        &self.value
    }
}
