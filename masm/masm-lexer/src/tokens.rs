#[cfg(feature = "serde")]
use serde::{Serialize,Deserialize};

#[derive(Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Tokens {
    Whitespace,
    Newline,
    Semicolon,
    Comma,
    Dollar,
    SingleQuote,
    SingleQuoteString(String),
    DoubleQuote,
    DoubleQuoteString(String),
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
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Token {
    pub token: Tokens,
    pub src: String,
    pub start: usize,
    pub end: usize,
}
