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

pub fn tokenize<S: Into<String>>(code: S) -> Vec<Token> {
    let code = code.into();
    let mut tokens = Vec::new();

    let mut quote_active = false;
    let mut double_quote_active = false;
    let mut whitespace_active = false;
    let mut comment_active = false;

    let mut collected = String::new();

    for (i, char) in code.chars().enumerate() {
        if char == '\n' {
            if comment_active {
                comment_active = false;

                save_comment(&mut collected, &mut tokens, i)
            }
        }

        match char {
            ';' => {
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
            ' ' | '\r' => {
                if !whitespace_active && !comment_active {
                    save_collected(&mut collected, &mut tokens, i);
                    whitespace_active = true;
                }
            }
            '\n' => {
                if comment_active {
                    save_comment(&mut collected, &mut tokens, i);
                } else if whitespace_active {
                    save_whitespace(&mut collected, &mut tokens, i);
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
            _ => {}
        };

        collected.push(char);
    }

    if comment_active {
        save_comment(&mut collected, &mut tokens, code.len())
    } else if whitespace_active {
        save_whitespace(&mut collected, &mut tokens, code.len())
    }

    tokens
}
