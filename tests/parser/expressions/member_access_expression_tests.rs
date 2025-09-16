// Tests for parsing member access expressions

use bsharp::syntax::nodes::expressions::MemberAccessExpression;

fn parse_member_access_expr(code: &str) -> Result<MemberAccessExpression, String> {
    Err(format!("Parser not yet implemented: {}", code))
}

#[test]
fn test_parse_member_access_expr() {
    let code = "foo.bar";
    // let expected = ...;
    // assert_eq!(parse_member_access_expr(code), Ok(expected));
    assert!(parse_member_access_expr(code).is_err());
}
