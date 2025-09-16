// Tests for parsing parameters

use bsharp::syntax::nodes::types::Parameter;

fn parse_parameter(code: &str) -> Result<Parameter, String> {
    Err(format!("Parser not yet implemented: {}", code))
}

#[test]
fn test_parse_parameter() {
    let code = "int x";
    // let expected = ...;
    // assert_eq!(parse_parameter(code), Ok(expected));
    assert!(parse_parameter(code).is_err());
}
