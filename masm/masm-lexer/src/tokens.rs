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

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum InstructionType {
    Mov,
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
    HexNumberLiteral(String),
    NumberLiteral(String),
    // Identifiers
    Identifier(String),
    // Keywords
    Instruction(InstructionType),
}

fn matches_hex(value: &str) -> bool {
    value.starts_with("0x")
        && value
            .trim_start_matches("0x")
            .chars()
            .all(|c| c.is_numeric())
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
            "mov" => Instruction(InstructionType::Mov),
            c if matches_hex(c) => HexNumberLiteral(collected.trim_start_matches("0x").to_string()),
            c if c.chars().all(|c| c.is_numeric()) => NumberLiteral(collected),
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
