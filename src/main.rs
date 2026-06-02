use std::{fs, path::PathBuf, process};

use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    file: PathBuf,
}
#[derive(PartialEq)]
enum Token {
    CLOSE,
    OPEN,
    COLON,
    COMMA,
    STRING(String),
}

fn lex(input: &str) -> Result<Vec<Token>, String> {
    let mut token = Vec::new();
    let mut chars = input.chars().peekable();
    while let Some(&char) = chars.peek() {
        match &char {
            '{' => {
                token.push(Token::OPEN);
                chars.next();
            }
            '}' => {
                token.push(Token::CLOSE);
                chars.next();
            }
            ' ' | '\t' | '\n' | '\r' => {
                chars.next();
            }
            ':' => {
                token.push(Token::COLON);
                chars.next();
            }
            ',' => {
                token.push(Token::COMMA);
                chars.next();
            }
            '"' => {
                chars.next();
                let mut content = String::new();
                let mut closed = false;
                while let Some(&c) = chars.peek() {
                    match &c {
                        '"' => {
                            closed = true;
                            chars.next();
                            break;
                        }
                        _ => {
                            content.push(c);
                            chars.next();
                        }
                    };
                }
                if !closed {
                    return Err("Unterminated string literal".to_string());
                }
                token.push(Token::STRING(content));
            }
            _ => {
                return Err(format!("Unexpected character: '{}'", char));
            }
        };
    }
    Ok(token)
}

fn parse(tokens: &[Token]) -> Result<(), String> {
    if tokens.is_empty() {
        return Err("Empty input is invalid JSON".to_string());
    }
    if tokens[0] != Token::OPEN {
        return Err("JSON must start with '{'".to_string());
    }
    if tokens.len() == 2 && tokens[1] == Token::CLOSE {
        return Ok(());
    }
    let mut i = 1;
    while i < tokens.len() {
        if tokens[i] == Token::CLOSE {
            if i == tokens.len() - 1 {
                return Ok(());
            } else {
                return Err("Unexpected tokens after valid closing brace".to_string());
            }
        }
        match &tokens[i] {
            Token::STRING(_) => i += 1,
            _ => return Err("Expected a string key inside JSON object".to_string()),
        }
        if i >= tokens.len() || tokens[i] != Token::COLON {
            return Err("Expected ':' separator after key".to_string());
        }
        i += 1;

        if i >= tokens.len() {
            return Err("Expected value after ':'".to_string());
        }
        match &tokens[i] {
            Token::STRING(_) => i += 1,
            _ => return Err("Expected a string key inside JSON object".to_string()),
        }
        if i <= tokens.len() {
            if tokens[i] == Token::COMMA {
                i += 1;
                if i < tokens.len() && tokens[i] == Token::CLOSE {
                    return Err("Trailing comma is invalid in JSON".to_string());
                }
            } else if tokens[i] != Token::CLOSE {
                return Err("Expected ',' or closing '}' after key-value pair".to_string());
            }
        }
    }
    Err("Missing closing brace '}'".to_string())
}

fn main() {
    let args = Args::parse();
    let content = match fs::read_to_string(&args.file) {
        Ok(content) => content,
        Err(e) => {
            eprint!("Error reading file {}, {}", args.file.display(), e);
            process::exit(1);
        }
    };
    println!("{:?}", content);
    let results = lex(&content).and_then(|x| parse(&x));
    match results {
        Ok(_) => {
            println!("Valid JSON");
            process::exit(0);
        }
        Err(err) => {
            eprintln!("Invalid JSON: {}", err);
            process::exit(1);
        }
    }
}
