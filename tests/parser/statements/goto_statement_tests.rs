// Tests for parsing goto statements

use bsharp::parser::nodes::statements::GotoStatement;

fn parse_goto_statement(code: &str) -> Result<GotoStatement, String> {
    Err(format!("Parser not yet implemented: {}", code))
}

#[test]
fn test_parse_goto_statement() {
    let code = "goto label;";
    // let expected = ...;
    // assert_eq!(parse_goto_statement(code), Ok(expected));
    assert!(parse_goto_statement(code).is_err());
}
