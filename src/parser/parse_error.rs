use crate::token::Token;

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct ParseError {
    pub token: Token,
    pub message: String,
}

impl ParseError {
    pub fn new(token: Token, message: String) -> Self {
        Self { token, message }
    }
}
