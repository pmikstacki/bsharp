// Tests for parsing throw expressions

use bsharp::parser::nodes::expressions::ThrowExpression;

fn parse_throw_expr(code: &str) -> Result<ThrowExpression, String> {
    Err(format!("Parser not yet implemented: {}", code))
}

#[test]
fn test_parse_throw_expr() {
    let code = "throw new Exception()";
    // let expected = ...;
    // assert_eq!(parse_throw_expr(code), Ok(expected));
    assert!(parse_throw_expr(code).is_err());
}
