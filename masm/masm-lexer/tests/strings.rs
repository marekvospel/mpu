use masm_lexer::*;
use test_utils::*;
use anyhow::Result;

#[test]
fn should_have_string() -> Result<()> {
    let code = get_pkg_fixture!("strings/should_have_string.masm")?;
    let assertion = get_pkg_fixture!("strings/should_have_string.json")?;

    let assertion: Vec<Token> = serde_json::from_str(&assertion)?;

    assert_eq!(tokenize(code), assertion);

    Ok(())
}

#[test]
fn should_have_double_string() -> Result<()> {
    let code = get_pkg_fixture!("strings/should_have_double_string.masm")?;
    let assertion = get_pkg_fixture!("strings/should_have_double_string.json")?;

    let assertion: Vec<Token> = serde_json::from_str(&assertion)?;

    assert_eq!(tokenize(code), assertion);

    Ok(())
}

#[test]
fn should_allow_special_chars_in_string() -> Result<()> {
    let code = get_pkg_fixture!("strings/should_allow_special_chars_in_string.masm")?;
    let assertion = get_pkg_fixture!("strings/should_allow_special_chars_in_string.json")?;

    let assertion: Vec<Token> = serde_json::from_str(&assertion)?;
    assert_eq!(tokenize(code), assertion);

    Ok(())
}
