// Tests for parsing conditional expressions

use bsharp::parser::nodes::expressions::ConditionalExpression;

fn parse_conditional_expr(code: &str) -> Result<ConditionalExpression, String> {
    Err(format!("Parser not yet implemented: {}", code))
}

#[test]
fn test_parse_conditional_expr() {
    let code = "x ? y : z";
    // let expected = ...;
    // assert_eq!(parse_conditional_expr(code), Ok(expected));
    assert!(parse_conditional_expr(code).is_err());
}
