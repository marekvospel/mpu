use crate::asl::math_expression::MathExpression;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub mod math_expression;

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ASLNodes {
    MathExpr(MathExpression),
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ASLNode {
    pub(crate) node: ASLNodes, /*,
                               tokens: Vec<Token>,
                               loc: SourceLocation,*/
}
