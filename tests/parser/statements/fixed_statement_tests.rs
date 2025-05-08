// Tests for parsing fixed statements

use bsharp::parser::nodes::statements::FixedStatement;

fn parse_fixed_statement(code: &str) -> Result<FixedStatement, String> {
    Err(format!("Parser not yet implemented: {}", code))
}

#[test]
fn test_parse_fixed_statement() {
    let code = "fixed (int* p = &a) { }";
    // let expected = ...;
    // assert_eq!(parse_fixed_statement(code), Ok(expected));
    assert!(parse_fixed_statement(code).is_err());
}
