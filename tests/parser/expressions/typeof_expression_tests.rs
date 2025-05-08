// Tests for parsing typeof expressions

use bsharp::parser::nodes::expressions::TypeofExpression;

fn parse_typeof_expr(code: &str) -> Result<TypeofExpression, String> {
    Err(format!("Parser not yet implemented: {}", code))
}

#[test]
fn test_parse_typeof_expr() {
    let code = "typeof(int)";
    // let expected = ...;
    // assert_eq!(parse_typeof_expr(code), Ok(expected));
    assert!(parse_typeof_expr(code).is_err());
}
