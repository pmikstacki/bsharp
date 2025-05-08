// Tests for parsing query expressions

use bsharp::parser::nodes::expressions::QueryExpression;

fn parse_query_expr(code: &str) -> Result<QueryExpression, String> {
    Err(format!("Parser not yet implemented: {}", code))
}

#[test]
fn test_parse_query_expr() {
    let code = "from x in xs select x";
    // let expected = ...;
    // assert_eq!(parse_query_expr(code), Ok(expected));
    assert!(parse_query_expr(code).is_err());
}
