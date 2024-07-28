use std::io::{self, Write};
use anyhow::Result;

use crate::{token::{token_type::TokenType, Token, TokenValue}, InterpreterError};

pub struct Lexer<'a> {
    text: &'a str,
    line: usize,
    pos: Option<usize>,
    iter: std::iter::Peekable<std::str::Chars<'a>>,
    keywords: std::collections::HashMap<&'static str, TokenType>,
    tokens: Vec<Token>,
}

fn is_digit(chr: Option<char>) -> bool {
    match chr {
        Some(chr) => chr.is_ascii_digit(),
        None => false,
    }
}

impl<'a> Lexer<'a> {
    pub fn new(text: &'a str) -> Self {
        Self {
            text,
            line: 1,
            pos: None,
            iter: text.chars().peekable(),
            tokens: Vec::new(),
            // there must be a better way to do this
            keywords: std::collections::HashMap::from([
                ("and", TokenType::And),
                ("class", TokenType::Class),
                ("else", TokenType::Else),
                ("false", TokenType::False),
                ("for", TokenType::For),
                ("fun", TokenType::Fun),
                ("if", TokenType::If),
                ("nil", TokenType::Nil),
                ("or", TokenType::Or),
                ("print", TokenType::Print),
                ("return", TokenType::Return),
                ("super", TokenType::Super),
                ("this", TokenType::This),
                ("true", TokenType::True),
                ("var", TokenType::Var),
                ("while", TokenType::While),
            ]),
        }
    }

    pub fn tokenize(&mut self, print_tokens: bool) -> Result<Vec<Token>, InterpreterError> {
        let mut lexical_failure = false;

        while let Some(chr) = self.advance() {
            match chr {
                // grouping tokens
                '(' => self.add_token(Token::from_token_type(self.line, TokenType::LeftParen)),
                ')' => self.add_token(Token::from_token_type(self.line, TokenType::RightParen)),
                '{' => self.add_token(Token::from_token_type(self.line, TokenType::LeftBrace)),
                '}' => self.add_token(Token::from_token_type(self.line, TokenType::RightBrace)),
                // separator tokens
                ',' => self.add_token(Token::from_token_type(self.line, TokenType::Comma)),
                '.' => self.add_token(Token::from_token_type(self.line, TokenType::Dot)),
                ';' => self.add_token(Token::from_token_type(self.line, TokenType::Semicolon)),
                // arithmetic tokens
                '-' => self.add_token(Token::from_token_type(self.line, TokenType::Minus)),
                '+' => self.add_token(Token::from_token_type(self.line, TokenType::Plus)),
                '*' => self.add_token(Token::from_token_type(self.line, TokenType::Star)),
                '/' => {
                    if self.peek() == Some('/') {
                        while self.peek() != Some('\n') && self.peek().is_some() {
                            self.advance();
                        }
                    }
                    else {
                        self.add_token(Token::from_token_type(self.line, TokenType::Slash));
                    }
                },
                // comparison tokens
                '=' => if self.match_char('=') { self.add_token(Token::from_token_type(self.line, TokenType::EqualEqual)) } else { self.add_token(Token::from_token_type(self.line, TokenType::Equal)) },
                '!' => if self.match_char('=') { self.add_token(Token::from_token_type(self.line, TokenType::BangEqual)) } else { self.add_token(Token::from_token_type(self.line, TokenType::Bang)) },
                '<' => if self.match_char('=') { self.add_token(Token::from_token_type(self.line, TokenType::LessEqual)) } else { self.add_token(Token::from_token_type(self.line, TokenType::Less)) },
                '>' => if self.match_char('=') { self.add_token(Token::from_token_type(self.line, TokenType::GreaterEqual)) } else { self.add_token(Token::from_token_type(self.line, TokenType::Greater)) },
                // identifiers
                // strings
                '"' => {
                    if !self.string() {
                        lexical_failure = true;
                    }
                }
                unmatched => {
                    if unmatched.is_whitespace() {
                        continue;
                    }
                    // numbers
                    if unmatched.is_ascii_digit() && self.number().is_ok() {
                        continue;
                    }
                    // identifiers and keywords
                    if (unmatched.is_ascii_alphabetic() || unmatched == '_') && self.identifier().is_ok() {
                        continue;
                    }

                    self.report(self.line, &format!("Unexpected character: {}", unmatched));
                    lexical_failure = true;
                }
            }
        }

        io::stderr().flush().unwrap();
        self.add_token(Token::new(TokenType::Eof, self.line, None, TokenValue::None));

        if print_tokens {
            for token in &self.tokens {
                token.print();
            }
        }

        if lexical_failure {
            return Err(InterpreterError::LexicalFailure);
        }

        Ok(self.tokens.clone())
    }

    fn add_token(&mut self, token: Token) {
        self.tokens.push(token);
    }

    fn advance(&mut self) -> Option<char> {
        let chr = self.iter.next()?;

        if chr == '\n' {
            self.line += 1;
        }

        match self.pos {
            Some(pos) => self.pos = Some(pos + 1),
            None => self.pos = Some(0),
        }

        Some(chr)
    }

    fn get_lexeme(&self, start: usize, end: usize) -> String {
        self.text.chars().skip(start).take(end - start + 1).collect()
    }

    fn identifier(&mut self) -> Result<(), InterpreterError> {
        let start = self.pos();
        while self.peek().map_or(false, |chr| chr.is_ascii_alphanumeric() || chr == '_') {
            self.advance();
        }

        let lexeme = self.get_lexeme(start, self.pos());
        if let Some(keyword) = self.keywords.get(lexeme.as_str()) {
            self.add_token(Token::from_token_type(self.line, *keyword));
        }
        else {
            self.add_token(Token::new_identifier(self.line, lexeme));
        }

        Ok(())
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.peek() == Some(expected) {
            self.advance();
            return true;
        }

        false
    }

    fn number(&mut self) -> Result<(), InterpreterError> {
        let start = self.pos();
        while is_digit(self.peek()) {
            self.advance();
        }

        // look for a fractional part
        if self.peek() == Some('.') && is_digit(self.peek_to(1)) {
            // consume the "."
            self.advance();

            while is_digit(self.peek()) {
                self.advance();
            }
        }

        let lexeme = self.get_lexeme(start, self.pos());
        let value = lexeme.parse::<f64>().unwrap();
        self.add_token(Token::new_number(self.line, lexeme, value));

        Ok(())
    }

    fn peek(&mut self) -> Option<char> {
        self.iter.peek().copied()
    }

    fn peek_to(&mut self, n: usize) -> Option<char> {
        self.iter.clone().nth(n)
    }

    fn pos(&self) -> usize {
        self.pos.unwrap()
    }

    fn report(&self, line: usize, message: &str) {
        eprintln!("[line {}] Error: {}", line, message);
    }

    fn string(&mut self) -> bool {
        let start = self.pos();
        while self.peek() != Some('"') && self.peek().is_some() {
            if self.peek() == Some('\n') {
                self.line += 1;
            }
            self.advance();
        }

        if self.peek().is_none() {
            self.report(self.line, "Unterminated string.");
            return false;
        }

        // consume the closing "
        self.advance();

        // trim the surrounding quotes
        let lexeme = self.get_lexeme(start + 1, self.pos() - 1);
        self.add_token(Token::new_string(self.line, lexeme));
        true
    }
}
