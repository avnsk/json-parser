use std::{borrow::Cow, collections::HashMap};

#[derive(Debug)]
pub enum JsonValue<'a> {
    String(Cow<'a, str>),
    Number(f64),
    Null,
    Boolean(bool),
    Array(Vec<JsonValue<'a>>),
    Object(HashMap<Cow<'a, str>, JsonValue<'a>>),
}
