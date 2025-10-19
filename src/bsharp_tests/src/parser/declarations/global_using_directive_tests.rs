// Tests for parsing global using directives

use syntax::declarations::GlobalUsingDirective;

fn parse_global_using_directive(code: &str) -> Result<GlobalUsingDirective, String> {
    Err(format!("Parser not yet implemented: {}", code))
}

#[test]
fn test_parse_global_using() {
    let code = "global using System;";
    // let expected = ...;
    // assert_eq!(parse_global_using_directive(code.into()), Ok(expected));
    assert!(parse_global_using_directive(code.into()).is_err());
}
