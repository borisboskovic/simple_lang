use super::TokenType;

#[derive(Debug)]
pub struct Token {
    pub line: i32,
    pub token_type: TokenType,
    pub lexeme: String,
}
