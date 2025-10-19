#![cfg(test)]
use parser::expressions::declarations::type_declaration_parser::parse_struct_declaration_span as parse_struct_declaration;
use syntax::declarations::{Modifier, StructBodyDeclaration, StructDeclaration};
use syntax::identifier::Identifier;
use syntax::types::{PrimitiveType, Type, TypeParameter, Variance};

#[test]
fn test_simple_struct() {
    let input = "struct MyStruct {}";
    let expected = StructDeclaration {
        attributes: vec![],
        modifiers: vec![],
        name: Identifier::new("MyStruct"),
        type_parameters: None,
        primary_constructor_parameters: None,
        base_types: vec![],
        body_declarations: vec![],
        constraints: None,
    };

    match parse_struct_declaration(input.into()) {
        Ok((remaining, actual)) => {
            assert_eq!(*remaining.fragment(), "");
            assert_eq!(actual, expected);
        }
        Err(e) => panic!("Parsing failed: {:?}", e),
    }
}

#[test]
fn test_public_struct() {
    let input = "public struct MyPublicStruct {}";
    let expected = StructDeclaration {
        attributes: vec![],
        modifiers: vec![Modifier::Public],
        name: Identifier::new("MyPublicStruct"),
        type_parameters: None,
        primary_constructor_parameters: None,
        base_types: vec![],
        body_declarations: vec![],
        constraints: None,
    };

    match parse_struct_declaration(input.into()) {
        Ok((remaining, actual)) => {
            assert!(remaining.fragment().trim().is_empty());
            assert_eq!(actual, expected);
        }
        Err(e) => panic!("Parsing failed: {:?}", e),
    }
}

#[test]
fn test_struct_with_single_generic_parameter() {
    let input = "struct MyGenericStruct<T> {}";
    let expected = StructDeclaration {
        attributes: vec![],
        modifiers: vec![],
        name: Identifier::new("MyGenericStruct"),
        type_parameters: Some(vec![TypeParameter {
            name: Identifier::new("T"),
            variance: Variance::None,
        }]),
        primary_constructor_parameters: None,
        base_types: vec![],
        body_declarations: vec![],
        constraints: None,
    };

    match parse_struct_declaration(input.into()) {
        Ok((remaining, actual)) => {
            assert!(remaining.fragment().trim().is_empty());
            assert_eq!(actual, expected);
        }
        Err(e) => panic!("Parsing failed: {:?}", e),
    }
}

#[test]
fn test_public_struct_with_multiple_generic_parameters() {
    let input = "public struct MyComplexStruct<K, V> {}";
    let expected = StructDeclaration {
        attributes: vec![],
        modifiers: vec![Modifier::Public],
        name: Identifier::new("MyComplexStruct"),
        type_parameters: Some(vec![
            TypeParameter {
                name: Identifier::new("K"),
                variance: Variance::None,
            },
            TypeParameter {
                name: Identifier::new("V"),
                variance: Variance::None,
            },
        ]),
        primary_constructor_parameters: None,
        base_types: vec![],
        body_declarations: vec![],
        constraints: None,
    };

    match parse_struct_declaration(input.into()) {
        Ok((remaining, actual)) => {
            assert!(remaining.fragment().trim().is_empty());
            assert_eq!(actual, expected);
        }
        Err(e) => panic!("Parsing failed: {:?}", e),
    }
}

#[test]
fn test_struct_with_interface() {
    let input = "struct MyStruct : IDisposable {}";
    let (_, result) = parse_struct_declaration(input.into()).unwrap();

    assert_eq!(result.base_types.len(), 1);
    if let Type::Reference(id) = &result.base_types[0] {
        assert_eq!(id.to_string(), "IDisposable");
    } else {
        panic!("Expected Reference type but got {:?}", result.base_types[0]);
    }
}

#[test]
fn test_struct_with_multiple_interfaces() {
    let input = "struct MyStruct : IComparable, IDisposable {}";
    let (_, result) = parse_struct_declaration(input.into()).unwrap();

    assert_eq!(result.base_types.len(), 2);

    if let Type::Reference(id) = &result.base_types[0] {
        assert_eq!(id.to_string(), "IComparable");
    } else {
        panic!("Expected Reference type");
    }

    if let Type::Reference(id) = &result.base_types[1] {
        assert_eq!(id.to_string(), "IDisposable");
    } else {
        panic!("Expected Reference type");
    }
}

#[test]
fn test_struct_with_field() {
    let input = "struct Point { int x; }";
    let (_, result) = parse_struct_declaration(input.into()).unwrap();

    assert_eq!(result.body_declarations.len(), 1);
    match &result.body_declarations[0] {
        StructBodyDeclaration::Field(field) => {
            assert_eq!(field.name.to_string(), "x");
            match &field.field_type {
                Type::Primitive(pt) => assert_eq!(*pt, PrimitiveType::Int),
                _ => panic!("Expected primitive type"),
            }
        }
        _ => panic!("Expected field member"),
    };
}

#[test]
fn test_struct_with_attribute() {
    let input = "[Serializable] struct MyStruct {}";
    let (_, result) = parse_struct_declaration(input.into()).unwrap();

    assert_eq!(result.attributes.len(), 1);
    assert_eq!(result.attributes[0].attributes.len(), 1);
    assert_eq!(result.attributes[0].attributes[0].name.to_string(), "Serializable");
}
