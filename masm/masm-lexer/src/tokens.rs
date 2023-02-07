pub(crate) use crate::*;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum OperatorType {
    AddOperator,
    SubOperator,
    MulOperator,
    DivOperator,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Tokens {
    // Whitespace
    Whitespace,
    Newline,
    // Comment
    Comment(String),
    // Separators
    Colon,
    Comma,
    Dollar,
    OpenBracket,
    CloseBracket,
    // Operators
    Operator(OperatorType),
    // Literals
    StringLiteral(String),
    // Identifiers
    Identifier(String),
}

impl Tokens {
    pub fn detect(collected: String) -> Self {
        use Tokens::*;
        match collected.as_str() {
            // Separators
            ":" => Colon,
            "," => Comma,
            "$" => Dollar,
            "(" => OpenBracket,
            ")" => CloseBracket,
            // Operators
            "+" => Operator(OperatorType::AddOperator),
            "-" => Operator(OperatorType::SubOperator),
            "*" => Operator(OperatorType::MulOperator),
            "/" => Operator(OperatorType::DivOperator),
            "\n" => Newline,
            _ => Identifier(collected),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Token {
    pub token: Tokens,
    pub src: String,
    pub loc: SourceLocation,
}
