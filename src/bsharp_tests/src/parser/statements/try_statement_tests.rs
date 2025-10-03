// Tests for parsing try statements

use syntax::nodes::statements::TryStatement;

fn parse_try_statement(code: &str) -> Result<TryStatement, String> {
    Err(format!("Parser not yet implemented: {}", code))
}

#[test]
fn test_parse_try_statement() {
    let code = "try { } catch { }";
    // let expected = ...;
    // assert_eq!(parse_try_statement(code), Ok(expected));
    assert!(parse_try_statement(code).is_err());
}
