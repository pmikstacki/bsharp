// Tests for parsing nameof expressions

use bsharp::parser::nodes::expressions::NameofExpression;

fn parse_nameof_expr(code: &str) -> Result<NameofExpression, String> {
    Err(format!("Parser not yet implemented: {}", code))
}

#[test]
fn test_parse_nameof_expr() {
    let code = "nameof(Foo)";
    // let expected = ...;
    // assert_eq!(parse_nameof_expr(code), Ok(expected));
    assert!(parse_nameof_expr(code).is_err());
}
