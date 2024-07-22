use std::{fmt::Display, io::{self, Write}};
use anyhow::Result;

use crate::InterpreterError;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KeywordType {
    And,
    Class,
    Else,
    False,
    For,
    Fun,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TokenType {
    // grouping tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,

    // separator tokens
    Comma,
    Dot,
    Semicolon,

    // arithmetic tokens
    Minus,
    Plus,
    Slash,
    Star,

    // comparison tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // literals
    Identifier(String),
    Keyword(KeywordType),
    String(String),
    Number(String, f64),

    // special tokens
    EOF,
}

impl Display for KeywordType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KeywordType::And => write!(f, "and"),
            KeywordType::Class => write!(f, "class"),
            KeywordType::Else => write!(f, "else"),
            KeywordType::False => write!(f, "false"),
            KeywordType::For => write!(f, "for"),
            KeywordType::Fun => write!(f, "fun"),
            KeywordType::If => write!(f, "if"),
            KeywordType::Nil => write!(f, "nil"),
            KeywordType::Or => write!(f, "or"),
            KeywordType::Print => write!(f, "print"),
            KeywordType::Return => write!(f, "return"),
            KeywordType::Super => write!(f, "super"),
            KeywordType::This => write!(f, "this"),
            KeywordType::True => write!(f, "true"),
            KeywordType::Var => write!(f, "var"),
            KeywordType::While => write!(f, "while"),
        }
    }
}

// how to incorporate this in the future?
impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenType::LeftParen => write!(f, "("),
            TokenType::RightParen => write!(f, ")"),
            TokenType::LeftBrace =>  write!(f, "{{"),
            TokenType::RightBrace => write!(f, "}}"),

            TokenType::Comma => write!(f, ","),
            TokenType::Dot => write!(f, "."),
            TokenType::Semicolon => write!(f, ";"),
            TokenType::Minus => write!(f, "-"),
            TokenType::Plus => write!(f, "+"),
            TokenType::Slash => write!(f, "/"),
            TokenType::Star => write!(f, "*"),

            TokenType::Bang => write!(f, "!"),
            TokenType::BangEqual => write!(f, "!!"),
            TokenType::Equal => write!(f, "="),
            TokenType::EqualEqual => write!(f, "=="),
            TokenType::Greater => write!(f, ">"),
            TokenType::GreaterEqual => write!(f, ">="),
            TokenType::Less => write!(f, "<"),
            TokenType::LessEqual => write!(f, "<="),

            TokenType::Identifier(_) => write!(f, "IDENTIFIER"),
            TokenType::Keyword(_) => write!(f, "KEYWORD"),
            TokenType::String(_) => write!(f, "STRING"),
            TokenType::Number(_, _) => write!(f, "NUMBER"),

            TokenType::EOF => write!(f, "EOF"),
        }
    }
}

pub struct Lexer<'a> {
    text: &'a str,
    line: i32,
    pos: Option<usize>,
    iter: std::iter::Peekable<std::str::Chars<'a>>,
    keywords: std::collections::HashMap<&'static str, KeywordType>,
    tokens: Vec<TokenType>,
}

impl<'a> Lexer<'a> {
    pub fn new(text: &'a str) -> Self {
        Self {
            text: text,
            line: 1,
            pos: None,
            iter: text.chars().peekable(),
            tokens: Vec::new(),
            // there must be a better way to do this
            keywords: std::collections::HashMap::from([
                ("and", KeywordType::And),
                ("class", KeywordType::Class),
                ("else", KeywordType::Else),
                ("false", KeywordType::False),
                ("for", KeywordType::For),
                ("fun", KeywordType::Fun),
                ("if", KeywordType::If),
                ("nil", KeywordType::Nil),
                ("or", KeywordType::Or),
                ("print", KeywordType::Print),
                ("return", KeywordType::Return),
                ("super", KeywordType::Super),
                ("this", KeywordType::This),
                ("true", KeywordType::True),
                ("var", KeywordType::Var),
                ("while", KeywordType::While),
            ]),
        }
    }

    pub fn tokenize(&mut self, print_tokens: bool) -> Result<Vec<TokenType>, InterpreterError> {
        let mut lexical_failure = false;

        while let Some(chr) = self.advance() {
            match chr {
                // grouping tokens
                '(' => self.add_token(TokenType::LeftParen),
                ')' => self.add_token(TokenType::RightParen),
                '{' => self.add_token(TokenType::LeftBrace),
                '}' => self.add_token(TokenType::RightBrace),
                // separator tokens
                ',' => self.add_token(TokenType::Comma),
                '.' => self.add_token(TokenType::Dot),
                ';' => self.add_token(TokenType::Semicolon),
                // arithmetic tokens
                '-' => self.add_token(TokenType::Minus),
                '+' => self.add_token(TokenType::Plus),
                '*' => self.add_token(TokenType::Star),
                '/' => {
                    if self.peek() == Some('/') {
                        while self.peek() != Some('\n') && self.peek() != None {
                            self.advance();
                        }
                    }
                    else {
                        self.add_token(TokenType::Slash);
                    }
                },
                // comparison tokens
                '=' => if self.match_char('=') { self.add_token(TokenType::EqualEqual) } else { self.add_token(TokenType::Equal) },
                '!' => if self.match_char('=') { self.add_token(TokenType::BangEqual) } else { self.add_token(TokenType::Bang) },
                '<' => if self.match_char('=') { self.add_token(TokenType::LessEqual) } else { self.add_token(TokenType::Less) },
                '>' => if self.match_char('=') { self.add_token(TokenType::GreaterEqual) } else { self.add_token(TokenType::Greater) },
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
                    if unmatched.is_ascii_digit() {
                        if let Ok(_) = self.number() {
                            continue;
                        }
                    }
                    // identifiers and keywords
                    if unmatched.is_ascii_alphabetic() || unmatched == '_' {
                        if let Ok(_) = self.identifier() {
                            continue;
                        }
                    }

                    self.report(self.line, &format!("Unexpected character: {}", unmatched));
                    lexical_failure = true;
                }
            }
        }

        io::stderr().flush().unwrap();
        self.add_token(TokenType::EOF);

        if print_tokens {
            for token in &self.tokens {
                self.print_token(token);
            }
        }

        if lexical_failure {
            return Err(InterpreterError::LexicalFailure);
        }

        Ok(self.tokens.clone())
    }

    pub fn print_token(&self, token: &TokenType) {
        match token {
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
            // literals
            TokenType::Identifier(ref str) => println!("IDENTIFIER {0} null", &str),
            TokenType::String(ref str) => println!("STRING \"{0}\" {0}", &str),
            TokenType::Number(str, num) => {
                // this is a hack to get the output to match the book
                if f64::trunc(*num) == *num {
                    println!("NUMBER {str} {num:.1}");
                }
                else {
                    println!("NUMBER {str} {num}");
                }
            }
            TokenType::Keyword(keyword) => {
                let keyword_str = match keyword {
                    KeywordType::And => "AND and",
                    KeywordType::Class => "CLASS class",
                    KeywordType::Else => "ELSE else",
                    KeywordType::False => "FALSE false",
                    KeywordType::For => "FOR for",
                    KeywordType::Fun => "FUN fun",
                    KeywordType::If => "IF if",
                    KeywordType::Nil => "NIL nil",
                    KeywordType::Or => "OR or",
                    KeywordType::Print => "PRINT print",
                    KeywordType::Return => "RETURN return",
                    KeywordType::Super => "SUPER super",
                    KeywordType::This => "THIS this",
                    KeywordType::True => "TRUE true",
                    KeywordType::Var => "VAR var",
                    KeywordType::While => "WHILE while",
                };
                println!("{keyword_str} null");
            }

            TokenType::EOF => println!("EOF  null"),
        }
    }

    fn add_token(&mut self, token: TokenType) {
        self.tokens.push(token);
    }

    fn advance(&mut self) -> Option<char> {
        let chr = self.iter.next();
        if chr.is_none() {
            return None;
        }

        if chr.unwrap() == '\n' {
            self.line += 1;
        }

        match self.pos {
            Some(pos) => self.pos = Some(pos + 1),
            None => self.pos = Some(0),
        }

        chr
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
            self.add_token(TokenType::Keyword(*keyword));
        }
        else {
            self.add_token(TokenType::Identifier(lexeme));
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
        self.add_token(TokenType::Number(lexeme, value));

        Ok(())
    }

    fn peek(&mut self) -> Option<char> {
        match self.iter.peek() {
            Some(chr) => Some(*chr),
            None => None,
        }
    }

    fn peek_to(&mut self, n: usize) -> Option<char> {
        self.iter.clone().nth(n)
    }

    fn pos(&self) -> usize {
        self.pos.unwrap()
    }

    fn report(&self, line: i32, message: &str) {
        writeln!(io::stderr(), "[line {}] Error: {}", line, message).unwrap();
    }

    fn string(&mut self) -> bool {
        let start = self.pos();
        while self.peek() != Some('"') && self.peek() != None {
            if self.peek() == Some('\n') {
                self.line += 1;
            }
            self.advance();
        }

        if self.peek() == None {
            self.report(self.line, "Unterminated string.");
            return false;
        }

        // consume the closing "
        self.advance();

        // trim the surrounding quotes
        let lexeme = self.get_lexeme(start + 1, self.pos() - 1);
        self.add_token(TokenType::String(lexeme));
        true
    }
}

fn is_digit(chr: Option<char>) -> bool {
    match chr {
        Some(chr) => chr.is_ascii_digit(),
        None => false,
    }
}
