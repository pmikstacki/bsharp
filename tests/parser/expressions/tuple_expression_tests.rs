// Tests for parsing tuple expressions

use bsharp::parser::nodes::expressions::TupleExpression;

fn parse_tuple_expr(code: &str) -> Result<TupleExpression, String> {
    Err(format!("Parser not yet implemented: {}", code))
}

#[test]
fn test_parse_tuple_expr() {
    let code = "(1, 2)";
    // let expected = ...;
    // assert_eq!(parse_tuple_expr(code), Ok(expected));
    assert!(parse_tuple_expr(code).is_err());
}
