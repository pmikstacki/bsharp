// Tests for parsing invocation expressions

use bsharp::syntax::nodes::expressions::InvocationExpression;

fn parse_invocation_expr(code: &str) -> Result<InvocationExpression, String> {
    Err(format!("Parser not yet implemented: {}", code))
}

#[test]
fn test_parse_invocation_expr() {
    let code = "foo(bar)";
    // let expected = ...;
    // assert_eq!(parse_invocation_expr(code), Ok(expected));
    assert!(parse_invocation_expr(code).is_err());
}
