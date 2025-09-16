// Tests for parsing yield statements

use bsharp::syntax::nodes::statements::YieldStatement;

fn parse_yield_statement(code: &str) -> Result<YieldStatement, String> {
    Err(format!("Parser not yet implemented: {}", code))
}

#[test]
fn test_parse_yield_statement() {
    let code = "yield return 1;";
    // let expected = ...;
    // assert_eq!(parse_yield_statement(code), Ok(expected));
    assert!(parse_yield_statement(code).is_err());
}
