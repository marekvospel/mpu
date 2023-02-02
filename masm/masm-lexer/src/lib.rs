//! Tokenizer & Lexer of My / Mark's Assembly Language.
//!
//! There are two modules in this lib. `tokens` and `tokenize`. Inside tokens you can find the
//! output this library will give. Inside tokenize lays the function that does all the tokenization
//! and lexical analysis.
//!
//! ## Usage
//! To make importing easier, everything is imported publicly in the main module, so you can just
//! use `use masm_lexer::*;` and everything will be imported as needed.
//! ```
//! use masm_lexer::*;
//!
//! tokenize("...")?;
//! ```
//!
//! See [tokenize()]

use thiserror::Error;

pub use crate::position::*;
pub use crate::tokenize::*;
pub use crate::tokens::*;

pub mod position;
pub mod tokenize;
pub mod tokens;

#[derive(Debug, Error, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum LexError {
    #[error("unterminated string literal at {at}")]
    UnterminatedString { at: Position },
}
