// Tests for parsing pattern expressions

use syntax::expressions::Pattern;

fn parse_pattern(code: &str) -> Result<Pattern, String> {
    Err(format!("Parser not yet implemented: {}", code))
}

#[test]
fn test_parse_pattern() {
    let code = "x is int";
    // let expected = ...;
    // assert_eq!(parse_pattern(code.into()), Ok(expected));
    assert!(parse_pattern(code.into()).is_err());
}
