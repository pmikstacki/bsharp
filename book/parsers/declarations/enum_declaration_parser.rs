use crate::parser::errors::BResult;
use crate::parser::nodes::declarations::enum_declaration::{EnumDeclaration, EnumMember};
use crate::parser::parser_helpers::{bws, keyword, nom_to_bs};
use crate::parsers::declarations::attribute_parser::parse_attribute_lists;
use crate::parsers::declarations::modifier_parser::parse_modifiers;
use crate::parsers::declarations::type_declaration_helpers::{parse_close_brace, parse_open_brace};
use crate::parsers::expressions::expression_parser::parse_expression;
use crate::parsers::identifier_parser::parse_identifier;
use crate::parsers::types::type_parser::parse_type_expression;
use nom::{
    character::complete::char,
    combinator::opt,
    multi::separated_list0,
    sequence::{preceded, tuple},
};

/// Parse a C# enum declaration.
/// 
/// Supports enum declarations in several formats:
/// 
/// ```csharp
/// enum Days { Monday, Tuesday, Wednesday }
/// enum ErrorCode { Success = 0, NotFound = 404 }
/// enum FilePermissions : int { Read = 1, Write = 2, Execute = 4 }
/// [Flags]
/// enum Weekend : byte {
///     Saturday = 64,
///     Sunday = 128,
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
