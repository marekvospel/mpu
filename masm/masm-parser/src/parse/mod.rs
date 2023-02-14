use crate::asl::ASLNode;
use crate::parse::math_expression::parse_math;
use crate::ParseError;
use masm_lexer::{Token, Tokens};

pub(crate) mod math_expression;

#[derive(Debug, Clone, Eq, PartialEq)]
enum ParserState {
    NumberExpr(Vec<Token>),
    None,
}

pub fn parse(input: Vec<Token>) -> Result<Vec<ASLNode>, ParseError> {
    let mut list: Vec<ASLNode> = Vec::new();
    let mut errors: Vec<ParseError> = Vec::new();
    let mut parser_state = ParserState::None;

    for token in input.into_iter() {
        match &token.token {
            /*Tokens::OpenBracket
            |*/ Tokens::NumberLiteral(_)
            | Tokens::HexNumberLiteral(_)
            | Tokens::Operator(_)
            /*| Tokens::CloseBracket*/ => match parser_state {
                ParserState::None => {
                    parser_state = ParserState::NumberExpr(vec![token]);
                }
                ParserState::NumberExpr(mut vec) => {
                    vec.push(token);
                    parser_state = ParserState::NumberExpr(vec);
                }
                _ => {}
            },
            Tokens::Whitespace => {}
            _ => {
                if let ParserState::NumberExpr(tokens) = parser_state.clone() {
                    let expression_or_lit = parse_math(tokens);

                    println!("{expression_or_lit:?}");
                    match expression_or_lit {
                        Ok(node) => list.push(node),
                        Err(err) => errors.push(err),
                    }
                }
            }
        }
    }

    // println!("state: {parser_state:?}");
    // println!("tree: {list:?}");
    Ok(list)
}
