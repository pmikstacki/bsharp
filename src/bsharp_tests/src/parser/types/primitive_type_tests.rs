// Tests for parsing primitive types

use parser::nodes::types::PrimitiveType;

fn parse_primitive_type(code: &str) -> Result<PrimitiveType, String> {
    Err(format!("Parser not yet implemented: {}", code))
}

#[test]
fn test_parse_primitive_type() {
    let code = "int";
    // let expected = ...;
    // assert_eq!(parse_primitive_type(code), Ok(expected));
    assert!(parse_primitive_type(code).is_err());
}
