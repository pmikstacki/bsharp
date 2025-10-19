// Tests for parsing switch labels

use syntax::statements::SwitchLabel;

fn parse_switch_label(code: &str) -> Result<SwitchLabel, String> {
    Err(format!("Parser not yet implemented: {}", code))
}

#[test]
fn test_parse_switch_label() {
    let code = "case 1:";
    // let expected = ...;
    // assert_eq!(parse_switch_label(code.into()), Ok(expected));
    assert!(parse_switch_label(code.into()).is_err());
}
