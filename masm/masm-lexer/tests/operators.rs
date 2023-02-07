use anyhow::Result;
use masm_lexer::{tokenize, Token};
use test_utils::get_pkg_fixture;

#[test]
fn should_have_all_operators() -> Result<()> {
    let code = get_pkg_fixture!("operators/should_have_all_operators.masm")?;
    println!("{}", serde_json::to_string(&tokenize(&code)?)?);
    let assertion = get_pkg_fixture!("operators/should_have_all_operators.json")?;

    let assertion: Vec<Token> = serde_json::from_str(&assertion)?;

    assert_eq!(tokenize(code)?, assertion);

    Ok(())
}
