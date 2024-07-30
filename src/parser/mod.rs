pub mod assignment_expression;
pub mod binary_expression;
pub mod expression;
pub mod expression_value;
pub mod grouping_expression;
pub mod literal_expression;
pub mod parse_error;
pub mod statement;
pub mod unary_expression;
pub mod variable_expression;

use expression::Expression;
use parse_error::ParseError;
use statement::Statement;

use crate::token::{token_type::TokenType, token_value::TokenValue, Token};

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

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    failed: bool,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            current: 0,
            failed: false,
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Statement>, ParseError> {
        let mut statements: Vec<Statement> = Vec::new();

        while !self.is_at_end() {
            let stmt = self.declaration()?;
            statements.push(stmt);
        }

        Ok(statements)
    }

    fn advance(&mut self) -> Option<&Token> {
        if let Some(token) = self.tokens.get(self.current) {
            self.current += 1;
            return Some(token);
        }

        None
    }

    fn assignment(&mut self) -> Result<Expression, ParseError> {
        let expr = self.equality()?;

        if match_tokens!(self, TokenType::Equal) {
            let equals = self.previous().unwrap().clone();
            let value = self.assignment()?;

            if let Expression::Variable(var) = expr {
                return Ok(Expression::new_assignment(var.name().clone(), value));
            }

            return Err(self.error(&equals, "Invalid assignment target."));
        }

        Ok(expr)
    }

    fn check(&self, token_type: TokenType) -> bool {
        if let Some(next_token) = self.peek() {
            return next_token.token_type == token_type;
        }

        false
    }

    fn comparison(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.term()?;

        while match_tokens!(
            self,
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual
        ) {
            let operator = self.previous().unwrap().clone();
            let right = self.term()?;
            let left = expr;
            expr = Expression::new_binary(left, operator, right);
        }

        Ok(expr)
    }

    fn consume(
        &mut self,
        token_type: TokenType,
        message: &str,
    ) -> Result<Option<&Token>, ParseError> {
        if self.check(token_type) {
            return Ok(self.advance());
        }

        let token = match self.peek() {
            Some(token) => token.clone(),
            None => Token::new_eof(0),
        };
        Err(self.error(&token, message))
    }

    fn declaration(&mut self) -> Result<Statement, ParseError> {
        if match_tokens!(self, TokenType::Var) {
            self.variable_declaration()
        } else {
            self.statement()
        }
    }

    fn error(&mut self, token: &Token, message: &str) -> ParseError {
        match (&token.token_type, &token.value) {
            (TokenType::Eof, _) => self.report(token.line, " at end", message),
            (_, TokenValue::None) => self.report(
                token.line,
                format!(" at '{}'", token.token_type).as_str(),
                message,
            ),
            _ => self.report(
                token.line,
                format!(" at '{}'", token.value).as_str(),
                message,
            ),
        }

        self.failed = true;
        ParseError::Error(token.token_type, message.to_string())
    }

    fn equality(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.comparison()?;

        while match_tokens!(self, TokenType::BangEqual, TokenType::EqualEqual) {
            let operator = self.previous().unwrap().clone();
            let right = self.comparison()?;
            let left = expr;
            expr = Expression::new_binary(left, operator, right);
        }

        Ok(expr)
    }

    fn expression(&mut self) -> Result<Expression, ParseError> {
        self.assignment()
    }

    fn expression_statement(&mut self) -> Result<Statement, ParseError> {
        let expr = self.expression()?;
        if let Err(err) = self.consume(TokenType::Semicolon, "Expect ';' after expression.") {
            Err(err)
        } else {
            Ok(Statement::Expression(expr))
        }
    }

    fn factor(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.unary()?;

        while match_tokens!(self, TokenType::Slash, TokenType::Star) {
            let left = expr;
            let operator = self.previous().unwrap().clone();
            let right = self.unary()?;
            expr = Expression::new_binary(left, operator, right);
        }

        Ok(expr)
    }

    fn is_at_end(&self) -> bool {
        if let Some(token) = self.peek() {
            token.token_type == TokenType::Eof
        } else {
            true
        }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    fn previous(&self) -> Option<&Token> {
        self.tokens.get(self.current - 1)
    }

    fn primary(&mut self) -> Result<Expression, ParseError> {
        if match_tokens!(self, TokenType::False, TokenType::True, TokenType::Nil) {
            let token = self.previous().unwrap().clone();
            let expr = Expression::new_literal(&token);
            return Ok(expr);
        }

        // special case for number and string literals
        if match_tokens!(self, TokenType::Number, TokenType::String) {
            let token = self.previous().unwrap();
            let expr = Expression::new_literal(token);
            return Ok(expr);
        }

        if match_tokens!(self, TokenType::Identifier) {
            let token = self.previous().unwrap().clone();
            let expr = Expression::new_variable(token);
            return Ok(expr);
        }

        if match_tokens!(self, TokenType::LeftParen) {
            let expr = self.expression()?;
            if let Err(err) = self.consume(TokenType::RightParen, "Expect ')' after expression.") {
                self.synchronize();
                return Err(err);
            }
            let expr = Expression::new_grouping(expr);
            return Ok(expr);
        }

        let token = match self.peek() {
            Some(token) => token.clone(),
            None => Token::new_eof(0),
        };
        Err(self.error(&token, "Expect expression."))
    }

    fn print_statement(&mut self) -> Result<Statement, ParseError> {
        let expr = self.expression()?;
        if let Err(err) = self.consume(TokenType::Semicolon, "Expect ';' after value.") {
            Err(err)
        } else {
            Ok(Statement::Print(expr))
        }
    }

    fn report(&self, line: usize, location: &str, message: &str) {
        eprintln!("[line {line}] Error{location}: {message}");
    }

    fn statement(&mut self) -> Result<Statement, ParseError> {
        if match_tokens!(self, TokenType::Print) {
            self.print_statement()
        } else {
            self.expression_statement()
        }
    }

    fn synchronize(&mut self) {
        self.advance();

        while self.peek().is_some() {
            if self.previous().unwrap().token_type == TokenType::Semicolon {
                return;
            }

            match self.peek().unwrap().token_type {
                TokenType::Class
                | TokenType::Fun
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => return,
                _ => {}
            }

            self.advance();
        }
    }

    fn term(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.factor()?;

        while match_tokens!(self, TokenType::Minus, TokenType::Plus) {
            let left = expr;
            let operator = self.previous().unwrap().clone();
            let right = self.factor()?;
            expr = Expression::new_binary(left, operator, right);
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expression, ParseError> {
        if match_tokens!(self, TokenType::Bang, TokenType::Minus) {
            let operator = self.previous().unwrap().clone();
            let right = self.unary()?;
            let expr = Expression::new_unary(operator, right);
            return Ok(expr);
        }

        self.primary()
    }

    fn variable_declaration(&mut self) -> Result<Statement, ParseError> {
        let name = self.consume(TokenType::Identifier, "Expect variable name.")?;
        let name = name.unwrap().clone();

        let mut expr = None;
        if match_tokens!(self, TokenType::Equal) {
            expr = Some(self.expression()?);
        }

        if let Err(err) = self.consume(TokenType::Semicolon, "Expect ';' after expression.") {
            Err(err)
        } else {
            Ok(Statement::Variable(name, expr))
        }
    }
}
