// Tests for parsing pattern expressions

use bsharp::parser::nodes::expressions::Pattern;

fn parse_pattern(code: &str) -> Result<Pattern, String> {
    Err(format!("Parser not yet implemented: {}", code))
}

#[test]
fn test_parse_pattern() {
    let code = "x is int";
    // let expected = ...;
    // assert_eq!(parse_pattern(code), Ok(expected));
    assert!(parse_pattern(code).is_err());
}
