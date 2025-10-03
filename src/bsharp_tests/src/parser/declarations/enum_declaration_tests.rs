// Tests for parsing enum declarations

use parser::expressions::declarations::enum_declaration_parser::parse_enum_declaration;
use parser::syntax::errors;
use syntax::nodes::declarations::Modifier;
use syntax::nodes::expressions::expression::Expression;
use syntax::nodes::expressions::literal::Literal;
use syntax::nodes::types::{PrimitiveType, Type};

// Local test helper to avoid import issues
fn parse_full_input<'a, O, F>(input: &'a str, parser: F) -> Result<(&'a str, O), String>
where
    F: FnOnce(&'a str) -> errors::BResult<&'a str, O>,
{
    match parser(input) {
        Ok((remaining, result)) => Ok((remaining, result)),
        Err(err) => Err(format!("Parse error: {:?}", err)),
    }
}

#[test]
fn test_simple_enum_declaration() {
    let input = "enum MyEnum { }";
    let result = parse_full_input(input, parse_enum_declaration);
    assert!(result.is_ok());
    let (_remaining, decl) = result.unwrap();
    assert_eq!(decl.name.name, "MyEnum");
    assert!(decl.attributes.is_empty());
    assert!(decl.modifiers.is_empty());
    assert!(decl.underlying_type.is_none());
    assert!(decl.enum_members.is_empty());
}

#[test]
fn test_enum_with_members() {
    let input = "enum Direction { North, East, South, West }";
    let result = parse_full_input(input, parse_enum_declaration);
    assert!(result.is_ok());
    let (_remaining, decl) = result.unwrap();
    assert_eq!(decl.name.name, "Direction");
    assert_eq!(decl.enum_members.len(), 4);
    assert_eq!(decl.enum_members[0].name.name, "North");
    assert_eq!(decl.enum_members[1].name.name, "East");
    assert_eq!(decl.enum_members[2].name.name, "South");
    assert_eq!(decl.enum_members[3].name.name, "West");
}

#[test]
fn test_enum_with_values() {
    let input = "enum ErrorCode { Success = 0, NotFound = 404, ServerError = 500 }";
    let result = parse_full_input(input, parse_enum_declaration);
    assert!(result.is_ok());
    let (_remaining, decl) = result.unwrap();
    assert_eq!(decl.name.name, "ErrorCode");
    assert_eq!(decl.enum_members.len(), 3);

    // Check that values were parsed correctly
    assert_eq!(decl.enum_members[0].name.name, "Success");
    if let Some(Expression::Literal(Literal::Integer(0))) = decl.enum_members[0].value {
        // Success
    } else {
        panic!("Expected integer literal 0");
    }

    assert_eq!(decl.enum_members[1].name.name, "NotFound");
    if let Some(Expression::Literal(Literal::Integer(404))) = decl.enum_members[1].value {
        // Success
    } else {
        panic!("Expected integer literal 404");
    }
}

#[test]
fn test_enum_with_underlying_type() {
    let input = "enum IntFlags : int { None = 0, Flag1 = 1, Flag2 = 2, Flag3 = 4 }";
    let result = parse_full_input(input, parse_enum_declaration);
    assert!(result.is_ok());
    let (_remaining, decl) = result.unwrap();
    assert_eq!(decl.name.name, "IntFlags");

    // Check underlying type
    assert!(decl.underlying_type.is_some());
    if let Some(Type::Primitive(primitive)) = decl.underlying_type {
        assert_eq!(primitive, PrimitiveType::Int);
    } else {
        panic!("Expected int primitive type");
    }

    // Check members
    assert_eq!(decl.enum_members.len(), 4);
}

#[test]
fn test_parse_enum_with_attributes_modifiers_and_base_type() {
    let code = "[Flags] public enum MyEnum : int { A, B }";
    let result = parse_full_input(code, parse_enum_declaration);
    assert!(result.is_ok(), "Parsing failed: {:?}", result.err());
    let (_remaining, decl) = result.unwrap();

    assert_eq!(decl.name.name, "MyEnum");

    // Check attributes
    assert_eq!(decl.attributes.len(), 1, "Expected 1 attribute list");
    assert!(
        !decl.attributes[0].attributes.is_empty(),
        "Expected attributes in the list"
    );
    assert_eq!(
        decl.attributes[0].attributes[0].name.name, "Flags",
        "Attribute name mismatch"
    );

    // Check modifiers
    assert_eq!(decl.modifiers.len(), 1, "Expected 1 modifier");
    assert_eq!(decl.modifiers[0], Modifier::Public, "Modifier mismatch");

    // Check underlying type
    assert!(
        decl.underlying_type.is_some(),
        "Expected an underlying type"
    );
    if let Some(Type::Primitive(primitive)) = &decl.underlying_type {
        assert_eq!(*primitive, PrimitiveType::Int, "Underlying type mismatch");
    } else {
        panic!(
            "Expected int primitive type, got {:?}",
            decl.underlying_type
        );
    }

    assert_eq!(decl.enum_members.len(), 2, "Member count mismatch");
}
