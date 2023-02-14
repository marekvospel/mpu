use crate::asl::ASLNode;
use crate::ParseError;
use masm_lexer::{InstructionType, Token, Tokens};

pub(crate) mod math_expression;

/*
#[derive(Debug, Clone, Eq, PartialEq)]
enum ParserState {
    NumberExpr(Vec<Token>),
    None,
}*/

fn lines(input: Vec<Token>) -> Vec<Vec<Token>> {
    let mut out = Vec::new();
    let mut temp = Vec::new();

    for token in input {
        if let Tokens::Newline = token.token {
            out.push(temp);
            temp = Vec::new();
            continue;
        }

        temp.push(token)
    }

    if !temp.is_empty() {
        out.push(temp);
    }

    out
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum InstructionArgument {
    Label(String),
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Default)]
enum LineParserState {
    #[default]
    None,
    Instruction {
        instruction: InstructionType,
        args: Vec<InstructionArgument>,
        comma: bool,
    },
}

fn parse_line(line: Vec<Token>) -> Result<Option<ASLNode>, ParseError> {
    let mut out = None;
    let mut state = LineParserState::default();

    for token in line {
        match &mut state {
            LineParserState::Instruction {
                instruction: _,
                args,
                comma,
            } => match token.token {
                Tokens::Comma => {
                    if args.is_empty() || *comma {
                        todo!("Unexpected comma")
                    }

                    *comma = true;
                }
                Tokens::Identifier(label) => {
                    if !args.is_empty() && !*comma {
                        todo!("Expected comma")
                    }

                    args.push(InstructionArgument::Label(label));
                    *comma = false;
                }
                Tokens::Whitespace => {}
                token => {
                    todo!("Unexpected {token:?}")
                }
            },
            _ => {
                match token.token {
                    Tokens::Instruction(instruction) => {
                        state = LineParserState::Instruction {
                            instruction,
                            args: Vec::new(),
                            comma: false,
                        }
                    }
                    Tokens::Identifier(_identifier) => {
                        // Label
                    }
                    _ => {}
                }
            }
        }
    }

    println!("{state:?}");

    Ok(out)
}

pub fn parse(input: Vec<Token>) -> Result<Vec<ASLNode>, ParseError> {
    let mut list: Vec<ASLNode> = Vec::new();
    // let mut errors: Vec<ParseError> = Vec::new();

    let lines = lines(input.clone());

    for line in lines {
        let result = parse_line(line);
    }

    Ok(list)
}
