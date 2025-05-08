// Tests for parsing binary operators

use bsharp::parser::nodes::expressions::BinaryOperator;

fn parse_binary_operator(code: &str) -> Result<BinaryOperator, String> {
    Err(format!("Parser not yet implemented: {}", code))
}

#[test]
fn test_parse_binary_operator() {
    let code = "+";
    // let expected = ...;
    // assert_eq!(parse_binary_operator(code), Ok(expected));
    assert!(parse_binary_operator(code).is_err());
}
