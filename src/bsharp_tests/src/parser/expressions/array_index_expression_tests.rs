// Tests for parsing array index expressions

use syntax::expressions::ArrayIndexExpression;

fn parse_array_index_expr(code: &str) -> Result<ArrayIndexExpression, String> {
    Err(format!("Parser not yet implemented: {}", code))
}

#[test]
fn test_parse_array_index_expr() {
    let code = "arr[0]";
    // let expected = ...;
    // assert_eq!(parse_array_index_expr(code.into()), Ok(expected));
    assert!(parse_array_index_expr(code.into()).is_err());
}
