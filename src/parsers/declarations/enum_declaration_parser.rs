use nom::character::complete::char;
use nom::combinator::opt;
use nom::multi::separated_list0;
use nom::sequence::{preceded, tuple};

use crate::parser::errors::BResult;
use crate::parser::nodes::declarations::EnumDeclaration;
use crate::parser::nodes::declarations::enum_declaration::EnumMember;
use crate::parser::parser_helpers::{bws, keyword, nom_to_bs};
use crate::parsers::declarations::attribute_parser::parse_attribute_lists;
use crate::parsers::declarations::modifier_parser::parse_modifiers_for_decl_type;
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
pub fn parse_enum_declaration<'a>(input: &'a str) -> BResult<&'a str, EnumDeclaration<'a>> {
    // Parse attributes (e.g., [Flags])
    let (input, attribute_lists) = parse_attribute_lists(input)?;
    
    // Convert AttributeList to Vec<Attribute> as expected by EnumDeclaration
    let attributes = attribute_lists.into_iter()
        .flat_map(|list| list.attributes)
        .collect();
    
    // Use the improved declaration header parser to handle whitespace and modifiers
    let mut header_parser = crate::parsers::declaration_helpers::parse_declaration_header(
        |i| parse_modifiers_for_decl_type(i, "enum"),
        "enum"
    );
    
    let (input, (modifiers, _)) = header_parser(input)?;
    
    // Parse enum name with proper whitespace handling
    let (input, name) = bws(nom_to_bs(parse_identifier))(input)?;
    
    // Parse optional underlying type (e.g., ': byte', ': int')
    let (input, underlying_type) = opt(
        tuple((
            bws(nom_to_bs(char::<&str, nom::error::Error<&str>>(':'))),
            bws(parse_type_expression)
        ))
    )(input)?;
    
    // Extract the Type from the tuple, if present
    let underlying_type = underlying_type.map(|(_, ty)| ty);
    
    // Parse the enum opening brace
    let (input, _) = bws(nom_to_bs(char::<&str, nom::error::Error<&str>>('{')))(input)?;
    
    // Parse enum members
    let (input, members) = parse_enum_members(input)?;
    
    // Parse the closing brace
    let (input, _) = bws(nom_to_bs(char::<&str, nom::error::Error<&str>>('}')))(input)?;
    
    Ok((input, EnumDeclaration {
        attributes,
        modifiers,
        name,
        underlying_type,
        members,
    }))
}

/// Parse a list of enum members
/// Example: "None = 0, Monday = 1, Tuesday = 2"
fn parse_enum_members<'a>(input: &'a str) -> BResult<&'a str, Vec<EnumMember<'a>>> {
    // Parse a comma-separated list of enum members
    // The list can be empty or have a trailing comma
    separated_list0(
        bws(nom_to_bs(char::<&str, nom::error::Error<&str>>(','))),
        bws(parse_enum_member)
    )(input)
}

/// Parse a single enum member
/// Example: "Monday = 1" or just "Monday"
fn parse_enum_member<'a>(input: &'a str) -> BResult<&'a str, EnumMember<'a>> {
    // Parse attributes for enum member
    let (input, attribute_lists) = parse_attribute_lists(input)?;
    
    // Convert AttributeList to Vec<Attribute>
    let attributes = attribute_lists.into_iter()
        .flat_map(|list| list.attributes)
        .collect();
    
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
        attributes,
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
        assert!(decl.members.is_empty());
    }
    
    #[test]
    fn test_enum_with_members() {
        let input = "enum Direction { North, East, South, West }";
        let result = parse_full_input(input, parse_enum_declaration);
        assert!(result.is_ok());
        let (_remaining, decl) = result.unwrap();
        assert_eq!(decl.name.name, "Direction");
        assert_eq!(decl.members.len(), 4);
        assert_eq!(decl.members[0].name.name, "North");
        assert_eq!(decl.members[1].name.name, "East");
        assert_eq!(decl.members[2].name.name, "South");
        assert_eq!(decl.members[3].name.name, "West");
    }
    
    #[test]
    fn test_enum_with_values() {
        let input = "enum ErrorCode { Success = 0, NotFound = 404, ServerError = 500 }";
        let result = parse_full_input(input, parse_enum_declaration);
        assert!(result.is_ok());
        let (_remaining, decl) = result.unwrap();
        assert_eq!(decl.name.name, "ErrorCode");
        assert_eq!(decl.members.len(), 3);
        
        // Check that values were parsed correctly
        assert_eq!(decl.members[0].name.name, "Success");
        if let Some(Expression::Literal(Literal::Integer(0))) = decl.members[0].value {
            // Success
        } else {
            panic!("Expected integer literal 0");
        }
        
        assert_eq!(decl.members[1].name.name, "NotFound");
        if let Some(Expression::Literal(Literal::Integer(404))) = decl.members[1].value {
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
        assert_eq!(decl.members.len(), 4);
    }
    
    #[test]
    fn test_enum_with_modifiers_and_attributes() {
        let input = "[Flags] public enum Colors { Red = 1, Green = 2, Blue = 4, Yellow = 8 }";
        let result = parse_full_input(input, parse_enum_declaration);
        assert!(result.is_ok());
        let (_remaining, decl) = result.unwrap();
        assert_eq!(decl.name.name, "Colors");
        
        // Check attributes
        assert_eq!(decl.attributes.len(), 1);
        assert_eq!(decl.attributes[0].name.name, "Flags");
        
        // Check modifiers
        assert_eq!(decl.modifiers.len(), 1);
        assert_eq!(decl.modifiers[0], Modifier::Public);
        
        // Check members
        assert_eq!(decl.members.len(), 4);
    }
}
