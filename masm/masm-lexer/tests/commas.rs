use anyhow::Result;
use masm_lexer::*;
use test_utils::*;

#[test]
fn should_have_comma() -> Result<()> {
    let code = get_pkg_fixture!("commas/should_have_comma.masm")?;
    let assertion = get_pkg_fixture!("commas/should_have_comma.json")?;

    let assertion: Vec<Token> = serde_json::from_str(&assertion)?;

    assert_eq!(tokenize(code), assertion);

    Ok(())
}
