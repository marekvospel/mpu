use crate::math_expression::MathParseToken::Untransformed;
use crate::*;

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum MathExpressionNode {
    Expression(Box<MathExpression>),
    Literal(u32),
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct MathExpression {
    left: MathExpressionNode,
    operator: OperatorType,
    right: MathExpressionNode,
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
    Calc,
}

pub(crate) fn parse_math(tokens: Vec<Token>) {
    // ignore brackets for now
    let mut transforming = to_parse_tokens(tokens);

    for state in [MathParserState::Term, MathParserState::Calc] {
        let mut transformed = Vec::new();
        for (i, token) in transforming.clone().into_iter().enumerate() {
            if let Untransformed(token) = token.clone() {
                if let Tokens::Operator(op) = &token.token {
                    if state == MathParserState::Term
                        && (*op == OperatorType::MulOperator || *op == OperatorType::DivOperator)
                    {
                        let left = match transformed.clone().last() {
                            Some(token) => {
                                if !transformed.is_empty() {
                                    transformed.remove(transformed.len() - 1);
                                }
                                token
                            }
                            None => transforming.get(i - 1).unwrap(),
                        }
                        .to_owned();

                        let right = transforming.get(i + 1).unwrap().to_owned();

                        println!("left: {left:?}\nright: {right:?}");

                        transformed.push(MathParseToken::Expression(MathExpression {
                            left: left.into(),
                            right: right.into(),
                            operator: *op,
                        }))
                    }
                }
            } else {
                transformed.push(token)
            }
        }
        transforming = transformed
    }

    println!("{transforming:?}");
}
