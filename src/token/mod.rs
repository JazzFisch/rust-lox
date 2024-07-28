use std::fmt::Display;

use token_type::TokenType;

pub mod token_type;

#[derive(Debug, Default, Clone, PartialEq)]
pub enum TokenValue {
    #[default]
    None,
    Number(f64),
    String(String),
    Identifier(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub line: usize,
    pub lexeme: Option<String>,
    pub value: TokenValue,
}

impl Token {
    pub fn new(
        token_type: TokenType,
        line: usize,
        lexeme: Option<String>,
        value: TokenValue,
    ) -> Self {
        Self {
            token_type,
            line,
            lexeme,
            value,
        }
    }

    pub fn from_token_type(line: usize, token_type: TokenType) -> Self {
        Self::new(token_type, line, None, TokenValue::None)
    }

    pub fn new_number(line: usize, lexeme: String, value: f64) -> Self {
        Self::new(
            TokenType::Number,
            line,
            Some(lexeme),
            TokenValue::Number(value),
        )
    }

    pub fn new_string(line: usize, value: String) -> Self {
        Self::new(TokenType::String, line, None, TokenValue::String(value))
    }

    pub fn new_identifier(line: usize, value: String) -> Self {
        Self::new(
            TokenType::Identifier,
            line,
            None,
            TokenValue::Identifier(value),
        )
    }

    pub fn new_eof(line: usize) -> Self {
        Self::from_token_type(line, TokenType::Eof)
    }

    pub fn print(&self) {
        match self.token_type {
            // grouping tokens
            TokenType::LeftParen => println!("LEFT_PAREN ( null"),
            TokenType::RightParen => println!("RIGHT_PAREN ) null"),
            TokenType::LeftBrace => println!("LEFT_BRACE {{ null"),
            TokenType::RightBrace => println!("RIGHT_BRACE }} null"),
            // separator tokens
            TokenType::Comma => println!("COMMA , null"),
            TokenType::Dot => println!("DOT . null"),
            TokenType::Semicolon => println!("SEMICOLON ; null"),
            // arithmetic tokens
            TokenType::Minus => println!("MINUS - null"),
            TokenType::Plus => println!("PLUS + null"),
            TokenType::Star => println!("STAR * null"),
            TokenType::Slash => println!("SLASH / null"),
            // comparison tokens
            TokenType::Equal => println!("EQUAL = null"),
            TokenType::EqualEqual => println!("EQUAL_EQUAL == null"),
            TokenType::Bang => println!("BANG ! null"),
            TokenType::BangEqual => println!("BANG_EQUAL != null"),
            TokenType::Greater => println!("GREATER > null"),
            TokenType::GreaterEqual => println!("GREATER_EQUAL >= null"),
            TokenType::Less => println!("LESS < null"),
            TokenType::LessEqual => println!("LESS_EQUAL <= null"),
            // keywords
            TokenType::And => println!("AND and null"),
            TokenType::Class => println!("CLASS class null"),
            TokenType::Else => println!("ELSE else null"),
            TokenType::False => println!("FALSE false null"),
            TokenType::For => println!("FOR for null"),
            TokenType::Fun => println!("FUN fun null"),
            TokenType::If => println!("IF if null"),
            TokenType::Nil => println!("NIL nil null"),
            TokenType::Or => println!("OR or null"),
            TokenType::Print => println!("PRINT print null"),
            TokenType::Return => println!("RETURN return null"),
            TokenType::Super => println!("SUPER super null"),
            TokenType::This => println!("THIS this null"),
            TokenType::True => println!("TRUE true null"),
            TokenType::Var => println!("VAR var null"),
            TokenType::While => println!("WHILE while null"),
            // special tokens
            TokenType::Eof => println!("EOF  null"),
            // literals
            TokenType::Identifier => {
                if let TokenValue::Identifier(value) = &self.value {
                    println!("IDENTIFIER {0} null", value);
                    return;
                }
                unreachable!("Expected identifier.  Found {:?}", self.value);
            }
            TokenType::String => {
                if let TokenValue::String(value) = &self.value {
                    println!("STRING \"{0}\" {0}", value);
                    return;
                }
                unreachable!("Expected string.  Found {:?}", self.value);
            }
            TokenType::Number => {
                // this is a hack to get the output to match the book
                if let TokenValue::Number(value) = self.value {
                    if let Some(lexeme) = &self.lexeme {
                        if f64::trunc(value) == value {
                            println!("NUMBER {} {:.1}", lexeme, value);
                        } else {
                            println!("NUMBER {} {}", lexeme, value);
                        }
                        return;
                    }
                    unreachable!("Expected lexeme.  Found {:?}", self.lexeme);
                }
            }
        }
    }
}

impl Display for TokenValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenValue::None => write!(f, ""),
            TokenValue::Number(value) => {
                if f64::trunc(*value) == *value {
                    write!(f, "{:.1}", value)
                } else {
                    write!(f, "{}", value)
                }
            }
            TokenValue::String(value) => write!(f, "{}", value),
            TokenValue::Identifier(value) => write!(f, "{}", value),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_new() {
        let token = Token::new(
            TokenType::Number,
            1,
            Some("1".to_string()),
            TokenValue::Number(1.0),
        );
        assert_eq!(token.token_type, TokenType::Number);
        assert_eq!(token.line, 1);
        assert_eq!(token.lexeme, Some("1".to_string()));
        assert_eq!(token.value, TokenValue::Number(1.0));
    }

    #[test]
    fn test_token_from_token_type() {
        let token = Token::from_token_type(1, TokenType::Number);
        assert_eq!(token.token_type, TokenType::Number);
        assert_eq!(token.line, 1);
        assert_eq!(token.lexeme, None);
        assert_eq!(token.value, TokenValue::None);
    }

    #[test]
    fn test_token_new_number() {
        let token = Token::new_number(1, "1".to_string(), 1.0);
        assert_eq!(token.token_type, TokenType::Number);
        assert_eq!(token.line, 1);
        assert_eq!(token.lexeme, Some("1".to_string()));
        assert_eq!(token.value, TokenValue::Number(1.0));
    }

    #[test]
    fn test_token_new_string() {
        let token = Token::new_string(1, "string".to_string());
        assert_eq!(token.token_type, TokenType::String);
        assert_eq!(token.line, 1);
        assert_eq!(token.lexeme, None);
        assert_eq!(token.value, TokenValue::String("string".to_string()));
    }

    #[test]
    fn test_token_new_identifier() {
        let token = Token::new_identifier(1, "identifier".to_string());
        assert_eq!(token.token_type, TokenType::Identifier);
        assert_eq!(token.line, 1);
        assert_eq!(token.lexeme, None);
        assert_eq!(
            token.value,
            TokenValue::Identifier("identifier".to_string())
        );
    }

    #[test]
    fn test_token_new_eof() {
        let token = Token::new_eof(1);
        assert_eq!(token.token_type, TokenType::Eof);
        assert_eq!(token.line, 1);
        assert_eq!(token.lexeme, None);
        assert_eq!(token.value, TokenValue::None);
    }
}
