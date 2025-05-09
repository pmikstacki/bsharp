use nom::{branch::alt, bytes::complete::tag, character::complete::multispace0, combinator::{map, opt}};

use crate::parser::errors::{BResult, BSharpParseError, CustomErrorKind};
use crate::parser::nodes::declarations::{
    StructDeclaration, StructMember, Modifier
};
use crate::parser::nodes::identifier::Identifier;
use crate::parser::nodes::types::{Type, TypeParameter};
use crate::parser::parser_helpers::{bws, nom_to_bs, keyword};
use crate::parsers::declarations::attribute_parser::parse_attribute_lists;
use crate::parsers::declarations::field_declaration_parser::parse_field_declaration;
use crate::parsers::declarations::method_declaration_parser::parse_method_declaration;
use crate::parsers::declarations::modifier_parser::parse_modifiers;
use crate::parsers::declarations::type_parameter_parser::opt_parse_type_parameter_list;
use crate::parsers::declarations::base_types_parser::parse_base_type_list;
use crate::parsers::identifier_parser::parse_identifier;
use crate::parsers::declarations::type_declaration_helpers::{parse_open_brace, parse_close_brace, at_end_of_body};

/// Parse a struct member (field, method, constructor, etc.)
fn parse_struct_member<'a>(input: &'a str) -> BResult<&'a str, StructMember<'a>> {
    // Try parsing different member types in a specific order
    alt((
        map(parse_field_declaration, StructMember::Field),
        map(nom_to_bs(parse_method_declaration), StructMember::Method),
        // TODO: Add other members like properties, events, indexers, operators
        // as they are implemented
    ))(input)
}

/// Parses a C# struct declaration with full feature support.
/// 
/// # Examples
/// 
/// Basic struct:
/// ```csharp
/// struct Point { int x; int y; }
/// ```
/// 
/// Struct with modifiers and interfaces:
/// ```csharp
/// public struct Measurement : IComparable, IFormattable { ... }
/// ```
/// 
/// Generic struct with attributes:
/// ```csharp
/// [Serializable]
/// public struct KeyValuePair<TKey, TValue> { ... }
/// ```
/// Parses a C# struct declaration with full feature support.
///
/// # Examples
///
/// Basic struct:
/// ```csharp
/// struct Point { int x; int y; }
/// ```
///
/// Struct with modifiers and interfaces:
/// ```csharp
/// public struct Measurement : IComparable, IFormattable { ... }
/// ```
///
/// Generic struct with attributes:
/// ```csharp
/// [Serializable]
/// public struct KeyValuePair<TKey, TValue> { ... }
/// ```
pub fn parse_struct_declaration<'a>(input: &'a str) -> BResult<&'a str, StructDeclaration<'a>> {
    // First parse attributes (can be empty)
    let (input, attributes) = parse_attribute_lists(input)?;
    
    // Parse optional modifiers (public, private, etc.) but NOT the struct keyword itself
    // We're only looking for access/other modifiers that precede the struct keyword
    let (input, modifiers) = parse_modifiers(input)?;
    
    // Parse the 'struct' keyword as a separate token (not a modifier)
    // This should always be present for a struct declaration
    let (input, _) = nom_to_bs(multispace0::<&str, nom::error::Error<&str>>)(input)?;
    let (input, _) = keyword("struct")(input)?;
    
    // Parse the struct name (identifier)
    let (input, _) = nom_to_bs(multispace0::<&str, nom::error::Error<&str>>)(input)?;
    let (input, name) = nom_to_bs(parse_identifier)(input)?;
    
    // Parse optional type parameters like <T> or <K, V>
    let (input, _) = nom_to_bs(multispace0::<&str, nom::error::Error<&str>>)(input)?;
    let (input, type_params_opt) = opt(nom_to_bs(opt_parse_type_parameter_list))(input)?;
    let type_parameters = type_params_opt.and_then(|tp| tp);
    
    // Parse optional base type list (interfaces)
    let (input, _) = nom_to_bs(multispace0::<&str, nom::error::Error<&str>>)(input)?;
    let (input, base_types) = parse_base_type_list(input)?;
    
    // Parse the struct body between { }
    let (input, _) = nom_to_bs(multispace0::<&str, nom::error::Error<&str>>)(input)?;
    let (input, _) = nom_to_bs(tag::<&str, &str, nom::error::Error<&str>>("{"))(input)?; // Fix the incorrect tag function call syntax
    
    // Parse struct members (fields, methods, etc.)
    let mut members = Vec::new();
    let mut current = input;
    
    // Keep parsing members until we hit the closing brace
    while !at_end_of_body(current) {
        match parse_struct_member(current) {
            Ok((rest, member)) => {
                members.push(member);
                current = rest;
            },
            Err(_) => break, // Break if we can't parse more members
        }
    }
    
    // Parse the closing brace
    let (input, _) = parse_close_brace(current)?;
    
    // Return the completed struct declaration
    Ok((input, StructDeclaration {
        attributes,
        modifiers,
        name,
        type_parameters,
        base_types, 
        members,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::nodes::declarations::Modifier;
    use crate::parser::nodes::declarations::field_declaration::FieldDeclaration;
    use crate::parser::nodes::types::{PrimitiveType, TypeParameter};
    use crate::parser::nodes::identifier::Identifier;
    use crate::parser::nodes::types::Variance;

    #[test]
    fn test_simple_struct() {
        let input = "struct MyStruct {}";
        let expected = StructDeclaration {
            attributes: vec![],
            modifiers: vec![],
            name: Identifier::new("MyStruct"),
            type_parameters: None,
            base_types: vec![],
            members: vec![],
        };

        match parse_struct_declaration(input) {
            Ok((remaining, actual)) => {
                assert_eq!(remaining, "");
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
            base_types: vec![],
            members: vec![],
        };

        match parse_struct_declaration(input) {
            Ok((remaining, actual)) => {
                assert_eq!(remaining, "");
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
                variance: crate::parser::nodes::types::Variance::None,
            }]),
            base_types: vec![],
            members: vec![],
        };

        match parse_struct_declaration(input) {
            Ok((remaining, actual)) => {
                assert_eq!(remaining, "");
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
                    variance: crate::parser::nodes::types::Variance::None,
                },
                TypeParameter {
                    name: Identifier::new("V"),
                    variance: crate::parser::nodes::types::Variance::None,
                },
            ]),
            base_types: vec![],
            members: vec![],
        };

        match parse_struct_declaration(input) {
            Ok((remaining, actual)) => {
                assert_eq!(remaining, "");
                assert_eq!(actual, expected);
            }
            Err(e) => panic!("Parsing failed: {:?}", e),
        }
    }

    #[test]
    fn test_struct_with_interface() {
        let input = "struct MyStruct : IDisposable {}";
        let (_, result) = parse_struct_declaration(input).unwrap();
        
        assert_eq!(result.base_types.len(), 1);
        if let Type::Reference(id) = &result.base_types[0] {
            assert_eq!(id.name, "IDisposable");
        } else {
            panic!("Expected Reference type but got {:?}", result.base_types[0]);
        }
    }

    #[test]
    fn test_struct_with_multiple_interfaces() {
        let input = "struct MyStruct : IComparable, IDisposable {}";
        let (_, result) = parse_struct_declaration(input).unwrap();
        
        assert_eq!(result.base_types.len(), 2);
        
        if let Type::Reference(id) = &result.base_types[0] {
            assert_eq!(id.name, "IComparable");
        } else {
            panic!("Expected Reference type");
        }
        
        if let Type::Reference(id) = &result.base_types[1] {
            assert_eq!(id.name, "IDisposable");
        } else {
            panic!("Expected Reference type");
        }
    }

    #[test]
    fn test_struct_with_field() {
        let input = "struct Point { int x; }";
        let (_, result) = parse_struct_declaration(input).unwrap();
        
        assert_eq!(result.members.len(), 1);
        match &result.members[0] {
            StructMember::Field(field) => {
                assert_eq!(field.name.name, "x");
                match &field.ty {
                    Type::Primitive(pt) => assert_eq!(*pt, PrimitiveType::Int),
                    _ => panic!("Expected primitive type"),
                }
            },
            _ => panic!("Expected field member"),
        };
    }

    #[test]
    fn test_struct_with_attribute() {
        let input = "[Serializable] struct MyStruct {}";
        let (_, result) = parse_struct_declaration(input).unwrap();
        
        assert_eq!(result.attributes.len(), 1);
        assert_eq!(result.attributes[0].attributes.len(), 1);
        assert_eq!(result.attributes[0].attributes[0].name.name, "Serializable");
    }
}
