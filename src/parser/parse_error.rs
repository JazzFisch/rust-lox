use crate::token::token_type::TokenType;

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum ParseError {
    #[error("Parse error: [line {0}] Error: {1}")]
    Error(TokenType, String),
}

// #[allow(dead_code)]
// #[derive(Clone, Debug)]
// pub struct ParseError {
//     pub token: Token,
//     pub message: String,
// }

// impl ParseError {
//     pub fn new(token: Token, message: String) -> Self {
//         Self { token, message }
//     }
// }
