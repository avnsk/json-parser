use std::borrow::Cow;

#[derive(PartialEq, Debug)]
pub enum Token<'a> {
    Close,
    Open,
    Colon,
    Comma,
    String(Cow<'a, str>),
    True,
    False,
    Number(f64),
    Null,
    OpenBracket,
    CloseBracket,
}
