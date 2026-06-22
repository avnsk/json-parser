use crate::token::Token;
use std::{borrow::Cow, iter::Peekable, str::CharIndices};

pub fn lex<'a>(input: &'a str) -> Result<Vec<Token<'a>>, String> {
    let mut token = Vec::new();
    let mut chars = input.char_indices().peekable();
    while let Some(&(idx, char)) = chars.peek() {
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
                let start_index = idx;
                chars.next(); // Consume opening quote
                let mut content = String::new();
                let mut closed = false;
                let mut end_index = start_index;
                let mut needs_unescaping = false;

                while let Some(&(idx, c)) = chars.peek() {
                    match c {
                        '"' => {
                            closed = true;
                            end_index = idx;
                            chars.next();
                            break;
                        }
                        '\\' => {
                            chars.next();
                            needs_unescaping = true;
                            if let Some((_, escaped)) = chars.next() {
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
                                            if let Some((_, h)) = chars.next() {
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
                if needs_unescaping {
                    token.push(Token::String(Cow::Owned(content)));
                } else {
                    token.push(Token::String(Cow::Borrowed(
                        &input[start_index + 1..end_index],
                    )));
                }
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
                while let Some(&(_, c)) = chars.peek() {
                    if c.is_ascii_digit()
                        || c == '-'
                        || c == '.'
                        || c == 'e'
                        || c == 'E'
                        || c == '+'
                    {
                        num_str.push(chars.next().unwrap().1);
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

fn match_keyword(chars: &mut Peekable<CharIndices<'_>>, expected: &str) -> bool {
    for c in expected.chars() {
        match chars.next() {
            Some((_, actual_char)) if actual_char == c => continue,
            _ => return false,
        }
    }
    true
}
