use crate::LexError::UnterminatedString;
pub(crate) use crate::*;

fn save_collected(
    code: &str,
    collected: &mut String,
    tokens: &mut Vec<Token>,
    start: Position,
    end: Position,
) {
    if !collected.is_empty() {
        let loc = SourceLocation { start, end };
        tokens.push(Token::detect(
            get_location_from_source(code, loc),
            collected.clone(),
            loc,
        ));
        *collected = String::new();
    }
}

fn save_comment(
    code: &str,
    collected: &mut String,
    tokens: &mut Vec<Token>,
    start: Position,
    end: Position,
) {
    if !collected.is_empty() {
        let loc = SourceLocation { start, end };
        tokens.push(Token {
            token: Tokens::Comment(collected.clone()),
            src: get_location_from_source(code, loc),
            loc,
        });
        *collected = String::new();
    }
}

fn save_whitespace(
    code: &str,
    collected: &mut String,
    tokens: &mut Vec<Token>,
    start: Position,
    end: Position,
) {
    if !collected.is_empty() {
        let loc = SourceLocation { start, end };
        tokens.push(Token {
            token: Tokens::Whitespace,
            src: get_location_from_source(code, loc),
            loc,
        });
        *collected = String::new();
    }
}

fn save_string(
    src: &str,
    collected: &mut String,
    tokens: &mut Vec<Token>,
    double: bool,
    start: Position,
    end: Position,
) {
    if !collected.is_empty() {
        let loc = SourceLocation { start, end };
        tokens.push(Token {
            token: if double {
                Tokens::DoubleQuoteString(collected.clone())
            } else {
                Tokens::SingleQuoteString(collected.clone())
            },
            src: get_location_from_source(src, loc),
            loc,
        });
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

pub fn tokenize<S: Into<String>>(code: S) -> Result<Vec<Token>, LexError> {
    let code = code.into();
    let mut tokens = Vec::new();

    let mut position = Position::new();
    let mut collect_position = Position::new();
    let mut reset_collect = false;

    let mut escape_active = false;
    let mut quote_active = false;
    let mut double_quote_active = false;
    let mut whitespace_active = false;
    let mut comment_active = false;

    let mut collected = String::new();

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

        if escape_active && (quote_active || double_quote_active) {
            collected.push(char);
            escape_active = false;
            continue;
        }

        match char {
            ';' => {
                if !quote_active && !double_quote_active {
                    comment_active = true;
                    if whitespace_active {
                        whitespace_active = false;
                        save_whitespace(
                            &code,
                            &mut collected,
                            &mut tokens,
                            collect_position,
                            last_position,
                        );
                    } else {
                        save_collected(
                            &code,
                            &mut collected,
                            &mut tokens,
                            collect_position,
                            last_position,
                        );
                    }

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
                if !quote_active && !double_quote_active && !comment_active {
                    if whitespace_active {
                        whitespace_active = false;
                        save_whitespace(
                            &code,
                            &mut collected,
                            &mut tokens,
                            collect_position,
                            last_position,
                        );
                    } else {
                        save_collected(
                            &code,
                            &mut collected,
                            &mut tokens,
                            collect_position,
                            last_position,
                        );
                    }
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
                if quote_active || double_quote_active {
                    escape_active = true;
                    continue;
                }
            }
            '\'' => {
                if !double_quote_active {
                    if quote_active {
                        save_string(
                            &code,
                            &mut collected,
                            &mut tokens,
                            false,
                            collect_position,
                            position,
                        );
                        quote_active = false;
                        reset_collect = true;
                        continue;
                    } else {
                        if comment_active {
                            save_comment(
                                &code,
                                &mut collected,
                                &mut tokens,
                                collect_position,
                                last_position,
                            );
                            comment_active = false
                        } else if whitespace_active {
                            save_whitespace(
                                &code,
                                &mut collected,
                                &mut tokens,
                                collect_position,
                                last_position,
                            );
                            whitespace_active = false
                        } else {
                            save_collected(
                                &code,
                                &mut collected,
                                &mut tokens,
                                collect_position,
                                last_position,
                            );
                        }
                        quote_active = true;
                        collect_position = position;
                        continue;
                    }
                }
            }
            '\"' => {
                if !quote_active {
                    if double_quote_active {
                        save_string(
                            &code,
                            &mut collected,
                            &mut tokens,
                            true,
                            collect_position,
                            position,
                        );
                        double_quote_active = false;
                        reset_collect = true;
                        continue;
                    } else {
                        if comment_active {
                            save_comment(
                                &code,
                                &mut collected,
                                &mut tokens,
                                collect_position,
                                last_position,
                            );
                            comment_active = false
                        } else if whitespace_active {
                            save_whitespace(
                                &code,
                                &mut collected,
                                &mut tokens,
                                collect_position,
                                last_position,
                            );
                            whitespace_active = false
                        } else {
                            save_collected(
                                &code,
                                &mut collected,
                                &mut tokens,
                                collect_position,
                                last_position,
                            );
                        }
                        double_quote_active = true;
                        collect_position = position;
                        continue;
                    }
                }
            }
            '\n' => {
                if comment_active {
                    save_comment(
                        &code,
                        &mut collected,
                        &mut tokens,
                        collect_position,
                        last_position,
                    );
                    comment_active = false
                } else if whitespace_active {
                    save_whitespace(
                        &code,
                        &mut collected,
                        &mut tokens,
                        collect_position,
                        last_position,
                    );
                    whitespace_active = false
                } else if quote_active || double_quote_active {
                    let error_position = inc_position(last_position, 'a');
                    return Err(UnterminatedString { at: error_position });
                } else {
                    save_collected(
                        &code,
                        &mut collected,
                        &mut tokens,
                        collect_position,
                        last_position,
                    );
                }
                let linebreak_position = inc_position(last_position, 'a');
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
                if !whitespace_active && !comment_active && !quote_active && !double_quote_active {
                    save_collected(
                        &code,
                        &mut collected,
                        &mut tokens,
                        collect_position,
                        last_position,
                    );
                    collect_position = position;
                    whitespace_active = true;
                }
            }
            _ => {
                if whitespace_active {
                    save_whitespace(
                        &code,
                        &mut collected,
                        &mut tokens,
                        collect_position,
                        last_position,
                    );
                    collect_position = position;
                    whitespace_active = false;
                }
            }
        };

        collected.push(char);
    }

    if comment_active {
        save_comment(
            &code,
            &mut collected,
            &mut tokens,
            collect_position,
            position,
        );
    } else if whitespace_active {
        save_whitespace(
            &code,
            &mut collected,
            &mut tokens,
            collect_position,
            position,
        );
    } else if quote_active || double_quote_active {
        let error_position = inc_position(position, 'a');
        return Err(UnterminatedString { at: error_position });
    } else {
        save_collected(
            &code,
            &mut collected,
            &mut tokens,
            collect_position,
            position,
        );
    }

    Ok(tokens)
}
