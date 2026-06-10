use std::{fs, iter::Peekable, path::PathBuf, process, str::Chars};

use clap::{Parser, builder::Str};

#[derive(Parser, Debug)]
struct Args {
    file: PathBuf,
}
#[derive(PartialEq, Debug)]
enum Token {
    Close,
    Open,
    Colon,
    Comma,
    String(String),
    True,
    False,
    Number(f64),
    Null,
}

#[derive(Debug)]
enum JsonValue {
    String(String),
    Number(f64),
    Null,
    Boolean(bool),
}

struct JsonParser<'a> {
    tokens: &'a [Token],
    pos: usize,
}

impl<'a> JsonParser<'a> {
    fn new(tokens: &'a [Token]) -> Self {
        Self { tokens, pos: 0 }
    }
    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }
    fn advance(&mut self) -> Option<&Token> {
        let token = self.tokens.get(self.pos);
        self.pos += 1;
        token
    }
    fn parse(&mut self) -> Result<JsonValue, String> {
        match self.peek() {
            Some(Token::Open) => self.parse_object(),
            _ => Err("Invalid JSON: Expected '{'".to_string()),
        }
    }

    fn parse_object(&mut self) -> Result<JsonValue, String> {
        self.advance();
        while let Some(token) = self.peek() {
            if *token == Token::Close {
                self.advance();
                return Ok(JsonValue::Null);
            }
        }
        Err("Expected '}'".to_string())
    }
}

fn lex(input: &str) -> Result<Vec<Token>, String> {
    let mut token = Vec::new();
    let mut chars = input.chars().peekable();
    while let Some(&char) = chars.peek() {
        match char {
            '{' => {
                token.push(Token::Open);
                chars.next();
            }
            '}' => {
                token.push(Token::Close);
                chars.next();
            }
            c if c.is_whitespace() => {
                chars.next();
            }

            ':' => {
                token.push(Token::Colon);
                chars.next();
            }
            ',' => {
                token.push(Token::Comma);
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
                token.push(Token::String(content));
            }
            't' => {
                chars.next();
                if match_keyword(&mut chars, "rue") {
                    token.push(Token::True);
                } else {
                    return Err("Invalid token: expected 'true'".to_string());
                }
            }
            'f' => {
                chars.next();
                if match_keyword(&mut chars, "alse") {
                    token.push(Token::False);
                } else {
                    return Err("Invalid token: expected 'false'".to_string());
                }
            }
            'n' => {
                chars.next();
                if match_keyword(&mut chars, "ull") {
                    token.push(Token::Null);
                } else {
                    return Err("Invalid token: expected 'null'".to_string());
                }
            }
            '0'..='9' | '-' => {
                let mut num_str = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_digit(10) || c == '-' || c == '.' {
                        num_str.push(chars.next().unwrap());
                    } else {
                        break;
                    }
                }
                let num = num_str
                    .parse::<f64>()
                    .map_err(|_| format!("Invalid Number {}", num_str))?;
                token.push(Token::Number(num));
            }

            _ => {
                return Err(format!("Unexpected character: '{}'", char));
            }
        };
    }
    println!("{:?}", token);
    Ok(token)
}

fn parse(tokens: &[Token]) -> Result<(), String> {
    if tokens.is_empty() {
        return Err("Empty input is invalid JSON".to_string());
    }
    if tokens[0] != Token::Open {
        return Err("JSON must start with '{'".to_string());
    }
    if tokens.len() == 2 && tokens[1] == Token::Close {
        return Ok(());
    }
    let mut i = 1;
    while i < tokens.len() {
        if tokens[i] == Token::Close {
            if i == tokens.len() - 1 {
                return Ok(());
            } else {
                return Err("Unexpected tokens after valid closing brace".to_string());
            }
        }
        match &tokens[i] {
            Token::String(_) => i += 1,
            _ => return Err("Expected a string key inside JSON object".to_string()),
        }
        if i >= tokens.len() || tokens[i] != Token::Colon {
            return Err("Expected ':' separator after key".to_string());
        }
        i += 1;

        if i >= tokens.len() {
            return Err("Expected value after ':'".to_string());
        }
        match &tokens[i] {
            Token::String(_) => i += 1,
            _ => return Err("Expected a string key inside JSON object".to_string()),
        }
        if i <= tokens.len() {
            if tokens[i] == Token::Comma {
                i += 1;
                if i < tokens.len() && tokens[i] == Token::Close {
                    return Err("Trailing comma is invalid in JSON".to_string());
                }
            } else if tokens[i] != Token::Close {
                return Err("Expected ',' or closing '}' after key-value pair".to_string());
            }
        }
    }
    Err("Missing closing brace '}'".to_string())
}

fn match_keyword(chars: &mut Peekable<Chars<'_>>, expected: &str) -> bool {
    for c in expected.chars() {
        if Some(c) != chars.next() {
            return false;
        }
    }
    true
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
