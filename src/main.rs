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
}

fn lex(input: &str) -> Result<Vec<Token>, String> {
    let mut token = Vec::new();
    let mut chars = input.chars().peekable();
    while let Some(&char) = chars.peek() {
        match char {
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
    if tokens.len() == 2 && tokens[0] == Token::OPEN && tokens[1] == Token::CLOSE {
        return Ok(());
    }

    Err("Invalid JSON structure for Step 1".to_string())
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
