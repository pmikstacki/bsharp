// Tests for parsing unary operators

use syntax::expressions::UnaryOperator;

fn parse_unary_operator(code: &str) -> Result<UnaryOperator, String> {
    Err(format!("Parser not yet implemented: {}", code))
}

#[test]
fn test_parse_unary_operator() {
    let code = "-";
    // let expected = ...;
    // assert_eq!(parse_unary_operator(code), Ok(expected));
    assert!(parse_unary_operator(code).is_err());
}
