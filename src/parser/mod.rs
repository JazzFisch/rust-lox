use crate::lexer::{KeywordType, TokenType};

pub enum Expression {
    Binary(Box<Expression>, TokenType, Box<Expression>),
    Grouping(Box<Expression>),
    Literal(TokenType),
    Unary(TokenType, Box<Expression>),
}

pub struct AstPrinter;

impl AstPrinter {
    pub fn print(&self, expr: &Expression) -> String {
        match expr {
            Expression::Binary(left, operator, right) => {
                format!("({} {} {})", operator, self.print(left), self.print(right))
            },
            Expression::Grouping(expr) => {
                format!("(group {})", self.print(expr))
            },
            Expression::Literal(token) => {
                match token {
                    TokenType::Number(_, num) => {
                        // this is a hack to get the output to match the book
                        if f64::trunc(*num) == *num {
                            format!("{num:.1}")
                        }
                        else {
                            format!("{num}")
                        }
                    },
                    TokenType::String(s) => s.clone(),
                    TokenType::Keyword(keyword) => format!("{keyword}"),
                    _ => "".to_string(),
                }
            },
            Expression::Unary(operator, right) => {
                format!("({} {})", operator, self.print(right))
            },
        }
    }
}

pub struct Parser {
    tokens: Vec<TokenType>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<TokenType>) -> Self {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Expression {
        self.expression()
    }

    fn advance(&mut self) -> Option<&TokenType> {
        if let Some(token) = self.tokens.get(self.current) {
            self.current += 1;
            return Some(token);
        }

        None
    }

    fn check(&self, token: &TokenType) -> bool {
        if let Some(next_token) = self.peek() {
            return *next_token == *token;
        }

        false
    }

    fn comparison(&mut self) -> Expression {
        let mut expr = self.term();

        while self.match_tokens(&[TokenType::Greater, TokenType::GreaterEqual, TokenType::Less, TokenType::LessEqual]) {
            let operator = self.previous().unwrap().clone();
            let right = self.term();
            expr = Expression::Binary(Box::new(expr), operator, Box::new(right));
        }

        expr
    }

    fn consume(&mut self, token: TokenType, _: &str) -> Option<&TokenType> {
        if self.check(&token) {
            return self.advance();
        }

        todo!("Error handling");
    }

    fn equality(&mut self) -> Expression {
        let mut expr = self.comparison();

        while self.match_tokens(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous().unwrap().clone();
            let right = self.comparison();
            expr = Expression::Binary(Box::new(expr), operator, Box::new(right));
        }

        expr
    }

    fn expression(&mut self) -> Expression {
        self.equality()
    }

    fn factor(&mut self) -> Expression {
        let mut expr = self.unary();

        while self.match_tokens(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous().unwrap().clone();
            let right = self.unary();
            expr = Expression::Binary(Box::new(expr), operator, Box::new(right));
        }

        expr
    }

    fn peek(&self) -> Option<&TokenType> {
        self.tokens.get(self.current)
    }

    fn match_tokens(&mut self, tokens: &[TokenType]) -> bool {
        for token in tokens {
            if self.check(token) {
                self.advance();
                return true;
            }
        }

        false
    }

    fn previous(&self) -> Option<&TokenType> {
        self.tokens.get(self.current - 1)
    }

    fn primary(&mut self) -> Expression {
        if self.match_tokens(&[TokenType::Keyword(KeywordType::False), TokenType::Keyword(KeywordType::True), TokenType::Keyword(KeywordType::Nil)]) {
            let previous = self.previous().unwrap().clone();
            return Expression::Literal(previous);
        }

        // special case for number and string literals
        let peeked = self.peek().cloned();
        match peeked {
            Some(TokenType::Number(_, _)) | Some(TokenType::String(_)) => {
                self.advance();
                return Expression::Literal(peeked.unwrap().clone());
            },
            _ => (),
        }

        if self.match_tokens(&[TokenType::LeftParen]) {
            let expr = self.expression();
            self.consume(TokenType::RightParen, "Expect ')' after expression.");
            return Expression::Grouping(Box::new(expr));
        }

        panic!("Unexpected token: {:?}", self.peek());
    }

    fn term(&mut self) -> Expression {
        let mut expr = self.factor();

        while self.match_tokens(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous().unwrap().clone();
            let right = self.factor();
            expr = Expression::Binary(Box::new(expr), operator, Box::new(right));
        }

        expr
    }

    fn unary(&mut self) -> Expression {
        if self.match_tokens(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous().unwrap().clone();
            let right = self.unary();
            return Expression::Unary(operator, Box::new(right));
        }

        self.primary()
    }
}


// TODO: someday figure out how to make this macro work
// macro_rules! match_tokens {
//     ($self:expr, $($token:expr),* $(,)?) => {{
//         $(
//             if $self.check(&$token) {
//                 $self.advance();
//                 true
//             } else
//         )* {
//             false
//         }
//     }};
// }

// macro_rules! match_tokens {
//     ($self:expr, $($token:expr),* $(,)?) => {{
//         let token = $self.peek();
//         if token.is_none() {
//             return false;
//         }

//         match token.unwrap() {
//             $(TokenType::$token)|* => {
//                 $self.advance();
//                 true
//             },
//             _ => false
//         }
//     }};
// }
