// Tests for parsing default expressions

use bsharp::parser::nodes::expressions::DefaultExpression;

fn parse_default_expr(code: &str) -> Result<DefaultExpression, String> {
    Err(format!("Parser not yet implemented: {}", code))
}

#[test]
fn test_parse_default_expr() {
    let code = "default(int)";
    // let expected = ...;
    // assert_eq!(parse_default_expr(code), Ok(expected));
    assert!(parse_default_expr(code).is_err());
}
