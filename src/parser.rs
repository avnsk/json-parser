use crate::ast::JsonValue;
use crate::token::Token;
use std::collections::HashMap;

pub struct JsonParser<'a> {
    tokens: &'a [Token],
    pos: usize,
}

impl<'a> JsonParser<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
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
    pub fn parse(&mut self) -> Result<JsonValue, String> {
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
