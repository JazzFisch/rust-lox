use std::fmt::Display;

use keyword_type::KeywordType;
use token_type::TokenType;

pub mod keyword_type;
pub mod token_type;

#[derive(Debug, Default, Clone)]
pub enum TokenValue {
    #[default]
    None,
    Number(f64),
    String(String),
    Identifier(String),
    Keyword(KeywordType)
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub line: usize,
    pub lexeme: Option<String>,
    pub value: TokenValue
}

impl Token {
    pub fn new(token_type: TokenType, line: usize, lexeme: Option<String>, value: TokenValue) -> Self {
        Self {
            token_type,
            line,
            lexeme,
            value
        }
    }

    pub fn new_character(line: usize, token_type: TokenType) -> Self {
        Self::new(token_type, line, None, TokenValue::None)
    }

    pub fn new_number(line: usize, lexeme: String, value: f64) -> Self {
        Self::new(TokenType::Number, line, Some(lexeme), TokenValue::Number(value))
    }

    pub fn new_string(line: usize, value: String) -> Self {
        Self::new(TokenType::String, line, None, TokenValue::String(value))
    }

    pub fn new_identifier(line: usize, value: String) -> Self {
        Self::new(TokenType::Identifier, line, None, TokenValue::Identifier(value))
    }

    pub fn new_keyword(line: usize, keyword: KeywordType) -> Self {
        Self::new(TokenType::Keyword, line, None, TokenValue::Keyword(keyword))
    }

    pub fn new_eof(line: usize) -> Self {
        Self::new_character(line, TokenType::Eof)
    }
}

impl Display for TokenValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenValue::None => write!(f, ""),
            TokenValue::Number(value) => {
                if f64::trunc(*value) == *value {
                    write!(f, "{:.1}", value)
                }
                else {
                    write!(f, "{}", value)
                }
            },
            TokenValue::String(value) => write!(f, "{}", value),
            TokenValue::Identifier(value) => write!(f, "{}", value),
            TokenValue::Keyword(value) => write!(f, "{}", value),
        }
    }
}
