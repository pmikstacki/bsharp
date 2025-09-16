// Tests for parsing null-forgiving expressions

use bsharp::syntax::nodes::expressions::NullForgivingExpression;

fn parse_null_forgiving_expr(code: &str) -> Result<NullForgivingExpression, String> {
    Err(format!("Parser not yet implemented: {}", code))
}

#[test]
fn test_parse_null_forgiving_expr() {
    let code = "foo!";
    // let expected = ...;
    // assert_eq!(parse_null_forgiving_expr(code), Ok(expected));
    assert!(parse_null_forgiving_expr(code).is_err());
}
