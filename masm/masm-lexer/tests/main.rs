use masm_lexer::*;

#[test]
pub fn should_return_empty_vec() {
    assert_eq!(tokenize(""), Vec::new())
}
