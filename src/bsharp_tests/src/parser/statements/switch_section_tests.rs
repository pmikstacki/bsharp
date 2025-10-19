// Tests for parsing switch sections

use syntax::statements::SwitchSection;

fn parse_switch_section(code: &str) -> Result<SwitchSection, String> {
    Err(format!("Parser not yet implemented: {}", code))
}

#[test]
fn test_parse_switch_section() {
    let code = "case 1: x++; break;";
    // let expected = ...;
    // assert_eq!(parse_switch_section(code.into()), Ok(expected));
    assert!(parse_switch_section(code.into()).is_err());
}
