use masm_lexer::*;

#[test]
fn should_have_string() {
    let mut assertion = Vec::new();

    assertion.push(Token {
        token: Tokens::Whitespace,
        src: "  ".into(),
        start: 0,
        end: 1,
    });

    assertion.push(Token {
        token: Tokens::Literal("db".into()),
        src: "db".into(),
        start: 2,
        end: 3,
    });

    assertion.push(Token {
        token: Tokens::Whitespace,
        src: " ".into(),
        start: 4,
        end: 4,
    });

    assertion.push(Token {
        token: Tokens::SingleQuote,
        src: "'".into(),
        start: 5,
        end: 5,
    });

    assertion.push(Token {
        token: Tokens::SingleQuoteString("Hello world".into()),
        src: "Hello world".into(),
        start: 6,
        end: 16,
    });

    assertion.push(Token {
        token: Tokens::SingleQuote,
        src: "'".into(),
        start: 17,
        end: 17,
    });

    // serde_json::to_string(tokenize("  db 'Hello world'"))
    // println!("{}", serde_json::to_string(&tokenize("  db 'Hello world'")).unwrap());
    // assert!(false);

    assert_eq!(tokenize("  db 'Hello world'"), assertion);
}

#[test]
fn should_have_double_string() {
    let mut assertion = Vec::new();

    assertion.push(Token {
        token: Tokens::Whitespace,
        src: "  ".into(),
        start: 0,
        end: 1,
    });

    assertion.push(Token {
        token: Tokens::Literal("db".into()),
        src: "db".into(),
        start: 2,
        end: 3,
    });

    assertion.push(Token {
        token: Tokens::Whitespace,
        src: " ".into(),
        start: 4,
        end: 4,
    });

    assertion.push(Token {
        token: Tokens::DoubleQuote,
        src: "\"".into(),
        start: 5,
        end: 5,
    });

    assertion.push(Token {
        token: Tokens::DoubleQuoteString("Hello world".into()),
        src: "Hello world".into(),
        start: 6,
        end: 16,
    });

    assertion.push(Token {
        token: Tokens::DoubleQuote,
        src: "\"".into(),
        start: 17,
        end: 17,
    });

    assert_eq!(tokenize("  db \"Hello world\""), assertion);
}

#[test]
fn should_allow_special_chars_in_string() {
    let mut assertion = Vec::new();

    assertion.push(Token {
        token: Tokens::DoubleQuote,
        src: "\"".into(),
        start: 0,
        end: 0,
    });

    assertion.push(Token {
        token: Tokens::DoubleQuoteString(" $'\"".into()),
        src: " $'\"".into(),
        start: 1,
        end: 5,
    });

    assertion.push(Token {
        token: Tokens::DoubleQuote,
        src: "\"".into(),
        start: 6,
        end: 6,
    });

    assert_eq!(tokenize("\" $'\\\"\""), assertion)
}
