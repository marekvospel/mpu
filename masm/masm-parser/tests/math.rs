use anyhow::Result;
use masm_lexer::*;
use masm_parser::asl::ASLNode;
use masm_parser::parse::parse;
use test_utils::get_pkg_fixture;

#[test]
fn should_create_simple_math_expression() -> Result<()> {
    let code = get_pkg_fixture!("math/should_create_simple_math_expr.masm")?;
    let result = parse(tokenize(code)?)?;
    let assertion = get_pkg_fixture!("math/should_create_simple_math_expr.json")?;
    let assertion: Vec<ASLNode> = serde_json::from_str(&assertion)?;

    assert_eq!(result, assertion);

    Ok(())
}

#[test]
#[ignore]
fn should_create_math_expression() -> Result<()> {
    let code = get_pkg_fixture!("math/should_create_math_expr.masm")?;
    let tokenized = tokenize(code)?;

    parse(tokenized);

    Ok(())
}

#[test]
#[ignore]
fn should_create_math_exr_mul_terms() -> Result<()> {
    let code = get_pkg_fixture!("math/should_create_math_expr_mul_terms.masm")?;
    let tokenized = tokenize(code)?;

    parse(tokenized);

    Ok(())
}
