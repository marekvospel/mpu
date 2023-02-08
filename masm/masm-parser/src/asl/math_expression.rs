use crate::math_expression::MathParseToken::Untransformed;
use crate::*;

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum MathExpressionNode {
    Expression(Box<MathExpression>),
    Literal(u32),
}

impl MathExpressionNode {
    pub fn evaluate(&self) -> u32 {
        match self {
            MathExpressionNode::Expression(expr) => expr.evaluate(),
            MathExpressionNode::Literal(lit) => *lit,
        }
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct MathExpression {
    left: MathExpressionNode,
    operator: OperatorType,
    right: MathExpressionNode,
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

#[derive(Debug, Clone, Eq, PartialEq)]
enum MathParseToken {
    Untransformed(Token),
    Expression(MathExpression),
}

impl From<Token> for MathParseToken {
    fn from(value: Token) -> Self {
        Untransformed(value)
    }
}

impl From<MathParseToken> for MathExpressionNode {
    fn from(value: MathParseToken) -> Self {
        match value {
            Untransformed(token) => {
                // MathExpressionNode::Literal()
                MathExpressionNode::Literal(match token.token {
                    Tokens::NumberLiteral(num) => num.parse().unwrap(),
                    _ => 0,
                })
            }
            MathParseToken::Expression(expr) => MathExpressionNode::Expression(Box::new(expr)),
        }
    }
}

fn to_parse_tokens(value: Vec<Token>) -> Vec<MathParseToken> {
    value.into_iter().map(Untransformed).collect()
}

#[derive(Debug, Eq, PartialEq)]
enum MathParserState {
    Term,
    Add,
}

pub(crate) fn parse_math(tokens: Vec<Token>) {
    // TODO: ignore brackets for now
    let mut transforming = to_parse_tokens(tokens);

    for state in [MathParserState::Term, MathParserState::Add] {
        let mut transformed: Vec<MathParseToken> = Vec::new();
        let mut skip = false;

        for (i, token) in transforming.clone().into_iter().enumerate() {
            if skip {
                skip = false;
                continue;
            }

            // Try to find the original tokens (Operators)
            if let Untransformed(token) = token.clone() {
                if let Tokens::Operator(op) = &token.token {
                    let is_mul = state == MathParserState::Term
                        && (*op == OperatorType::MulOperator || *op == OperatorType::DivOperator);

                    if is_mul || state == MathParserState::Add {
                        let left = transformed.clone().last().unwrap().to_owned();
                        transformed.remove(transformed.len() - 1);

                        let right = transforming.get(i + 1).unwrap().to_owned();

                        skip = true;

                        transformed.push(MathParseToken::Expression(MathExpression {
                            left: left.into(),
                            right: right.into(),
                            operator: *op,
                        }))
                    } else if !is_mul {
                        transformed.push(Untransformed(token));
                    }
                } else {
                    transformed.push(Untransformed(token));
                }
            } else {
                // Push already transformed tokens (Expressions)
                transformed.push(token);
            }
        }
        transforming = transformed
    }

    println!("{transforming:?}");
    println!("len: {}", transforming.len());

    if let MathParseToken::Expression(ex) = transforming.get(0).unwrap() {
        // println!("Evaluated math expression: {}", ex.evaluate());
    };

    // todo!()
}
