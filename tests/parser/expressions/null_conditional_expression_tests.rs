// Tests for parsing null-conditional expressions

use bsharp::parser::nodes::expressions::NullConditionalExpression;

fn parse_null_conditional_expr(code: &str) -> Result<NullConditionalExpression, String> {
    Err(format!("Parser not yet implemented: {}", code))
}

#[test]
fn test_parse_null_conditional_expr() {
    let code = "foo?.bar";
    // let expected = ...;
    // assert_eq!(parse_null_conditional_expr(code), Ok(expected));
    assert!(parse_null_conditional_expr(code).is_err());
}
