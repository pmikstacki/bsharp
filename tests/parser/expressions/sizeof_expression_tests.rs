// Tests for parsing sizeof expressions

use bsharp::parser::nodes::expressions::SizeofExpression;

fn parse_sizeof_expr(code: &str) -> Result<SizeofExpression, String> {
    Err(format!("Parser not yet implemented: {}", code))
}

#[test]
fn test_parse_sizeof_expr() {
    let code = "sizeof(int)";
    // let expected = ...;
    // assert_eq!(parse_sizeof_expr(code), Ok(expected));
    assert!(parse_sizeof_expr(code).is_err());
}
