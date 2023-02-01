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

    assert_eq!(tokenize("  db 'Hello world'"), assertion);
}
