use std::fmt::Display;
use std::io::Write;

use token_type::TokenType;
use token_value::TokenValue;

pub mod token_type;
pub mod token_value;

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

    pub fn print(&self, output: &mut dyn Write) -> std::io::Result<()> {
        match self.token_type {
            // grouping tokens
            TokenType::LeftParen => writeln!(output, "LEFT_PAREN ( null"),
            TokenType::RightParen => writeln!(output, "RIGHT_PAREN ) null"),
            TokenType::LeftBrace => writeln!(output, "LEFT_BRACE {{ null"),
            TokenType::RightBrace => writeln!(output, "RIGHT_BRACE }} null"),
            // separator tokens
            TokenType::Comma => writeln!(output, "COMMA , null"),
            TokenType::Dot => writeln!(output, "DOT . null"),
            TokenType::Semicolon => writeln!(output, "SEMICOLON ; null"),
            // arithmetic tokens
            TokenType::Minus => writeln!(output, "MINUS - null"),
            TokenType::Plus => writeln!(output, "PLUS + null"),
            TokenType::Star => writeln!(output, "STAR * null"),
            TokenType::Slash => writeln!(output, "SLASH / null"),
            // comparison tokens
            TokenType::Equal => writeln!(output, "EQUAL = null"),
            TokenType::EqualEqual => writeln!(output, "EQUAL_EQUAL == null"),
            TokenType::Bang => writeln!(output, "BANG ! null"),
            TokenType::BangEqual => writeln!(output, "BANG_EQUAL != null"),
            TokenType::Greater => writeln!(output, "GREATER > null"),
            TokenType::GreaterEqual => writeln!(output, "GREATER_EQUAL >= null"),
            TokenType::Less => writeln!(output, "LESS < null"),
            TokenType::LessEqual => writeln!(output, "LESS_EQUAL <= null"),
            // keywords
            TokenType::And => writeln!(output, "AND and null"),
            TokenType::Class => writeln!(output, "CLASS class null"),
            TokenType::Else => writeln!(output, "ELSE else null"),
            TokenType::False => writeln!(output, "FALSE false null"),
            TokenType::For => writeln!(output, "FOR for null"),
            TokenType::Fun => writeln!(output, "FUN fun null"),
            TokenType::If => writeln!(output, "IF if null"),
            TokenType::Nil => writeln!(output, "NIL nil null"),
            TokenType::Or => writeln!(output, "OR or null"),
            TokenType::Print => writeln!(output, "PRINT print null"),
            TokenType::Return => writeln!(output, "RETURN return null"),
            TokenType::Super => writeln!(output, "SUPER super null"),
            TokenType::This => writeln!(output, "THIS this null"),
            TokenType::True => writeln!(output, "TRUE true null"),
            TokenType::Var => writeln!(output, "VAR var null"),
            TokenType::While => writeln!(output, "WHILE while null"),
            // special tokens
            TokenType::Eof => writeln!(output, "EOF  null"),
            // literals
            TokenType::Identifier => {
                if let TokenValue::Identifier(value) = &self.value {
                    writeln!(output, "IDENTIFIER {0} null", value)?;
                    return Ok(());
                }
                unreachable!("Expected identifier.  Found {:?}", self.value);
            }
            TokenType::String => {
                if let TokenValue::String(value) = &self.value {
                    writeln!(output, "STRING \"{0}\" {0}", value)?;
                    return Ok(());
                }
                unreachable!("Expected string.  Found {:?}", self.value);
            }
            TokenType::Number => {
                // this is a hack to get the output to match the book
                if let TokenValue::Number(value) = self.value {
                    if let Some(lexeme) = &self.lexeme {
                        if f64::trunc(value) == value {
                            writeln!(output, "NUMBER {} {:.1}", lexeme, value)?;
                        } else {
                            writeln!(output, "NUMBER {} {}", lexeme, value)?;
                        }
                        return Ok(());
                    }
                }
                unreachable!("Expected lexeme.  Found {:?}", self.lexeme);
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
    use rstest::*;

    #[test]
    fn test_token_new() {
        let token = Token::new(
            TokenType::Number,
            4,
            Some("1.23".to_string()),
            TokenValue::Number(1.23),
        );
        assert_eq!(token.token_type, TokenType::Number);
        assert_eq!(token.line, 4);
        assert_eq!(token.lexeme, Some("1.23".to_string()));
        assert_eq!(token.value, TokenValue::Number(1.23));
    }

    #[test]
    fn test_token_from_token_type() {
        let token = Token::from_token_type(2, TokenType::BangEqual);
        assert_eq!(token.token_type, TokenType::BangEqual);
        assert_eq!(token.line, 2);
        assert_eq!(token.lexeme, None);
        assert_eq!(token.value, TokenValue::None);
    }

    #[test]
    fn test_token_new_number() {
        let token = Token::new_number(2, "1.23".to_string(), 1.23);
        assert_eq!(token.token_type, TokenType::Number);
        assert_eq!(token.line, 2);
        assert_eq!(token.lexeme, Some("1.23".to_string()));
        assert_eq!(token.value, TokenValue::Number(1.23));
    }

    #[test]
    fn test_token_new_string() {
        let token = Token::new_string(2, "test_string".to_string());
        assert_eq!(token.token_type, TokenType::String);
        assert_eq!(token.line, 2);
        assert_eq!(token.lexeme, None);
        assert_eq!(token.value, TokenValue::String("test_string".to_string()));
    }

    #[test]
    fn test_token_new_identifier() {
        let token = Token::new_identifier(2, "test_identifier".to_string());
        assert_eq!(token.token_type, TokenType::Identifier);
        assert_eq!(token.line, 2);
        assert_eq!(token.lexeme, None);
        assert_eq!(
            token.value,
            TokenValue::Identifier("test_identifier".to_string())
        );
    }

    #[test]
    fn test_token_new_eof() {
        let token = Token::new_eof(2);
        assert_eq!(token.token_type, TokenType::Eof);
        assert_eq!(token.line, 2);
        assert_eq!(token.lexeme, None);
        assert_eq!(token.value, TokenValue::None);
    }

    #[rstest]
    #[case(TokenType::LeftParen, "LEFT_PAREN ( null\n")]
    #[case(TokenType::RightParen, "RIGHT_PAREN ) null\n")]
    #[case(TokenType::LeftBrace, "LEFT_BRACE { null\n")]
    #[case(TokenType::RightBrace, "RIGHT_BRACE } null\n")]
    #[case(TokenType::Comma, "COMMA , null\n")]
    #[case(TokenType::Dot, "DOT . null\n")]
    #[case(TokenType::Semicolon, "SEMICOLON ; null\n")]
    #[case(TokenType::Minus, "MINUS - null\n")]
    #[case(TokenType::Plus, "PLUS + null\n")]
    #[case(TokenType::Star, "STAR * null\n")]
    #[case(TokenType::Slash, "SLASH / null\n")]
    #[case(TokenType::Equal, "EQUAL = null\n")]
    #[case(TokenType::EqualEqual, "EQUAL_EQUAL == null\n")]
    #[case(TokenType::Bang, "BANG ! null\n")]
    #[case(TokenType::BangEqual, "BANG_EQUAL != null\n")]
    #[case(TokenType::Greater, "GREATER > null\n")]
    #[case(TokenType::GreaterEqual, "GREATER_EQUAL >= null\n")]
    #[case(TokenType::Less, "LESS < null\n")]
    #[case(TokenType::LessEqual, "LESS_EQUAL <= null\n")]
    #[case(TokenType::And, "AND and null\n")]
    #[case(TokenType::Class, "CLASS class null\n")]
    #[case(TokenType::Else, "ELSE else null\n")]
    #[case(TokenType::False, "FALSE false null\n")]
    #[case(TokenType::For, "FOR for null\n")]
    #[case(TokenType::Fun, "FUN fun null\n")]
    #[case(TokenType::If, "IF if null\n")]
    #[case(TokenType::Nil, "NIL nil null\n")]
    #[case(TokenType::Or, "OR or null\n")]
    #[case(TokenType::Print, "PRINT print null\n")]
    #[case(TokenType::Return, "RETURN return null\n")]
    #[case(TokenType::Super, "SUPER super null\n")]
    #[case(TokenType::This, "THIS this null\n")]
    #[case(TokenType::True, "TRUE true null\n")]
    #[case(TokenType::Var, "VAR var null\n")]
    #[case(TokenType::While, "WHILE while null\n")]
    #[case(TokenType::Eof, "EOF  null\n")]
    fn test_token_print(#[case] token_type: TokenType, #[case] expected: &str) {
        let token = Token::from_token_type(2, token_type);
        let mut output = Vec::new();

        token.print(&mut output).unwrap();

        assert_eq!(String::from_utf8(output).unwrap(), expected);
    }
}
