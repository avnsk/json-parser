use std::{collections::HashMap, fs, iter::Peekable, path::PathBuf, process, str::Chars};

use clap::Parser;

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

    fn parse_value(&mut self) -> Result<JsonValue, String> {
        match self.advance() {
            Some(Token::String(s)) => Ok(JsonValue::String(s.clone())),
            Some(Token::True) => Ok(JsonValue::Boolean(true)),
            Some(Token::False) => Ok(JsonValue::Boolean(false)),
            Some(Token::Null) => Ok(JsonValue::Null),
            Some(Token::Number(n)) => Ok(JsonValue::Number(*n)),
            _ => Err("Expected a valid JSON value".to_string()),
        }
    }

    fn parse_object(&mut self) -> Result<JsonValue, String> {
        self.advance();
        let mut map = HashMap::new();
        while let Some(token) = self.peek() {
            if *token == Token::Close {
                self.advance();
                return Ok(JsonValue::Null);
            }
            let key = match self.advance() {
                Some(Token::String(s)) => s.clone(),
                _ => return Err("Expected key string".to_string()),
            };
            match self.advance() {
                Some(Token::Colon) => (),
                _ => return Err("Expected ':'".to_string()),
            };

            let value = self.parse_value()?;
            map.insert(key, value);
            match self.peek() {
                Some(Token::Comma) => {
                    self.advance();
                    if let Some(Token::Close) = self.peek() {
                        return Err("Trailing comma is invalid".to_string());
                    }
                }
                Some(Token::Close) => (),
                _ => return Err("Expected ',' or '}'".to_string()),
            };
        }
        Err("Unterminated object".to_string())
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
    let tokens = match lex(&content) {
        Ok(a) => a,
        Err(e) => {
            eprintln!("Invalid JSON: {}", e);
            process::exit(1);
        }
    };
    let mut parser = JsonParser::new(&tokens);
    match parser.parse() {
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
