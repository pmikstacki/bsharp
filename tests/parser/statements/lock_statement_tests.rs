// Tests for parsing lock statements

use bsharp::parser::nodes::statements::LockStatement;

fn parse_lock_statement(code: &str) -> Result<LockStatement, String> {
    Err(format!("Parser not yet implemented: {}", code))
}

#[test]
fn test_parse_lock_statement() {
    let code = "lock (obj) { }";
    // let expected = ...;
    // assert_eq!(parse_lock_statement(code), Ok(expected));
    assert!(parse_lock_statement(code).is_err());
}
