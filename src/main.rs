use std::{collections::HashMap, fs, iter::Peekable, path::PathBuf, process, str::Chars};

use clap::Parser;
#[derive(Parser, Debug)]
struct Args {
    file: PathBuf,
}
#[derive(PartialEq, Debug)]
pub enum Token {
    Close,
    Open,
    Colon,
    Comma,
    String(String),
    True,
    False,
    Number(f64),
    Null,
    OpenBracket,
    CloseBracket,
}

#[derive(Debug)]
pub enum JsonValue {
    String(String),
    Number(f64),
    Null,
    Boolean(bool),
    Array(Vec<JsonValue>),
    Object(HashMap<String, JsonValue>),
}

pub struct JsonParser<'a> {
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
        let value = self.parse_value()?;
        if self.pos < self.tokens.len() {
            return Err("Unexpected tokens after JSON value".to_string());
        }
        Ok(value)
    }

    fn parse_value(&mut self) -> Result<JsonValue, String> {
        match self.peek() {
            Some(Token::String(_)) => {
                if let Some(Token::String(s)) = self.advance() {
                    Ok(JsonValue::String(s.clone()))
                } else {
                    Err("Expected string".to_string())
                }
            }
            Some(Token::Open) => self.parse_object(),
            Some(Token::OpenBracket) => self.parse_array(),
            Some(Token::True) => {
                self.advance();
                Ok(JsonValue::Boolean(true))
            }
            Some(Token::False) => {
                self.advance();
                Ok(JsonValue::Boolean(false))
            }
            Some(Token::Null) => {
                self.advance();
                Ok(JsonValue::Null)
            }
            Some(Token::Number(n)) => {
                let val = *n;
                self.advance();
                Ok(JsonValue::Number(val))
            }
            _ => Err("Expected a valid JSON value".to_string()),
        }
    }

    fn parse_object(&mut self) -> Result<JsonValue, String> {
        self.advance();
        let mut map = HashMap::new();
        while let Some(token) = self.peek() {
            if *token == Token::Close {
                self.advance();
                return Ok(JsonValue::Object(map));
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

    fn parse_array(&mut self) -> Result<JsonValue, String> {
        self.advance();
        let mut elements = Vec::new();
        if let Some(Token::CloseBracket) = self.peek() {
            self.advance();
            return Ok(JsonValue::Array(elements));
        }
        loop {
            let value = self.parse_value()?;
            elements.push(value);
            match self.peek() {
                Some(Token::CloseBracket) => {
                    self.advance();
                    return Ok(JsonValue::Array(elements));
                }
                Some(Token::Comma) => {
                    self.advance();
                    if let Some(Token::CloseBracket) = self.peek() {
                        return Err("Trailing comma in array is invalid".to_string());
                    }
                }
                _ => return Err("Expected ',' or ']'".to_string()),
            }
        }
    }
}

pub fn lex(input: &str) -> Result<Vec<Token>, String> {
    let mut token = Vec::new();
    let mut chars = input.chars().peekable();
    while let Some(&char) = chars.peek() {
        match char {
            '[' => {
                token.push(Token::OpenBracket);
                chars.next();
            }
            ']' => {
                token.push(Token::CloseBracket);
                chars.next();
            }
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
                chars.next(); // Consume opening quote
                let mut content = String::new();
                let mut closed = false;

                while let Some(&c) = chars.peek() {
                    match c {
                        '"' => {
                            closed = true;
                            chars.next();
                            break;
                        }
                        '\\' => {
                            chars.next();
                            if let Some(escaped) = chars.next() {
                                match escaped {
                                    '"' => content.push('"'),
                                    '\\' => content.push('\\'),
                                    '/' => content.push('/'),
                                    'b' => content.push('\x08'),
                                    'f' => content.push('\x0c'),
                                    'n' => content.push('\n'),
                                    'r' => content.push('\r'),
                                    't' => content.push('\t'),
                                    'u' => {
                                        let mut hex = String::new();
                                        for _ in 0..4 {
                                            if let Some(h) = chars.next() {
                                                hex.push(h);
                                            }
                                        }
                                        if let Ok(code) = u32::from_str_radix(&hex, 16) {
                                            if let Some(ch) = char::from_u32(code) {
                                                content.push(ch);
                                            } else {
                                                return Err(
                                                    "Invalid unicode escape value".to_string()
                                                );
                                            }
                                        } else {
                                            return Err(
                                                "Invalid unicode escape hex syntax".to_string()
                                            );
                                        }
                                    }
                                    _ => {
                                        return Err(format!(
                                            "Invalid escape sequence: \\{}",
                                            escaped
                                        ));
                                    }
                                }
                            } else {
                                return Err("Unterminated string escape sequence".to_string());
                            }
                        }
                        _ => {
                            if (c as u32) < 32 {
                                return Err(format!(
                                    "Unescaped control character (U+{:04X}) found inside string literal",
                                    c as u32
                                ));
                            }
                            content.push(c);
                            chars.next();
                        }
                    }
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
                    if c.is_ascii_digit()
                        || c == '-'
                        || c == '.'
                        || c == 'e'
                        || c == 'E'
                        || c == '+'
                    {
                        num_str.push(chars.next().unwrap());
                    } else {
                        break;
                    }
                }
                if num_str.starts_with("0") && num_str.len() > 1 && !num_str.starts_with("0.") {
                    return Err(format!("Invalid leading zero in number: {}", num_str));
                }
                if num_str.starts_with("-0") && num_str.len() > 2 && !num_str.starts_with("-0.") {
                    return Err(format!(
                        "Invalid leading zero in negative number: {}",
                        num_str
                    ));
                }
                if num_str.starts_with('.') || num_str.starts_with("-.") {
                    return Err(format!("Numbers must start with a digit: {}", num_str));
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn run_test_file(path: &std::path::Path) -> bool {
        let content = match fs::read_to_string(path) {
            Ok(c) => c,
            Err(_) => return false,
        };

        let tokens = match lex(&content) {
            Ok(t) => t,
            Err(_) => return false,
        };

        let mut parser = JsonParser::new(&tokens);
        parser.parse().is_ok()
    }

    #[test]
    fn test_all_json_files() {
        let dir_path = "test/";

        if let Ok(paths) = fs::read_dir(dir_path) {
            for path in paths {
                let p = path.expect("Failed to read path").path();
                let p_str = p.to_str().unwrap();

                let is_expected_to_fail = p_str.contains("invalid") || p_str.contains("fail");

                let result = run_test_file(&p);

                if is_expected_to_fail {
                    assert!(!result, "File should have been invalid but passed: {:?}", p);
                } else {
                    assert!(result, "File should have been valid but failed: {:?}", p);
                }
            }
        }
    }
}
