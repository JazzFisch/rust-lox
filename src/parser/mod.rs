use std::io::{self, Write};

use parse_error::ParseError;

use crate::token::{keyword_type::KeywordType, token_type::TokenType, Token, TokenValue};

pub mod ast_printer;
pub mod parse_error;

macro_rules! match_tokens {
    ($self:expr, $($token:expr),* $(,)?) => {{
        $(
            if $self.check($token) {
                $self.advance();
                true
            } else
        )* {
            false
        }
    }};
}

pub enum Expression {
    Binary(Box<Expression>, Token, Box<Expression>),
    Grouping(Box<Expression>),
    Literal(Token),
    Unary(Token, Box<Expression>),
}

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    failed: bool
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0, failed: false }
    }

    pub fn parse(&mut self) -> Result<Expression, ParseError> {
        self.expression()
    }

    pub fn failed(&self) -> bool {
        self.failed
    }

    fn advance(&mut self) -> Option<&Token> {
        if let Some(token) = self.tokens.get(self.current) {
            self.current += 1;
            return Some(token);
        }

        None
    }

    fn check(&self, token_type: TokenType) -> bool {
        if let Some(next_token) = self.peek() {
            return next_token.token_type == token_type;
        }

        false
    }

    fn comparison(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.term()?;

        while match_tokens!(self, TokenType::Greater, TokenType::GreaterEqual, TokenType::Less, TokenType::LessEqual) {
            let operator = self.previous().unwrap().clone();
            let right = self.term()?;
            expr = Expression::Binary(Box::new(expr), operator, Box::new(right));
        }

        Ok(expr)
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> Result<Option<&Token>, ParseError> {
        if self.check(token_type) {
            return Ok(self.advance());
        }

        let token = match self.peek() {
            Some(token) => token.clone(),
            None => Token::new_eof(0),
        };
        Err(self.error(&token, message))
    }

    fn error(&mut self, token: &Token, message: &str) -> ParseError {
        if token.token_type == TokenType::Eof {
            self.report(token.line, " at end", message);
        }
        else {
            self.report(token.line, format!(" at '{}'", token.value).as_str(), message);
        }

        self.failed = true;
        ParseError::new(token.clone(), message.to_string())
    }

    fn equality(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.comparison()?;

        while match_tokens!(self, TokenType::BangEqual, TokenType::EqualEqual) {
            let operator = self.previous().unwrap().clone();
            let right = self.comparison()?;
            expr = Expression::Binary(Box::new(expr), operator, Box::new(right));
        }

        Ok(expr)
    }

    fn expression(&mut self) -> Result<Expression, ParseError> {
        self.equality()
    }

    fn factor(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.unary()?;

        while match_tokens!(self, TokenType::Slash, TokenType::Star) {
            let operator = self.previous().unwrap().clone();
            let right = self.unary()?;
            expr = Expression::Binary(Box::new(expr), operator, Box::new(right));
        }

        Ok(expr)
    }

    fn match_keywords(&mut self, keyword_types: &[KeywordType]) -> bool {
        for keyword_type in keyword_types {
            if self.check(TokenType::Keyword) {
                if let TokenValue::Keyword(token_keyword) = self.peek().unwrap().value {
                    if token_keyword == *keyword_type {
                        self.advance();
                        return true;
                    }
                }
            }
        }

        false
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    fn previous(&self) -> Option<&Token> {
        self.tokens.get(self.current - 1)
    }

    fn primary(&mut self) -> Result<Expression, ParseError> {
        if self.match_keywords(&[KeywordType::False, KeywordType::True, KeywordType::Nil]) {
            let previous = self.previous().unwrap().clone();
            return Ok(Expression::Literal(previous));
        }

        // special case for number and string literals
        if match_tokens!(self, TokenType::Number, TokenType::String) {
            let previous = self.previous().unwrap();
            return Ok(Expression::Literal(previous.clone()));
        }

        if match_tokens!(self, TokenType::LeftParen) {
            let expr = self.expression()?;
            if let Err(err) =  self.consume(TokenType::RightParen, "Expect ')' after expression.") {
                self.synchronize();
                return Err(err);
            }
            return Ok(Expression::Grouping(Box::new(expr)));
        }

        let token = match self.peek() {
            Some(token) => token.clone(),
            None => Token::new_eof(0),
        };
        Err(self.error(&token, "Expect expression."))
    }

    fn report(&self, line: usize, location: &str, message: &str) {
        writeln!(io::stderr(), "[line {line}] Error{location}: {message}").unwrap();
    }

    fn synchronize(&mut self) {
        self.advance();

        while self.peek().is_some() {
            if self.previous().unwrap().token_type == TokenType::Semicolon {
                return;
            }

            if self.peek().unwrap().token_type == TokenType::Keyword {
                match self.peek().unwrap().value {
                    TokenValue::Keyword(keyword) => {
                        match keyword {
                            KeywordType::Class | KeywordType::Fun | KeywordType::Var | KeywordType::For |
                            KeywordType::If | KeywordType::While | KeywordType::Print | KeywordType::Return => {
                                return;
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }

            self.advance();
        }
    }

    fn term(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.factor()?;

        while match_tokens!(self, TokenType::Minus, TokenType::Plus) {
            let operator = self.previous().unwrap().clone();
            let right = self.factor()?;
            expr = Expression::Binary(Box::new(expr), operator, Box::new(right));
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expression, ParseError> {
        if match_tokens!(self, TokenType::Bang, TokenType::Minus) {
            let operator = self.previous().unwrap().clone();
            let right = self.unary()?;
            return Ok(Expression::Unary(operator, Box::new(right)));
        }

        self.primary()
    }
}
