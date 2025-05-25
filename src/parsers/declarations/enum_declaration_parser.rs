use nom::character::complete::char;
use nom::combinator::opt;
use nom::multi::separated_list0;
use nom::sequence::{preceded, tuple};

use crate::parser::errors::BResult;
use crate::parser::nodes::declarations::EnumDeclaration;
use crate::parser::nodes::declarations::enum_declaration::EnumMember;
use crate::parser::parser_helpers::{bws, nom_to_bs, keyword};
use crate::parsers::declarations::attribute_parser::parse_attribute_lists;
use crate::parsers::declarations::modifier_parser::parse_modifiers;
use crate::parsers::declarations::type_declaration_helpers::{parse_open_brace, parse_close_brace};
use crate::parsers::expressions::expression_parser::parse_expression;
use crate::parsers::identifier_parser::parse_identifier;
use crate::parsers::types::type_parser::parse_type_expression;

/// Parse an enum declaration including attributes, modifiers, and members
/// Example in C#:
/// ```csharp
/// [Flags]
/// public enum DaysOfWeek : byte {
///     None = 0,
///     Monday = 1,
///     Tuesday = 2,
///     Wednesday = 4, 
///     Thursday = 8,
///     Friday = 16,
///     Saturday = 32,
///     Sunday = 64,
///     Weekend = Saturday | Sunday
/// }
/// ```
pub fn parse_enum_declaration(input: &str) -> BResult<&str, EnumDeclaration> {
    println!("parse_enum_declaration: input = \"{}\"" , input);
    // Parse attributes and convert to the expected format
    let (input, attribute_lists) = parse_attribute_lists(input)?;

    // Parse modifiers (public, internal, etc.)
    let (input, modifiers) = parse_modifiers(input)?;

    // Parse "enum" keyword
    let (input, _) = bws(keyword("enum"))(input)?;

    // Parse enum name
    let (input, name) = bws(parse_identifier)(input)?;
    
    // Parse optional underlying type (: byte, : int, etc.)
    let (input, underlying_type) = opt(tuple((
        bws(nom_to_bs(char::<&str, nom::error::Error<&str>>(':'))),
        bws(nom_to_bs(parse_type_expression))
    )))(input)?;
    
    // Extract the Type from the tuple, if present
    let underlying_type = underlying_type.map(|(_, ty)| ty);
    
    // Parse the enum body
    let (input, _) = parse_open_brace(input)?;
    
    // Parse enum members
    let (input, members) = parse_enum_members(input)?;
    
    // Parse the closing brace
    let (input, _) = parse_close_brace(input)?;
    
    Ok((input, EnumDeclaration {
        attributes: attribute_lists,
        modifiers,
        name,
        underlying_type,
        enum_members: members,
    }))
}

/// Parse a list of enum members
/// Example: "None = 0, Monday = 1, Tuesday = 2"
fn parse_enum_members<'a>(input: &'a str) -> BResult<&'a str, Vec<EnumMember>> {
    // Parse a comma-separated list of enum members
    // The list can be empty or have a trailing comma
    separated_list0(
        bws(nom_to_bs(char::<&str, nom::error::Error<&str>>(','))),
        bws(parse_enum_member)
    )(input)
}

/// Parse a single enum member
/// Example: "Monday = 1" or just "Monday"
fn parse_enum_member<'a>(input: &'a str) -> BResult<&'a str, EnumMember> {
    // Parse attributes for enum member
    let (input, attribute_lists) = parse_attribute_lists(input)?;
    
    // Parse the member name
    let (input, name) = bws(parse_identifier)(input)?;
    
    // Parse optional value assignment (e.g., "= 1" or "= Monday | Tuesday")
    let (input, value) = opt(
        preceded(
            bws(nom_to_bs(char::<&str, nom::error::Error<&str>>('='))),
            bws(parse_expression)
        )
    )(input)?;
    
    Ok((input, EnumMember {
        attributes: attribute_lists,
        name,
        value,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::nodes::types::{Type, PrimitiveType};
    use crate::parser::nodes::declarations::Modifier;
    use crate::parser::nodes::expressions::expression::Expression;
    use crate::parser::nodes::expressions::literal::Literal;
    
    // Local test helper to avoid import issues
    fn parse_full_input<'a, O, F>(input: &'a str, parser: F) -> Result<(&'a str, O), String>
    where
        F: FnOnce(&'a str) -> crate::parser::errors::BResult<&'a str, O>,
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
        let code = "#[Flags] public enum MyEnum : int { A, B }";
        let result = parse_full_input(code, parse_enum_declaration);
        assert!(result.is_ok(), "Parsing failed: {:?}", result.err());
        let (_remaining, decl) = result.unwrap();

        assert_eq!(decl.name.name, "MyEnum");
        
        // Check attributes
        assert_eq!(decl.attributes.len(), 1, "Expected 1 attribute list");
        assert!(!decl.attributes[0].attributes.is_empty(), "Expected attributes in the list");
        assert_eq!(decl.attributes[0].attributes[0].name.name, "Flags", "Attribute name mismatch");
        
        // Check modifiers
        assert_eq!(decl.modifiers.len(), 1, "Expected 1 modifier");
        assert_eq!(decl.modifiers[0], Modifier::Public, "Modifier mismatch");

        // Check underlying type
        assert!(decl.underlying_type.is_some(), "Expected an underlying type");
        if let Some(Type::Primitive(primitive)) = &decl.underlying_type {
            assert_eq!(*primitive, PrimitiveType::Int, "Underlying type mismatch");
        } else {
            panic!("Expected int primitive type, got {:?}", decl.underlying_type);
        }
        
        assert_eq!(decl.enum_members.len(), 2, "Member count mismatch");
    }
}
