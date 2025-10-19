#![cfg(test)]

use std::any::Any;
use nom::Offset;
use parser::expressions::declarations::base_types_parser::*;
use syntax::types::Type;

#[test]
fn test_no_base_types() {
    let input = "{ }";
    let (rest, types) = parse_base_type_list(input.into()).unwrap();
    assert_eq!(rest.fragment().to_string(), "{ }".to_string());
    assert!(types.is_empty());
}

#[test]
fn test_single_interface() {
    let input = ": IDisposable { }";
    let (rest, types) = parse_base_type_list(input.into()).unwrap();
    assert_eq!(rest.fragment().to_string(), "{ }".to_string());
    assert_eq!(types.len(), 1);

    if let Type::Reference(id) = &types[0] {
        assert_eq!(id.to_string(), "IDisposable");
    } else {
        panic!("Expected Reference type but got {:?}", types[0]);
    }
}

#[test]
fn test_multiple_interfaces() {
    let input = ": IComparable, IEnumerable, IDisposable { }";
    let (rest, types) = parse_base_type_list(input.into()).unwrap();
    assert_eq!(rest.fragment().to_string(), "{ }".to_string());
    assert_eq!(types.len(), 3);

    if let Type::Reference(id) = &types[0] {
        assert_eq!(id.to_string(), "IComparable");
    } else {
        panic!("Expected Reference type but got {:?}", types[0]);
    }

    if let Type::Reference(id) = &types[1] {
        assert_eq!(id.to_string(), "IEnumerable");
    } else {
        panic!("Expected Reference type but got {:?}", types[1]);
    }

    if let Type::Reference(id) = &types[2] {
        assert_eq!(id.to_string(), "IDisposable");
    } else {
        panic!("Expected Reference type but got {:?}", types[2]);
    }
}

#[test]
fn test_generic_interface() {
    let input = ": IEnumerable<string> { }";
    let (rest, types) = parse_base_type_list(input.into()).unwrap();
    assert_eq!(rest.to_string(), "{ }");
    assert_eq!(types.len(), 1);

    if let Type::Generic { base, args } = &types[0] {
        assert_eq!(base.to_string(), "IEnumerable");
        assert_eq!(args.len(), 1);
        match &args[0] {
            Type::Primitive(prim) => assert_eq!(format!("{:?}", prim), "String"),
            _ => panic!("Expected string primitive type"),
        }
    } else {
        panic!("Expected Generic type but got {:?}", types[0]);
    }
}

#[test]
fn test_whitespace_variations() {
    // Extra whitespace around colon
    let input = "  :  IDisposable { }";
    let (rest, types) = parse_base_type_list(input.into()).unwrap();
    assert_eq!(rest.fragment().to_string(), "{ }".to_string());
    assert_eq!(types.len(), 1);

    // Extra whitespace around comma
    let input = ": IComparable  ,  IDisposable { }";
    let (rest, types) = parse_base_type_list(input.into()).unwrap();
    assert_eq!(rest.fragment().to_string(), "{ }".to_string());
    assert_eq!(types.len(), 2);
}

#[test]
fn test_qualified_interface_name() {
    let input = ": System.Collections.IEnumerable { }";
    let (rest, types) = parse_base_type_list(input.into()).unwrap();
    assert_eq!(rest.fragment().to_string(), "{ }");
    assert_eq!(types.len(), 1);

    if let Type::Reference(id) = &types[0] {
        assert_eq!(id.to_string(), "System.Collections.IEnumerable");
    } else {
        panic!("Expected Reference type but got {:?}", types[0]);
    }
}
