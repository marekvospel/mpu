use anyhow::Result;
use masm_lexer::*;
use masm_parser::*;
use test_utils::get_pkg_fixture;

#[test]
fn should_create_simple_math_expression() -> Result<()> {
    let code = get_pkg_fixture!("math/should_create_simple_math_expr.masm")?;
    let tokenized = tokenize(code)?;

    parse(tokenized);

    Ok(())
}

#[test]
fn should_create_math_expression() -> Result<()> {
    let code = get_pkg_fixture!("math/should_create_math_expr.masm")?;
    let tokenized = tokenize(code)?;

    parse(tokenized);

    Ok(())
}

#[test]
fn should_create_math_exr_mul_terms() -> Result<()> {
    let code = get_pkg_fixture!("math/should_create_math_expr_mul_terms.masm")?;
    let tokenized = tokenize(code)?;

    parse(tokenized);

    assert!(false);

    Ok(())
}
