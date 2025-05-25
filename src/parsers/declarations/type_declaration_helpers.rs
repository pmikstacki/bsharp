// Common helpers for type declarations (struct, class, interface, record, enum)
// This module provides shared functionality for parsing C# type declarations

use nom::character::complete::{char, multispace0};

use crate::parser::errors::BResult;
use crate::parser::nodes::declarations::{
    attribute::AttributeList,
    modifier::Modifier,
};
use crate::parser::nodes::types::{Type, TypeParameter};
use crate::parser::nodes::identifier::Identifier;
use crate::parser::parser_helpers::{bws, nom_to_bs};
use crate::parsers::identifier_parser::parse_identifier;
use crate::parsers::declarations::type_parameter_parser::opt_parse_type_parameter_list;
use crate::parsers::declarations::modifier_parser::parse_modifiers_for_decl_type;
use crate::parsers::declarations::base_types_parser::parse_base_type_list;
use crate::parsers::declarations::attribute_parser::parse_attribute_lists;

/// Core structure for type declarations (class, struct, interface, record)
/// Contains the common elements shared by all these declaration types
pub struct BaseTypeDeclaration {
    pub attributes: Vec<AttributeList>,
    pub modifiers: Vec<Modifier>,
    pub name: Identifier,
    pub type_parameters: Option<Vec<TypeParameter>>,
    pub base_types: Vec<Type>,
}

/// Parse a type declaration header, handling attributes, modifiers, keyword, name and type parameters
/// Returns the parsed BaseTypeDeclaration and the remaining input
pub fn parse_type_declaration_header<'a>(
    input: &'a str,
    declaration_type: &'static str,
    keyword: &'static str,
) -> BResult<&'a str, BaseTypeDeclaration> {
    // Parse attributes first
    let (input, attribute_lists) = parse_attribute_lists(input)?;
    
    println!("parse_type_declaration_header: input = {:?}", input);

    // Try to parse using the declaration helper which handles the keyword and modifiers
    let mut header_parser = crate::parsers::declaration_helpers::parse_declaration_header(
        |i| parse_modifiers_for_decl_type(i, declaration_type),
        keyword
    );
    
    // Parse the header with improved whitespace handling
    let (remaining, (modifiers, _)) = match header_parser(input) {
        Ok(result) => result,
        Err(err) => {
            println!("Error parsing declaration header for {}: {:?}", declaration_type, err);
            return Err(err);
        }
    };
    
    // Parse the type name
    let (remaining, name) = match bws(nom_to_bs(parse_identifier))(remaining) {
        Ok(result) => result,
        Err(err) => {
            println!("Error parsing type name for {}: {:?}", declaration_type, err);
            return Err(err);
        }
    };
    
    // Parse type parameters directly - avoid nested Option
    let (remaining, type_parameters) = bws(nom_to_bs(opt_parse_type_parameter_list))(remaining)?;
    
    // Parse optional base types (interfaces or base classes)
    let (remaining, base_types) = parse_base_type_list(remaining)?;
    
    Ok((remaining, BaseTypeDeclaration {
        attributes: attribute_lists,
        modifiers,
        name,
        type_parameters,
        base_types,
    }))
}

/// Parse the opening brace of a type declaration body
pub fn parse_open_brace(input: &str) -> BResult<&str, ()> {
    let (input, _) = bws(nom_to_bs(char::<_, nom::error::Error<_>>('{')))(input)?;
    Ok((input, ()))
}

/// Parse the closing brace of a type declaration body
pub fn parse_close_brace(input: &str) -> BResult<&str, ()> {
    let (input, _) = bws(nom_to_bs(char::<_, nom::error::Error<_>>('}')))(input)?;
    Ok((input, ()))
}

/// Skip whitespace and check if we've reached the end of a body (closing brace)
pub fn at_end_of_body(input: &str) -> bool {
    let (after_ws, _) = multispace0::<&str, nom::error::Error<&str>>(input).unwrap_or((input, ""));
    after_ws.starts_with('}')
}
