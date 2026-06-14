use std::collections::HashMap;

#[derive(Debug)]
pub enum JsonValue {
    String(String),
    Number(f64),
    Null,
    Boolean(bool),
    Array(Vec<JsonValue>),
    Object(HashMap<String, JsonValue>),
}
