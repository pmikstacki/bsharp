// Tests for parsing label statements

use syntax::nodes::statements::LabelStatement;

fn parse_label_statement(code: &str) -> Result<LabelStatement, String> {
    Err(format!("Parser not yet implemented: {}", code))
}

#[test]
fn test_parse_label_statement() {
    let code = "myLabel:";
    // let expected = ...;
    // assert_eq!(parse_label_statement(code), Ok(expected));
    assert!(parse_label_statement(code).is_err());
}
