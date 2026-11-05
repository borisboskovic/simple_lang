mod token;
mod token_type;

pub use token::Token;
pub use token_type::TokenType;

pub fn tokenize(source_code: &str) -> Vec<Token> {
    let mut line = 1;
    let mut position = 0;
    let mut result = Vec::new();

    while position < source_code.len() {
        let current_char = source_code.chars().nth(position).unwrap();

        match current_char {
            '=' => result.push(Token {
                line,
                token_type: TokenType::Equal,
                lexeme: "=".to_string(),
            }),
            '+' => result.push(Token {
                line,
                token_type: TokenType::Plus,
                lexeme: "+".to_string(),
            }),
            '-' => result.push(Token {
                line,
                token_type: TokenType::Minus,
                lexeme: "i".to_string(),
            }),
            '*' => result.push(Token {
                line,
                token_type: TokenType::Star,
                lexeme: "*".to_string(),
            }),
            '/' => result.push(Token {
                line,
                token_type: TokenType::Slash,
                lexeme: "/".to_string(),
            }),
            '(' => result.push(Token {
                line,
                token_type: TokenType::LeftParen,
                lexeme: "(".to_string(),
            }),
            ')' => result.push(Token {
                line,
                token_type: TokenType::RightParen,
                lexeme: ")".to_string(),
            }),
            '\n' => {
                result.push(Token {
                    line,
                    token_type: TokenType::NewLine,
                    lexeme: '\n'.to_string(),
                });
                line += 1;
            }
            x if x.is_digit(10) => {
                let mut number_lexeme = x.to_string();
                position += 1;

                while position < source_code.len() {
                    let next_char = source_code.chars().nth(position).unwrap();
                    if next_char.is_digit(10) {
                        number_lexeme.push(next_char);
                        position += 1;
                    } else {
                        break;
                    }
                }

                result.push(Token {
                    line,
                    token_type: TokenType::NumberLiteral,
                    lexeme: number_lexeme,
                });

                continue;
            }
            x if x.is_ascii_alphabetic() || x == '_' => {
                let mut lexeme = x.to_string();
                position += 1;

                while position < source_code.len() {
                    let next_char = source_code.chars().nth(position).unwrap();
                    if next_char.is_ascii_alphanumeric() || next_char == '_' {
                        lexeme.push(next_char);
                        position += 1;
                    } else {
                        break;
                    }
                }

                result.push(Token {
                    line,
                    token_type: TokenType::Identifier,
                    lexeme: lexeme.to_string(),
                });

                continue;
            }
            ' ' => {}
            '\r' => {}
            char => panic!("Invalid character at line {}: {}", line, char),
        }

        position += 1;
    }

    result
}
