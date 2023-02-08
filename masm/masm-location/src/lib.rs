use std::fmt::{Display, Formatter};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SourceLocation {
    pub start: Position,
    pub end: Position,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Position {
    pub column: usize,
    pub line: usize,
    pub offset: usize,
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}

impl Default for Position {
    fn default() -> Self {
        Position {
            column: 0,
            line: 1,
            offset: 0,
        }
    }
}

pub fn get_location_from_source<S: Into<String>>(source: S, loc: SourceLocation) -> String {
    let source = source.into();

    source
        .chars()
        .skip(loc.start.offset)
        .take(loc.end.offset - loc.start.offset + 1)
        .collect()
}
