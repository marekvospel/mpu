use anyhow::Result;
use masm_lexer::*;
use test_utils::*;

#[test]
fn should_have_comment() -> Result<()> {
    let code = get_pkg_fixture!("comments/should_have_comment.masm")?;
    let assertion = get_pkg_fixture!("comments/should_have_comment.json")?;

    let assertion: Vec<Token> = serde_json::from_str(&assertion)?;

    assert_eq!(tokenize(code)?, assertion);

    Ok(())
}

#[test]
fn should_have_comment_newline() -> Result<()> {
    let code = get_pkg_fixture!("comments/should_have_comment_newline.masm")?;
    let assertion = get_pkg_fixture!("comments/should_have_comment_newline.json")?;

    let assertion: Vec<Token> = serde_json::from_str(&assertion)?;

    assert_eq!(tokenize(code)?, assertion);

    Ok(())
}

#[test]
fn should_have_two_comments() -> Result<()> {
    let code = get_pkg_fixture!("comments/should_have_two_comments.masm")?;
    let assertion = get_pkg_fixture!("comments/should_have_two_comments.json")?;

    let assertion: Vec<Token> = serde_json::from_str(&assertion)?;

    assert_eq!(tokenize(code)?, assertion);

    Ok(())
}
