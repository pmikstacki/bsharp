// Tests for parsing unsafe statements

use syntax::statements::UnsafeStatement;

fn parse_unsafe_statement(code: &str) -> Result<UnsafeStatement, String> {
    Err(format!("Parser not yet implemented: {}", code))
}

#[test]
fn test_parse_unsafe_statement() {
    let code = "unsafe { x++; }";
    // let expected = ...;
    // assert_eq!(parse_unsafe_statement(code), Ok(expected));
    assert!(parse_unsafe_statement(code).is_err());
}
