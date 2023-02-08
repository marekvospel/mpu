use crate::math_expression::parse_math;
use crate::*;

#[derive(Debug, Clone, Eq, PartialEq)]
enum ParserState {
    NumberExpr(Vec<Token>),
    None,
}

pub fn parse(input: Vec<Token>) {
    let mut list: Vec<ASLNode> = Vec::new();
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
                    parse_math(tokens);
                }
            }
        }
    }

    // println!("state: {parser_state:?}");
    // println!("tree: {list:?}");
}
