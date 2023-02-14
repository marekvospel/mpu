use crate::asl::math_expression::{MathExpression, MathExpressionNode};
use crate::asl::{ASLNode, ASLNodes};
use crate::parse::math_expression::MathParseToken::Untransformed;
use crate::ParseError;
use masm_lexer::{OperatorType, Token, Tokens};
use masm_location::SourceLocation;

#[derive(Debug, Clone, Eq, PartialEq)]
enum MathParseToken {
    Untransformed(Token),
    Expression(MathExpression),
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

pub(crate) fn parse_math(tokens: Vec<Token>) -> Result<ASLNode, ParseError> {
    // TODO: ignore brackets for now
    let mut transforming = to_parse_tokens(tokens.clone());

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

    // println!("{transforming:?}");
    // println!("len: {}", transforming.len());

    if transforming.len() != 1 {
        return Err(ParseError::InvalidMathExpression {
            loc: SourceLocation {
                start: tokens.first().unwrap().loc.start,
                end: tokens.last().unwrap().loc.start,
            },
        });
    }

    match transforming.get(0).unwrap() {
        MathParseToken::Expression(ex) => {
            println!("Evaluated math expression: {}", ex.evaluate());
            Ok(ASLNode {
                node: ASLNodes::MathExpr(ex.to_owned()),
            })
        }
        Untransformed(token) => {
            // return Ok(token)
            todo!()
        }
    }
}
