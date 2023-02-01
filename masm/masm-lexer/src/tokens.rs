#[derive(Debug, Eq, PartialEq)]
pub enum Tokens {
    Whitespace,
    Newline,
    Semicolon,
    Comma,
    Dollar,
    DoubleQuoteString(String),
    SingleQuoteString(String),
    Comment(String),
    Literal(String),
}

impl Token {
    pub fn detect<S: Into<String>>(src: S, index: usize) -> Self {
        let src = src.into();

        use Tokens::*;
        let token = match src.as_str() {
            src => Literal(src.into()),
        };

        Token {
            token,
            start: index - src.len(),
            end: index - 1,
            src,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Token {
    pub token: Tokens,
    pub src: String,
    pub start: usize,
    pub end: usize,
}
