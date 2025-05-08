// Tests for parsing class members

use bsharp::parser::nodes::declarations::ClassMember;

fn parse_class_member(code: &str) -> Result<ClassMember, String> {
    Err(format!("Parser not yet implemented: {}", code))
}

#[test]
fn test_parse_field_member() {
    let code = "int x;";
    // let expected = ClassMember::Field(...);
    // assert_eq!(parse_class_member(code), Ok(expected));
    assert!(parse_class_member(code).is_err());
}

#[test]
fn test_parse_method_member() {
    let code = "void Foo() {}";
    // let expected = ClassMember::Method(...);
    // assert_eq!(parse_class_member(code), Ok(expected));
    assert!(parse_class_member(code).is_err());
}
