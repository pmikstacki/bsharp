#![cfg(test)]
use bsharp::parsers::declarations::base_types_parser::*;
use bsharp::parser::nodes::identifier::Identifier;
use bsharp::parser::nodes::types::Type;

#[test]
fn test_no_base_types() {
    let input = "{ }";
    let (rest, types) = parse_base_type_list(input).unwrap();
    assert_eq!(rest, "{ }");
    assert!(types.is_empty());
}

#[test]
fn test_single_interface() {
    let input = ": IDisposable { }";
    let (rest, types) = parse_base_type_list(input).unwrap();
    assert_eq!(rest, "{ }");
    assert_eq!(types.len(), 1);
    
    if let Type::Reference(id) = &types[0] {
        assert_eq!(id.name, "IDisposable");
    } else {
        panic!("Expected Reference type but got {:?}", types[0]);
    }
}

#[test]
fn test_multiple_interfaces() {
    let input = ": IComparable, IEnumerable, IDisposable { }";
    let (rest, types) = parse_base_type_list(input).unwrap();
    assert_eq!(rest, "{ }");
    assert_eq!(types.len(), 3);
    
    if let Type::Reference(id) = &types[0] {
        assert_eq!(id.name, "IComparable");
    } else {
        panic!("Expected Reference type but got {:?}", types[0]);
    }
    
    if let Type::Reference(id) = &types[1] {
        assert_eq!(id.name, "IEnumerable");
    } else {
        panic!("Expected Reference type but got {:?}", types[1]);
    }
    
    if let Type::Reference(id) = &types[2] {
        assert_eq!(id.name, "IDisposable");
    } else {
        panic!("Expected Reference type but got {:?}", types[2]);
    }
}

#[test]
fn test_generic_interface() {
    let input = ": IEnumerable<string> { }";
    let (rest, types) = parse_base_type_list(input).unwrap();
    assert_eq!(rest, "{ }");
    assert_eq!(types.len(), 1);
    
    if let Type::Generic { base, args } = &types[0] {
        assert_eq!(base.name, "IEnumerable");
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
    let (rest, types) = parse_base_type_list(input).unwrap();
    assert_eq!(rest, "{ }");
    assert_eq!(types.len(), 1);
    
    // Extra whitespace around comma
    let input = ": IComparable  ,  IDisposable { }";
    let (rest, types) = parse_base_type_list(input).unwrap();
    assert_eq!(rest, "{ }");
    assert_eq!(types.len(), 2);
}

#[test]
fn test_qualified_interface_name() {
    let input = ": System.Collections.IEnumerable { }";
    let (rest, types) = parse_base_type_list(input).unwrap();
    assert_eq!(rest, "{ }");
    assert_eq!(types.len(), 1);
    
    if let Type::Reference(id) = &types[0] {
        assert_eq!(id.name, "System.Collections.IEnumerable");
    } else {
        panic!("Expected Reference type but got {:?}", types[0]);
    }
}
