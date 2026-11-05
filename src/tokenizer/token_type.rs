use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
pub enum TokenType {
    NumberLiteral,
    Identifier,
    Equal,
    Plus,
    Minus,
    Star,
    Slash,
    LeftParen,
    RightParen,
    NewLine,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            TokenType::NumberLiteral => String::from("NumberLiteral"),
            TokenType::Identifier => String::from("Identifier"),
            TokenType::Equal => String::from("Equal"),
            TokenType::Plus => String::from("Plus"),
            TokenType::Minus => String::from("Minus"),
            TokenType::Star => String::from("Star"),
            TokenType::Slash => String::from("Slash"),
            TokenType::LeftParen => String::from("LeftParen"),
            TokenType::RightParen => String::from("RightParen"),
            TokenType::NewLine => String::from("NewLine"),
        };

        write!(f, "{str}")
    }
}
