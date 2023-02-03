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
    SingleQuoteString(String),
    DoubleQuoteString(String),
    Comment(String),
    Literal(String),
}

impl Tokens {
    pub fn detect(collected: String) -> Self {
        use Tokens::*;
        match collected.as_str() {
            "$" => Dollar,
            ";" => Semicolon,
            "," => Comma,
            "\n" => Newline,
            _ => Literal(collected),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Token {
    pub token: Tokens,
    pub src: String,
    pub loc: SourceLocation,
}
