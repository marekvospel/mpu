//! Tokenizer & Lexer of My / Mark's Assembly Language.
//!
//! There are two modules in this lib. `tokens` and `tokenize`. Inside tokens you can find the
//! output this library will give. Inside tokenize lays the function that does all the tokenization
//! and lexical analysis.
//!
//! ## Usage
//! To make importing easier, everything is imported publicly in the main module, so you can just
//! use `use masm_lexer::*;` and everything will be imported as needed.
//! ```rust
//! use masm_lexer::*;
//!
//! tokenize("...")?;
//! ```
//!
//! See [tokenize()]

pub use crate::tokenize::*;
pub use crate::tokens::*;
pub(crate) use masm_location::*;
use std::fmt::{Display, Formatter};

pub mod tokenize;
pub mod tokens;

/// An enum of possible lexical errors.
/// see [LexErrors]
#[derive(Debug, thiserror::Error, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum LexError {
    #[error("unterminated string literal at {at}")]
    UnterminatedString { at: Position },
}

/// This struct is a wrapper for `Vec<LexError>`, so the tokenize function can return multiple errors.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct LexErrors {
    pub inner: Vec<LexError>,
}

impl From<Vec<LexError>> for LexErrors {
    fn from(value: Vec<LexError>) -> Self {
        LexErrors { inner: value }
    }
}

impl Display for LexErrors {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.inner.iter().try_for_each(|e| writeln!(f, "{e}"))
    }
}

impl std::error::Error for LexErrors {}
