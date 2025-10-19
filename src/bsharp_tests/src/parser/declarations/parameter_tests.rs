// Tests for parsing parameters

use syntax::types::Parameter;

fn parse_parameter(code: &str) -> Result<Parameter, String> {
    Err(format!("Parser not yet implemented: {}", code))
}

#[test]
fn test_parse_parameter() {
    let code = "int x";
    // let expected = ...;
    // assert_eq!(parse_parameter(code.into()), Ok(expected));
    assert!(parse_parameter(code.into()).is_err());
}
