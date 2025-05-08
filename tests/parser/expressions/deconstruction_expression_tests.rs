// Tests for parsing deconstruction expressions

use bsharp::parser::nodes::expressions::DeconstructionExpression;

fn parse_deconstruction_expr(code: &str) -> Result<DeconstructionExpression, String> {
    Err(format!("Parser not yet implemented: {}", code))
}

#[test]
fn test_parse_deconstruction_expr() {
    let code = "var (x, y) = point;";
    // let expected = ...;
    // assert_eq!(parse_deconstruction_expr(code), Ok(expected));
    assert!(parse_deconstruction_expr(code).is_err());
}
