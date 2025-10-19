// Tests for parsing checked statements

use syntax::statements::CheckedStatement;

fn parse_checked_statement(code: &str) -> Result<CheckedStatement, String> {
    Err(format!("Parser not yet implemented: {}", code))
}

#[test]
fn test_parse_checked_statement() {
    let code = "checked { x++; }";
    // let expected = ...;
    // assert_eq!(parse_checked_statement(code.into()), Ok(expected));
    assert!(parse_checked_statement(code.into()).is_err());
}
