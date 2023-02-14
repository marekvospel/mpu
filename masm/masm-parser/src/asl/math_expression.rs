use crate::*;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// This is the node type of MathExpression binary tree.
///
/// See [MathExpression]
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum MathExpressionNode {
    Expression(Box<MathExpression>),
    Literal(u32),
}

/// This expression is a binary tree with operator.
///
/// Both `left` and `right` side can either be another `MathExpression` or a `Literal`, which is
/// an unsigned 32 bit integer. The expression can be evaluated using the [MathExpression::evaluate]
/// function.
///
/// See [MathExpressionNode]
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MathExpression {
    pub left: MathExpressionNode,
    pub operator: OperatorType,
    pub right: MathExpressionNode,
}

impl MathExpressionNode {
    pub fn evaluate(&self) -> u32 {
        match self {
            MathExpressionNode::Expression(expr) => expr.evaluate(),
            MathExpressionNode::Literal(lit) => *lit,
        }
    }
}

impl MathExpression {
    pub fn evaluate(&self) -> u32 {
        match self.operator {
            OperatorType::AddOperator => self.left.evaluate() + self.right.evaluate(),
            OperatorType::SubOperator => self.left.evaluate() - self.right.evaluate(),
            OperatorType::MulOperator => self.left.evaluate() * self.right.evaluate(),
            OperatorType::DivOperator => self.left.evaluate() / self.right.evaluate(),
        }
    }
}
