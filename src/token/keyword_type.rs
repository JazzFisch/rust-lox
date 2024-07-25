use std::fmt::Display;

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
