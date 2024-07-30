use crate::token::Token;

#[derive(Debug, PartialEq)]
pub struct VariableExpression {
    name: Token,
}

impl VariableExpression {
    pub fn new(name: Token) -> Self {
        VariableExpression { name }
    }

    pub fn name(&self) -> &Token {
        &self.name
    }
}
