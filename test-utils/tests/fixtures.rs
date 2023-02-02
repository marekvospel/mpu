use std::fs::read_to_string;
use anyhow::Result;
use test_utils::*;

#[test]
fn should_read_fixture() -> Result<()> {
    let fixture = get_pkg_fixture!("fixture.txt")?;

    assert_eq!(fixture, "true");

    Ok(())
}

#[test]
/// Because this could behave differently on different machines (different drive locations,
/// different fs format) a file needs to be read to test whether this works
fn should_get_fixture_path() -> Result<()> {
    let fixture = get_pkg_file!("tests/fixtures/fixture.txt");

    assert_eq!(read_to_string(fixture)?, "true");

    Ok(())
}
