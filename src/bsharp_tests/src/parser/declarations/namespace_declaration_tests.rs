// Tests for parsing namespace declarations

use parser::Parsable;
use parser::expressions::declarations::namespace_declaration_parser::parse_namespace_declaration;
use syntax::declarations::NamespaceDeclaration;

// Local test helper to avoid import issues

#[test]
fn test_simple_namespace_declaration() {
    let input = "namespace MyNamespace { }";
    let result = NamespaceDeclaration::parse(input.into());
    assert!(result.is_ok());
    let (_remaining, decl) = result.unwrap();
    assert_eq!(decl.node.name.to_string(), "MyNamespace");
}

#[test]
fn test_qualified_namespace_declaration() {
    let input = "namespace System.Collections { }";
    let result = NamespaceDeclaration::parse(input.into());
    assert!(result.is_ok());
    let (_remaining, decl) = result.unwrap();
    assert_eq!(decl.node.name.to_string(), "System.Collections");
}

#[test]
fn test_parse_namespace() {
    let code = "namespace MyNs { }";
    let result = NamespaceDeclaration::parse(code.into());
    assert!(result.is_ok(), "Expected successful parsing of namespace");
    let (_remaining, decl) = result.unwrap();
    assert_eq!(decl.node.name.to_string(), "MyNs");
}
