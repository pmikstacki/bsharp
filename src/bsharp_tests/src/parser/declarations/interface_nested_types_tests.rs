#![cfg(test)]

use parser::expressions::declarations::type_declaration_parser::parse_interface_declaration;
use syntax::nodes::declarations::InterfaceBodyDeclaration;

// C# 8.0+ allows nested types in interfaces
// These tests verify that the parser correctly handles nested types

#[test]
fn test_interface_nested_class_accepted() {
    let code = r#"
interface ITest {
    class NestedClass {}
}
"#;
    let result = parse_interface_declaration(code);
    assert!(
        result.is_ok(),
        "Nested class in interface should be accepted (C# 8.0+)"
    );
    let (_, interface) = result.unwrap();
    assert_eq!(interface.body_declarations.len(), 1);
    match &interface.body_declarations[0] {
        InterfaceBodyDeclaration::NestedClass(_) => {}
        _ => panic!("Expected nested class"),
    }
}

#[test]
fn test_interface_nested_struct_accepted() {
    let code = r#"
interface ITest {
    struct NestedStruct {}
}
"#;
    let result = parse_interface_declaration(code);
    assert!(
        result.is_ok(),
        "Nested struct in interface should be accepted (C# 8.0+)"
    );
    let (_, interface) = result.unwrap();
    assert_eq!(interface.body_declarations.len(), 1);
    match &interface.body_declarations[0] {
        InterfaceBodyDeclaration::NestedStruct(_) => {}
        _ => panic!("Expected nested struct"),
    }
}

#[test]
fn test_interface_nested_interface_accepted() {
    let code = r#"
interface IOuter {
    interface IInner {}
}
"#;
    let result = parse_interface_declaration(code);
    assert!(
        result.is_ok(),
        "Nested interface in interface should be accepted (C# 8.0+)"
    );
    let (_, interface) = result.unwrap();
    assert_eq!(interface.body_declarations.len(), 1);
    match &interface.body_declarations[0] {
        InterfaceBodyDeclaration::NestedInterface(_) => {}
        _ => panic!("Expected nested interface"),
    }
}

#[test]
fn test_interface_nested_enum_accepted() {
    let code = r#"
interface ITest {
    enum NestedEnum {}
}
"#;
    let result = parse_interface_declaration(code);
    assert!(
        result.is_ok(),
        "Nested enum in interface should be accepted (C# 8.0+)"
    );
    let (_, interface) = result.unwrap();
    assert_eq!(interface.body_declarations.len(), 1);
    match &interface.body_declarations[0] {
        InterfaceBodyDeclaration::NestedEnum(_) => {}
        _ => panic!("Expected nested enum"),
    }
}

#[test]
fn test_interface_nested_record_accepted() {
    let code = r#"
interface ITest {
    record NestedRecord {}
}
"#;
    let result = parse_interface_declaration(code);
    assert!(
        result.is_ok(),
        "Nested record in interface should be accepted (C# 8.0+)"
    );
    let (_, interface) = result.unwrap();
    assert_eq!(interface.body_declarations.len(), 1);
    match &interface.body_declarations[0] {
        InterfaceBodyDeclaration::NestedRecord(_) => {}
        _ => panic!("Expected nested record"),
    }
}

#[test]
fn test_interface_with_method_accepted() {
    let code = r#"
interface ITest {
    void Method();
}
"#;
    let result = parse_interface_declaration(code);
    assert!(result.is_ok(), "Interface with method should be accepted");
    let (_, interface) = result.unwrap();
    assert_eq!(interface.body_declarations.len(), 1);
}

#[test]
fn test_interface_with_property_accepted() {
    let code = r#"
interface ITest {
    int Property { get; set; }
}
"#;
    let result = parse_interface_declaration(code);
    assert!(result.is_ok(), "Interface with property should be accepted");
    let (_, interface) = result.unwrap();
    assert_eq!(interface.body_declarations.len(), 1);
}

#[test]
fn test_interface_with_event_accepted() {
    let code = r#"
interface ITest {
    event EventHandler MyEvent;
}
"#;
    let result = parse_interface_declaration(code);
    assert!(result.is_ok(), "Interface with event should be accepted");
    let (_, interface) = result.unwrap();
    assert_eq!(interface.body_declarations.len(), 1);
}

#[test]
fn test_interface_with_indexer_accepted() {
    let code = r#"
interface ITest {
    int this[int index] { get; set; }
}
"#;
    let result = parse_interface_declaration(code);
    assert!(result.is_ok(), "Interface with indexer should be accepted");
    let (_, interface) = result.unwrap();
    assert_eq!(interface.body_declarations.len(), 1);
}

#[test]
fn test_interface_with_multiple_valid_members() {
    let code = r#"
interface ITest {
    void Method();
    int Property { get; }
    event EventHandler Event;
}
"#;
    let result = parse_interface_declaration(code);
    assert!(
        result.is_ok(),
        "Interface with multiple valid members should be accepted"
    );
    let (_, interface) = result.unwrap();
    assert_eq!(interface.body_declarations.len(), 3);
}
