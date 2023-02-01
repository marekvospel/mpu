use masm_lexer::*;

#[test]
fn should_have_comment() {
    let mut assertion = Vec::new();
    assertion.push(Token {
        token: Tokens::Semicolon,
        start: 0,
        end: 0,
        src: ";".into(),
    });

    assertion.push(Token {
        token: Tokens::Comment(" Hello world".into()),
        start: 1,
        end: 12,
        src: " Hello world".into(),
    });

    assert_eq!(tokenize("; Hello world"), assertion);

    assertion.push(Token {
        token: Tokens::Newline,
        start: 13,
        end: 13,
        src: "\n".into(),
    });

    assert_eq!(tokenize("; Hello world\n"), assertion);
}

#[test]
fn should_have_two_comments() {
    let mut assertion = Vec::new();

    assertion.push(Token {
        token: Tokens::Whitespace,
        src: " ".into(),
        start: 0,
        end: 0,
    });

    assertion.push(Token {
        token: Tokens::Semicolon,
        src: ";".into(),
        start: 1,
        end: 1,
    });

    assertion.push(Token {
        token: Tokens::Comment(" Hello world".into()),
        src: " Hello world".into(),
        start: 2,
        end: 13,
    });

    assertion.push(Token {
        token: Tokens::Newline,
        src: "\n".into(),
        start: 14,
        end: 14,
    });

    assertion.push(Token {
        token: Tokens::Whitespace,
        src: "  ".into(),
        start: 15,
        end: 16,
    });

    assertion.push(Token {
        token: Tokens::Semicolon,
        src: ";".into(),
        start: 17,
        end: 17,
    });

    assertion.push(Token {
        token: Tokens::Comment("Hi there  ".into()),
        src: "Hi there  ".into(),
        start: 18,
        end: 27,
    });

    assert_eq!(tokenize(" ; Hello world\n  ;Hi there  "), assertion);
}
