use crate::LexError::UnterminatedString;
pub(crate) use crate::*;

fn save_collected(
    code: &str,
    collected: &mut String,
    tokens: &mut Vec<Token>,
    start: Position,
    end: Position,
    state: &mut LexerState,
) {
    if !collected.is_empty() {
        let loc = SourceLocation { start, end };
        let src = get_location_from_source(code, loc);

        let content = collected.to_string();
        let token = match state {
            LexerState::Collecting => Tokens::detect(content),
            LexerState::Quote | LexerState::DoubleQuote => Tokens::StringLiteral(content),
            LexerState::Comment => Tokens::Comment(content),
            LexerState::Whitespace => Tokens::Whitespace,
        };

        tokens.push(Token { token, src, loc });

        *state = LexerState::Collecting;
        *collected = String::new();
    }
}

fn inc_position(mut position: Position, char: char) -> Position {
    position.offset += 1;
    if char == '\n' {
        position.line += 1;
        position.column = 0;
    } else {
        position.column += 1;
    }
    position
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum LexerState {
    Collecting,
    Quote,
    DoubleQuote,
    Whitespace,
    Comment,
}

impl LexerState {
    fn is_string(&self) -> bool {
        self == &LexerState::DoubleQuote || self == &LexerState::Quote
    }

    fn is_broken_by_whitespace(&self) -> bool {
        matches!(self, LexerState::Collecting)
    }
}

pub fn tokenize<S: Into<String>>(code: S) -> Result<Vec<Token>, LexErrors> {
    let code = code.into();

    // This might not be the best way to do things, because the original source file offset will be
    // wrong, but I don't really care about Windows, as Windows normies have no idea what a cli,
    // assembler or assembly language is.
    // I might return to this sometime.
    #[cfg(windows)]
    let code = code.replace("\r\n", "\n");

    let mut tokens = Vec::new();
    let mut errors = Vec::new();

    let mut position = Position::default();
    let mut collect_position = Position {
        line: 1,
        column: 1,
        offset: 0,
    };
    let mut collected = String::new();
    let mut reset_collect = false;

    let mut escape_active = false;

    let mut state = LexerState::Collecting;

    for (i, char) in code.chars().enumerate() {
        let last_position = position;
        position = inc_position(position, char);
        if i == 0 {
            position.offset = 0;
        }
        if reset_collect {
            collect_position = position;
            reset_collect = false;
        }

        if char == '\n' {
            escape_active = false;
        }

        if escape_active && (state.is_string()) {
            collected.push(char);
            escape_active = false;
            continue;
        }

        match char {
            ';' => {
                if !state.is_string() {
                    save_collected(
                        &code,
                        &mut collected,
                        &mut tokens,
                        collect_position,
                        last_position,
                        &mut state,
                    );
                    state = LexerState::Comment;

                    tokens.push(Token {
                        src: ";".into(),
                        token: Tokens::Semicolon,
                        loc: SourceLocation {
                            start: position,
                            end: position,
                        },
                    });
                    reset_collect = true;
                    continue;
                }
            }
            ',' => {
                if !state.is_string() && state != LexerState::Comment {
                    save_collected(
                        &code,
                        &mut collected,
                        &mut tokens,
                        collect_position,
                        last_position,
                        &mut state,
                    );
                    tokens.push(Token {
                        token: Tokens::Comma,
                        src: ",".into(),
                        loc: SourceLocation {
                            start: position,
                            end: position,
                        },
                    });
                    reset_collect = true;
                    continue;
                }
            }
            '\\' => {
                if state.is_string() {
                    escape_active = true;
                    continue;
                }
            }
            '\'' | '"' => {
                if (state != LexerState::DoubleQuote && char == '\'')
                    || (state != LexerState::Quote && char == '"')
                {
                    if (state == LexerState::Quote && char == '\'')
                        || (state == LexerState::DoubleQuote && char == '"')
                    {
                        save_collected(
                            &code,
                            &mut collected,
                            &mut tokens,
                            collect_position,
                            position,
                            &mut state,
                        );
                        reset_collect = true;
                        continue;
                    } else {
                        save_collected(
                            &code,
                            &mut collected,
                            &mut tokens,
                            collect_position,
                            last_position,
                            &mut state,
                        );
                        state = if char == '\'' {
                            LexerState::Quote
                        } else {
                            LexerState::DoubleQuote
                        };
                        collect_position = position;
                        continue;
                    }
                }
            }
            '\n' => {
                let linebreak_position = inc_position(last_position, 'a');
                if state.is_string() {
                    errors.push(UnterminatedString {
                        at: linebreak_position,
                    });
                }

                save_collected(
                    &code,
                    &mut collected,
                    &mut tokens,
                    collect_position,
                    last_position,
                    &mut state,
                );
                tokens.push(Token {
                    token: Tokens::Newline,
                    src: "\n".into(),
                    loc: SourceLocation {
                        start: linebreak_position,
                        end: linebreak_position,
                    },
                });
                reset_collect = true;
                continue;
            }
            c if c.is_whitespace() => {
                if state.is_broken_by_whitespace() {
                    save_collected(
                        &code,
                        &mut collected,
                        &mut tokens,
                        collect_position,
                        last_position,
                        &mut state,
                    );
                    collect_position = position;
                    state = LexerState::Whitespace;
                }
            }
            _ => {
                if state == LexerState::Whitespace {
                    save_collected(
                        &code,
                        &mut collected,
                        &mut tokens,
                        collect_position,
                        last_position,
                        &mut state,
                    );
                    collect_position = position;
                }
            }
        };

        collected.push(char);
    }

    if state.is_string() {
        let error_position = inc_position(position, 'a');
        errors.push(UnterminatedString { at: error_position });
        return Err(errors.into());
    } else {
        save_collected(
            &code,
            &mut collected,
            &mut tokens,
            collect_position,
            position,
            &mut state,
        );
    }

    if errors.is_empty() {
        Ok(tokens)
    } else {
        Err(errors.into())
    }
}
