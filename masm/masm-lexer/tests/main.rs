use anyhow::Result;
use masm_lexer::*;

#[test]
pub fn should_return_empty_vec() -> Result<()> {
    assert_eq!(tokenize("")?, vec![]);

    Ok(())
}
