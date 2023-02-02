pub(crate) use crate::*;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
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
    pub fn detect(src: String, collected: String, loc: SourceLocation) -> Self {
        use Tokens::*;
        let token = match collected.as_str() {
            "$" => Dollar,
            ";" => Semicolon,
            "," => Comma,
            "\n" => Newline,
            _ => Literal(collected),
        };

        Token { token, src, loc }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Token {
    pub token: Tokens,
    pub src: String,
    pub loc: SourceLocation,
}
