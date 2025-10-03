// Tests for parsing type parameters

use parser::nodes::types::Parameter;

fn parse_type_parameter(code: &str) -> Result<Parameter, String> {
    Err(format!("Parser not yet implemented: {}", code))
}

#[test]
fn test_parse_type_parameter() {
    let code = "T";
    // let expected = ...;
    // assert_eq!(parse_type_parameter(code), Ok(expected));
    assert!(parse_type_parameter(code).is_err());
}
