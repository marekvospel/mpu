pub(crate) use crate::*;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Tokens {
    Whitespace,
    Newline,
    Semicolon,
    Colon,
    Comma,
    Dollar,
    StringLiteral(String),
    Comment(String),
    Identifier(String),
}

impl Tokens {
    pub fn detect(collected: String) -> Self {
        use Tokens::*;
        match collected.as_str() {
            "$" => Dollar,
            ";" => Semicolon,
            ":" => Colon,
            "," => Comma,
            "\n" => Newline,
            _ => Identifier(collected),
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
