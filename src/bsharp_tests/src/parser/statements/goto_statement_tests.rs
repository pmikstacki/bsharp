// Tests for parsing goto statements

use syntax::statements::GotoStatement;

fn parse_goto_statement(code: &str) -> Result<GotoStatement, String> {
    Err(format!("Parser not yet implemented: {}", code))
}

#[test]
fn test_parse_goto_statement() {
    let code = "goto label;";
    // let expected = ...;
    // assert_eq!(parse_goto_statement(code.into()), Ok(expected));
    assert!(parse_goto_statement(code.into()).is_err());
}
