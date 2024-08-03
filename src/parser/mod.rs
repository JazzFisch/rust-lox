pub mod callable;
pub mod expression;
pub mod function;
pub mod object;
pub mod parse_error;
pub mod statement;

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

    fn and(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.equality()?;

        while match_tokens!(self, TokenType::And) {
            let operator = self.previous().unwrap().clone();
            let right = self.equality()?;
            expr = Expression::new_logical(expr, operator, right);
        }

        Ok(expr)
    }

    fn assignment(&mut self) -> Result<Expression, ParseError> {
        let expr = self.or()?;
        if match_tokens!(self, TokenType::Equal) {
            let equals = self.previous().unwrap().clone();
            let value = self.assignment()?;

            if let Expression::Variable { name } = expr {
                return Ok(Expression::new_assignment(name, value));
            }

            // We report an error if the left-hand side isn’t a valid assignment target,
            // but we don’t throw it because the parser isn’t in a confused state where
            // we need to go into panic mode and synchronize.
            let _ = self.error(&equals, "Invalid assignment target.");
        }

        Ok(expr)
    }

    fn block(&mut self) -> Result<Vec<Statement>, ParseError> {
        let mut statements: Vec<Statement> = Vec::new();

        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            let stmt = self.declaration()?;
            statements.push(stmt);
        }

        if let Err(err) = self.consume(TokenType::RightBrace, "Expect '}' after block.") {
            Err(err)
        } else {
            Ok(statements)
        }
    }

    fn call(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.primary()?;

        loop {
            if match_tokens!(self, TokenType::LeftParen) {
                expr = self.finish_call(expr)?;
            } else {
                break;
            }
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
        if match_tokens!(self, TokenType::Fun) {
            self.function("function")
        } else if match_tokens!(self, TokenType::Var) {
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

    fn function(&mut self, kind: &str) -> Result<Statement, ParseError> {
        let name = self.consume(
            TokenType::Identifier,
            format!("Expect {kind} name.").as_str(),
        )?;
        let name = name.unwrap().clone();

        let _ = self.consume(
            TokenType::LeftParen,
            format!("Expect '(' after {kind} name.").as_str(),
        );

        let mut parameters = Vec::new();
        if !self.check(TokenType::RightParen) {
            loop {
                if parameters.len() >= 255 {
                    let token = self.peek().unwrap().clone();
                    self.error(&token, "Can't have more than 255 parameters.");
                }

                let param = self.consume(TokenType::Identifier, "Expect parameter name.")?;
                parameters.push(param.unwrap().clone());

                if !match_tokens!(self, TokenType::Comma) {
                    break;
                }
            }
        }

        let _ = self.consume(TokenType::RightParen, "Expect ')' after parameters.");
        let _ = self.consume(
            TokenType::LeftBrace,
            format!("Expect '{{' before {kind} body.").as_str(),
        );
        let body = self.block()?;
        Ok(Statement::Function(name, parameters, body))
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

    // for ( <initializer>; <condition>; <increment>) { <body> }
    fn for_statement(&mut self) -> Result<Statement, ParseError> {
        let _ = self.consume(TokenType::LeftParen, "Expect '(' after 'for'.");

        // first we handle <initializer>
        let initializer: Option<Statement>;
        if match_tokens!(self, TokenType::Semicolon) {
            initializer = None;
        } else if match_tokens!(self, TokenType::Var) {
            initializer = Some(self.variable_declaration()?);
        } else {
            initializer = Some(self.expression_statement()?);
        }

        // next, the <condition>
        // a missing condition's value is true
        let mut condition = Expression::new_literal(&Token::from_token_type(0, TokenType::True));
        if !self.check(TokenType::Semicolon) {
            condition = self.expression()?;
        }
        let _ = self.consume(TokenType::Semicolon, "Expect ';' after loop condition.");

        // then, the <increment>
        let mut increment: Option<Expression> = None;
        if !self.check(TokenType::RightParen) {
            increment = Some(self.expression()?);
        }
        let _ = self.consume(TokenType::RightParen, "Expect ')' after for clauses.");

        // and finally, the <body>
        let mut body = self.statement()?;

        // now to desugar into a while loop
        // working backward, we place the increment expression at the bottom
        // of the block that contains the body
        if let Some(increment) = increment {
            body = Statement::Block(vec![body, Statement::Expression(increment)])
        }

        // wrap the condition an new body inside of a while statement
        body = Statement::While(condition, Box::new(body));

        // finally, if there is an initializer, place it at the
        // head of the body block
        if let Some(initializer) = initializer {
            body = Statement::Block(vec![initializer, body])
        }

        Ok(body)
    }

    fn finish_call(&mut self, callee: Expression) -> Result<Expression, ParseError> {
        let mut arguments: Vec<Expression> = Vec::new();

        if !self.check(TokenType::RightParen) {
            loop {
                if arguments.len() >= 255 {
                    let token = self.peek().unwrap().clone();
                    self.error(&token, "Can't have more than 255 arguments.");
                }

                arguments.push(self.expression()?);

                if !match_tokens!(self, TokenType::Comma) {
                    break;
                }
            }
        }

        let paren = self.consume(TokenType::RightParen, "Expect ')' after arguments.")?;
        let paren = paren.unwrap().clone();

        Ok(Expression::new_call(callee, paren, arguments))
    }

    fn if_statement(&mut self) -> Result<Statement, ParseError> {
        let _ = self.consume(TokenType::LeftParen, "Expect '(' after 'if'.");
        let condition = self.expression()?;
        let _ = self.consume(TokenType::RightParen, "Expect ')' after if condition.");

        let then_branch = self.statement()?;
        let mut else_branch: Option<Box<Statement>> = None;
        if match_tokens!(self, TokenType::Else) {
            let stmt = self.statement()?;
            else_branch = Some(Box::new(stmt));
        }

        Ok(Statement::If(condition, Box::new(then_branch), else_branch))
    }

    fn is_at_end(&self) -> bool {
        if let Some(token) = self.peek() {
            token.token_type == TokenType::Eof
        } else {
            true
        }
    }

    fn or(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.and()?;

        while match_tokens!(self, TokenType::Or) {
            let operator = self.previous().unwrap().clone();
            let right = self.expression()?;
            expr = Expression::new_logical(expr, operator, right);
        }

        Ok(expr)
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
        if match_tokens!(self, TokenType::For) {
            self.for_statement()
        } else if match_tokens!(self, TokenType::If) {
            self.if_statement()
        } else if match_tokens!(self, TokenType::Print) {
            self.print_statement()
        } else if match_tokens!(self, TokenType::While) {
            self.while_statement()
        } else if match_tokens!(self, TokenType::LeftBrace) {
            let block = self.block()?;
            Ok(Statement::Block(block))
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

        self.call()
    }

    // var <name> [ = <expression> ];
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

    // while (<condition>) { <body> }
    fn while_statement(&mut self) -> Result<Statement, ParseError> {
        let _ = self.consume(TokenType::LeftParen, "Expect '(' after 'while'.");
        let condition = self.expression()?;
        let _ = self.consume(TokenType::RightParen, "Expect ')' after condition.");
        let body = self.statement()?;

        Ok(Statement::While(condition, Box::new(body)))
    }
}
