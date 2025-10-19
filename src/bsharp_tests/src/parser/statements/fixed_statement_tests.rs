// Tests for parsing fixed statements

use syntax::statements::FixedStatement;

fn parse_fixed_statement(code: &str) -> Result<FixedStatement, String> {
    Err(format!("Parser not yet implemented: {}", code))
}

#[test]
fn test_parse_fixed_statement() {
    let code = "fixed (int* p = &a) { }";
    // let expected = ...;
    // assert_eq!(parse_fixed_statement(code.into()), Ok(expected));
    assert!(parse_fixed_statement(code.into()).is_err());
}
