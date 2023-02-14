use anyhow::Result;
use masm_lexer::tokenize;
use masm_parser::parse::parse;

#[test]
fn should_parse_mov() -> Result<()> {
    let input = "mov eax, ebx";

    let result = parse(tokenize(input)?)?;

    println!("{result:?}");

    Ok(())
}
