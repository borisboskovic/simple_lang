mod tokenizer;

use std::fs::File;
use std::io::Read;
use tokenizer::tokenize;

fn main() {
    let source_path = "./example_2.bb";
    // let source_path = "./example_1.bb";
    // let source_path = "./invalid_example_1.bb";

    let source = read_source(source_path).unwrap();
    let tokens = tokenize(&source);

    for token in &tokens {
        let mut lexeme_formatted = token.lexeme.clone();
        if lexeme_formatted.len() == 1 && lexeme_formatted.chars().nth(0) == Some('\n') {
            lexeme_formatted = String::from("\\n");
        }

        println!(
            "line: {}, token type: {}, lexeme: \"{}\"",
            token.line, token.token_type, lexeme_formatted
        );
    }
}

fn read_source(path: &str) -> std::io::Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
