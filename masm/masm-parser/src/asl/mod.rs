use crate::math_expression::MathExpression;
use crate::*;

pub mod math_expression;

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum ASLNodes {
    MathExpr(MathExpression),
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct ASLNode {
    node: ASLNodes,
    tokens: Vec<Token>,
    loc: SourceLocation,
}
