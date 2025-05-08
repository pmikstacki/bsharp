// Tests for parsing await expressions

use bsharp::parser::nodes::expressions::AwaitExpression;

fn parse_await_expr(code: &str) -> Result<AwaitExpression, String> {
    Err(format!("Parser not yet implemented: {}", code))
}

#[test]
fn test_parse_await_expr() {
    let code = "await foo()";
    // let expected = ...;
    // assert_eq!(parse_await_expr(code), Ok(expected));
    assert!(parse_await_expr(code).is_err());
}
