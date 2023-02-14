//! Abstract Syntax List parser for My/Mark's Assembly Language
//!

pub(crate) use masm_lexer::*;
pub(crate) use masm_location::*;
use thiserror::Error;

pub mod asl;
pub mod parse;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Invalid math expression at {loc}")]
    InvalidMathExpression { loc: SourceLocation },
}
