use anyhow::Result;
use masm_lexer::LexError::UnterminatedString;
use masm_lexer::*;
use test_utils::*;

#[test]
fn should_have_string() -> Result<()> {
    let code = get_pkg_fixture!("strings/should_have_string.masm")?;
    let assertion = get_pkg_fixture!("strings/should_have_string.json")?;

    let assertion: Vec<Token> = serde_json::from_str(&assertion)?;

    assert_eq!(tokenize(code)?, assertion);

    Ok(())
}

#[test]
fn should_have_double_string() -> Result<()> {
    let code = get_pkg_fixture!("strings/should_have_double_string.masm")?;
    let assertion = get_pkg_fixture!("strings/should_have_double_string.json")?;

    let assertion: Vec<Token> = serde_json::from_str(&assertion)?;

    assert_eq!(tokenize(code)?, assertion);

    Ok(())
}

#[test]
fn should_not_be_escaped() -> Result<()> {
    let code = get_pkg_fixture!("strings/should_not_be_escaped.masm")?;
    let assertion = get_pkg_fixture!("strings/should_not_be_escaped.json")?;

    let assertion: Vec<Token> = serde_json::from_str(&assertion)?;

    assert_eq!(tokenize(code)?, assertion);

    Ok(())
}

#[test]
fn should_allow_special_chars_in_string() -> Result<()> {
    let code = get_pkg_fixture!("strings/should_allow_special_chars_in_string.masm")?;
    let assertion = get_pkg_fixture!("strings/should_allow_special_chars_in_string.json")?;

    let assertion: Vec<Token> = serde_json::from_str(&assertion)?;
    assert_eq!(tokenize(code)?, assertion);

    Ok(())
}

#[test]
fn should_return_unterminated_string_error() -> Result<()> {
    let code = get_pkg_fixture!("strings/should_return_unterminated_string_error.masm")?;
    let result = tokenize(code).unwrap_err();

    assert_eq!(
        result,
        vec![UnterminatedString {
            at: Position {
                line: 1,
                column: 32,
                offset: 31
            }
        }]
        .into(),
    );

    Ok(())
}

#[test]
fn should_return_unterminated_string_error_eof() -> Result<()> {
    let code = get_pkg_fixture!("strings/should_return_unterminated_string_error_eof.masm")?;
    let result = tokenize(code).unwrap_err();

    assert_eq!(
        result,
        vec![UnterminatedString {
            at: Position {
                line: 1,
                column: 32,
                offset: 31
            }
        }]
        .into(),
    );

    Ok(())
}

#[test]
fn should_return_unterminated_double_string_error() -> Result<()> {
    let code = get_pkg_fixture!("strings/should_return_unterminated_double_string_error.masm")?;
    let result = tokenize(code).unwrap_err();

    assert_eq!(
        *result.inner.get(0).unwrap(),
        UnterminatedString {
            at: Position {
                line: 1,
                column: 32,
                offset: 31
            }
        },
    );

    assert_eq!(
        LexErrors::from(vec![result.inner.get(1).unwrap().to_owned()]).to_string(),
        "unterminated string literal at 2:32\n".to_string(),
    );

    Ok(())
}
