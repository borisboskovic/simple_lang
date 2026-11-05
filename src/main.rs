use std::fmt::{Debug, Formatter};
use std::fs::File;
use std::io::Read;

// #[derive(Debug)]
enum TokenType {
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

impl Debug for TokenType {
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

        write!(f, "{:16}", str)
    }
}

#[derive(Debug)]
struct Token {
    // TODO: Remove all instances of #[allow(dead_code)]
    #[allow(dead_code)]
    line: i32,
    #[allow(dead_code)]
    token_type: TokenType,
    #[allow(dead_code)]
    lexeme: String,
}

fn tokenize(source_code: &str) -> Vec<Token> {
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

fn main() {
    // let source_path = "./example_2.bb";
    let source_path = "./example_1.bb";
    // let source_path = "./invalid_example_1.bb";

    let source = read_source(source_path).unwrap();
    let tokens = tokenize(&source);

    for token in &tokens {
        println!("{:?}", token);
    }
}

fn read_source(path: &str) -> std::io::Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
