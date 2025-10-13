// Tests for parsing global attribute declarations

use parser::expressions::declarations::global_attribute_parser::parse_global_attribute;
use syntax::declarations::{Attribute, GlobalAttribute};
use syntax::identifier::Identifier;

#[test]
fn test_parse_global_attribute() {
    let code = "[assembly: MyAttr]";
    let expected = GlobalAttribute {
        target: Identifier {
            name: "assembly".to_string(),
        },
        attribute: Attribute {
            name: Identifier {
                name: "MyAttr".to_string(),
            },
            arguments: vec![],
            structured: Some(syntax::declarations::attribute::AttributeName {
                qualifier: vec![],
                name: Identifier {
                    name: "MyAttr".to_string(),
                },
                type_arguments: vec![],
            }),
        },
    };

    let result = parse_global_attribute(code);
    assert!(
        result.is_ok(),
        "Failed to parse global attribute: {:?}",
        result
    );

    let (remaining, actual) = result.unwrap();
    assert_eq!(remaining, "");
    assert_eq!(actual, expected);
}

#[test]
fn test_parse_module_attribute() {
    let code = "[module: TestAttribute]";
    let result = parse_global_attribute(code);
    assert!(
        result.is_ok(),
        "Failed to parse module attribute: {:?}",
        result
    );

    let (remaining, attr) = result.unwrap();
    assert_eq!(remaining, "");
    assert_eq!(attr.target.name, "module");
    assert_eq!(attr.attribute.name.name, "TestAttribute");
    assert!(attr.attribute.arguments.is_empty());
    assert!(attr.attribute.structured.is_some());
}

#[test]
fn test_parse_assembly_attribute_with_arguments() {
    let code = "[assembly: AssemblyVersion(\"1.0.0.0\")]";
    let result = parse_global_attribute(code);
    assert!(
        result.is_ok(),
        "Failed to parse assembly attribute with arguments: {:?}",
        result
    );

    let (remaining, attr) = result.unwrap();
    assert_eq!(remaining, "");
    assert_eq!(attr.target.name, "assembly");
    assert_eq!(attr.attribute.name.name, "AssemblyVersion");
    assert_eq!(attr.attribute.arguments.len(), 1);
    assert!(attr.attribute.structured.is_some());
}
