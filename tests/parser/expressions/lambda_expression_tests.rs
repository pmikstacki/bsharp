// Tests for parsing lambda expressions

use bsharp::parser::nodes::expressions::LambdaExpression;

fn parse_lambda_expr(code: &str) -> Result<LambdaExpression, String> {
    Err(format!("Parser not yet implemented: {}", code))
}

#[test]
fn test_parse_lambda_expr() {
    let code = "x => x + 1";
    // let expected = ...;
    // assert_eq!(parse_lambda_expr(code), Ok(expected));
    assert!(parse_lambda_expr(code).is_err());
}
