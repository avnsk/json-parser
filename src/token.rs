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
