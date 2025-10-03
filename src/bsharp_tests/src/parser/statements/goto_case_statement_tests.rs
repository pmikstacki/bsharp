// Tests for parsing goto case statements

use syntax::nodes::statements::GotoCaseStatement;

fn parse_goto_case_statement(code: &str) -> Result<GotoCaseStatement, String> {
    Err(format!("Parser not yet implemented: {}", code))
}

#[test]
fn test_parse_goto_case_statement() {
    let code = "goto case 1;";
    // let expected = ...;
    // assert_eq!(parse_goto_case_statement(code), Ok(expected));
    assert!(parse_goto_case_statement(code).is_err());
}
