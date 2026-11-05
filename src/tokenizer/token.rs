use super::TokenType;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct Token {
    pub line: i32,
    pub token_type: TokenType,
    pub lexeme: String,
}

impl Token {
    fn get_lexeme_formatted(&self) -> String {
        if self.lexeme.len() == 1 && self.lexeme.chars().nth(0) == Some('\n') {
            String::from("'\\n'")
        } else {
            String::from(format!("\"{}\"", self.lexeme))
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "line: {}, token type: {}, lexeme: {}",
            self.line,
            self.token_type,
            self.get_lexeme_formatted()
        )
    }
}
