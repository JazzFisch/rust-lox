use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq)]
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
    Identifier,
    String,
    Number,

    // keywords
    And, Class, Else, False, Fun, For, If, Nil, Or,
    Print, Return, Super, This, True, Var, While,

    // special tokens
    Eof,
}

impl TokenType {
    // pub fn is_keyword(&self) -> bool {
    //     match self {
    //         TokenType::And | TokenType::Class | TokenType::Else | TokenType::False |
    //         TokenType::Fun | TokenType::For | TokenType::If | TokenType::Nil |
    //         TokenType::Or | TokenType::Print | TokenType::Return | TokenType::Super |
    //         TokenType::This | TokenType::True | TokenType::Var | TokenType::While => true,
    //         _ => false,
    //     }
    // }
}

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
            TokenType::BangEqual => write!(f, "!="),
            TokenType::Equal => write!(f, "="),
            TokenType::EqualEqual => write!(f, "=="),
            TokenType::Greater => write!(f, ">"),
            TokenType::GreaterEqual => write!(f, ">="),
            TokenType::Less => write!(f, "<"),
            TokenType::LessEqual => write!(f, "<="),
            TokenType::Identifier => write!(f, "IDENTIFIER"),
            TokenType::String => write!(f, "STRING"),
            TokenType::Number => write!(f, "NUMBER"),
            TokenType::Eof => write!(f, "EOF"),
            _ => write!(f, "KEYWORD"),
        }
    }
}
