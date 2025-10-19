// Tests for parsing global attribute declarations

use parser::expressions::declarations::global_attribute_parser::parse_global_attribute;
use syntax::declarations::{Attribute, GlobalAttribute};
use syntax::identifier::Identifier;

#[test]
fn test_parse_global_attribute() {
    let code = "[assembly: MyAttr]";
    let result = parse_global_attribute(code.into());
    assert!(
        result.is_ok(),
        "Failed to parse global attribute: {:?}",
        result
    );

    let (remaining, actual) = result.unwrap();
    assert!(remaining.fragment().trim().is_empty());
    assert_eq!(actual.target.to_string(), "assembly");
    assert_eq!(actual.attribute.name.to_string(), "MyAttr");
    assert!(actual.attribute.arguments.is_empty());
    assert!(actual.attribute.structured.is_some());
}

#[test]
fn test_parse_module_attribute() {
    let code = "[module: TestAttribute]";
    let result = parse_global_attribute(code.into());
    assert!(
        result.is_ok(),
        "Failed to parse module attribute: {:?}",
        result
    );

    let (remaining, attr) = result.unwrap();
    assert!(remaining.fragment().trim().is_empty());
    assert_eq!(attr.target.to_string(), "module");
    assert_eq!(attr.attribute.name.to_string(), "TestAttribute");
    assert!(attr.attribute.arguments.is_empty());
    assert!(attr.attribute.structured.is_some());
}

#[test]
fn test_parse_assembly_attribute_with_arguments() {
    let code = "[assembly: AssemblyVersion(\"1.0.0.0\")]";
    let result = parse_global_attribute(code.into());
    assert!(
        result.is_ok(),
        "Failed to parse assembly attribute with arguments: {:?}",
        result
    );

    let (remaining, attr) = result.unwrap();
    assert!(remaining.fragment().trim().is_empty());
    assert_eq!(attr.target.to_string(), "assembly");
    assert_eq!(attr.attribute.name.to_string(), "AssemblyVersion");
    assert_eq!(attr.attribute.arguments.len(), 1);
    assert!(attr.attribute.structured.is_some());
}
