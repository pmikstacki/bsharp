// Tests for parsing checked expressions

use syntax::nodes::expressions::CheckedExpression;

fn parse_checked_expr(code: &str) -> Result<CheckedExpression, String> {
    Err(format!("Parser not yet implemented: {}", code))
}

#[test]
fn test_parse_checked_expr() {
    let code = "checked(x + y)";
    // let expected = ...;
    // assert_eq!(parse_checked_expr(code), Ok(expected));
    assert!(parse_checked_expr(code).is_err());
}
