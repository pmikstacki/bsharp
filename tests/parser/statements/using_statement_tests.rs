// Tests for parsing using statements

use bsharp::parser::nodes::statements::UsingStatement;

fn parse_using_statement(code: &str) -> Result<UsingStatement, String> {
    Err(format!("Parser not yet implemented: {}", code))
}

#[test]
fn test_parse_using_statement() {
    let code = "using (var x = foo()) { }";
    // let expected = ...;
    // assert_eq!(parse_using_statement(code), Ok(expected));
    assert!(parse_using_statement(code).is_err());
}
