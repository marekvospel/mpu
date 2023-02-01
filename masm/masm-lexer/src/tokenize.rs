pub(crate) use crate::*;

fn save_collected(collected: &mut String, tokens: &mut Vec<Token>, index: usize) {
    if collected.len() > 0 {
        tokens.push(Token::detect(collected.to_string(), index));
        *collected = String::new();
    }
}

fn save_comment(collected: &mut String, tokens: &mut Vec<Token>, index: usize) {
    if collected.len() > 0 {
        tokens.push(Token {
            token: Tokens::Comment(collected.clone()),
            start: index - collected.len(),
            end: index - 1,
            src: collected.clone(),
        });
        *collected = String::new();
    }
}

fn save_whitespace(collected: &mut String, tokens: &mut Vec<Token>, index: usize) {
    if collected.len() > 0 {
        tokens.push(Token {
            token: Tokens::Whitespace,
            start: index - collected.len(),
            end: index - 1,
            src: collected.clone(),
        });
        *collected = String::new();
    }
}

fn save_string(collected: &mut String, tokens: &mut Vec<Token>, index: usize, double: bool) {
    if collected.len() > 0 {
        tokens.push(Token {
            token: if double {
                Tokens::DoubleQuoteString(collected.clone())
            } else {
                Tokens::SingleQuoteString(collected.clone())
            },
            start: index - collected.len(),
            end: index - 1,
            src: collected.clone(),
        });
        *collected = String::new();
    }
}

pub fn tokenize<S: Into<String>>(code: S) -> Vec<Token> {
    let code = code.into();
    let mut tokens = Vec::new();

    let mut escape_active = false;
    let mut quote_active = false;
    // let mut double_quote_active = false;
    let mut whitespace_active = false;
    let mut comment_active = false;

    let mut collected = String::new();

    for (i, char) in code.chars().enumerate() {
        if char == '\n' {
            escape_active = false;
            if comment_active {
                comment_active = false;

                save_comment(&mut collected, &mut tokens, i)
            }
        }

        if escape_active {
            collected.push(char);
            continue;
        }

        match char {
            ';' => {
                if !quote_active {
                    comment_active = true;
                    if whitespace_active {
                        whitespace_active = false;
                        save_whitespace(&mut collected, &mut tokens, i);
                    } else {
                        save_collected(&mut collected, &mut tokens, i);
                    }

                    tokens.push(Token {
                        start: i,
                        end: i,
                        src: ";".into(),
                        token: Tokens::Semicolon,
                    });
                    continue;
                }
            }
            '\\' => {
                if quote_active {
                    escape_active = true;
                }
            }
            '\'' => {
                if !quote_active {
                    if comment_active {
                        save_comment(&mut collected, &mut tokens, i);
                        whitespace_active = false
                    } else if whitespace_active {
                        save_whitespace(&mut collected, &mut tokens, i);
                        whitespace_active = false
                    } else {
                        save_collected(&mut collected, &mut tokens, i);
                    }
                    tokens.push(Token {
                        token: Tokens::SingleQuote,
                        start: i,
                        end: i,
                        src: "'".into(),
                    });
                    quote_active = true;
                    continue;
                } else {
                    save_string(&mut collected, &mut tokens, i, false);
                    tokens.push(Token {
                        token: Tokens::SingleQuote,
                        start: i,
                        end: i,
                        src: "'".into(),
                    });
                    quote_active = false;
                    continue;
                }
            }
            ' ' | '\r' => {
                if !whitespace_active && !comment_active && !quote_active {
                    save_collected(&mut collected, &mut tokens, i);
                    whitespace_active = true;
                }
            }
            '\n' => {
                if comment_active {
                    save_comment(&mut collected, &mut tokens, i);
                    comment_active = false
                } else if whitespace_active {
                    save_whitespace(&mut collected, &mut tokens, i);
                    whitespace_active = false
                } else {
                    save_collected(&mut collected, &mut tokens, i);
                }
                tokens.push(Token {
                    start: i,
                    end: i,
                    src: "\n".into(),
                    token: Tokens::Newline,
                });
                continue;
            }
            _ => {
                if whitespace_active {
                    save_whitespace(&mut collected, &mut tokens, i);
                    whitespace_active = false;
                }
            }
        };

        collected.push(char);
    }

    if comment_active {
        save_comment(&mut collected, &mut tokens, code.len())
    } else if whitespace_active {
        save_whitespace(&mut collected, &mut tokens, code.len())
    } else if quote_active {
        save_string(&mut collected, &mut tokens, code.len(), false)
    }

    tokens
}
