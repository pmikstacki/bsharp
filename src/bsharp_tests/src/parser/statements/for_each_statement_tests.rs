// Tests for parsing foreach statements

use syntax::statements::ForEachStatement;

fn parse_for_each_statement(code: &str) -> Result<ForEachStatement, String> {
    Err(format!("Parser not yet implemented: {}", code))
}

#[test]
fn test_parse_for_each_statement() {
    let code = "foreach (var x in xs) { }";
    // let expected = ...;
    // assert_eq!(parse_for_each_statement(code.into()), Ok(expected));
    assert!(parse_for_each_statement(code.into()).is_err());
}
