// Common helpers for type declarations (struct, class, interface, record, enum)
// This module provides shared functionality for parsing C# type declarations

use nom::{
    branch::alt,
    character::complete::{char, multispace0},
    bytes::complete::tag,
    combinator::{map, opt},
    sequence::{delimited, tuple},
};

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
pub struct BaseTypeDeclaration<'a> {
    pub attributes: Vec<AttributeList<'a>>,
    pub modifiers: Vec<Modifier>,
    pub name: Identifier,
    pub type_parameters: Option<Vec<TypeParameter>>,
    pub base_types: Vec<Type<'a>>,
}

/// Parse a type declaration header, handling attributes, modifiers, keyword, name and type parameters
/// Returns the parsed BaseTypeDeclaration and the remaining input
pub fn parse_type_declaration_header<'a>(
    input: &'a str,
    declaration_type: &'static str,
    keyword: &'static str,
) -> BResult<&'a str, BaseTypeDeclaration<'a>> {
    // Parse attributes first
    let (input, attribute_lists) = parse_attribute_lists(input)?;
    
    // Try to parse using the declaration helper which handles the keyword and modifiers
    let mut header_parser = crate::parsers::declaration_helpers::parse_declaration_header(
        |i| parse_modifiers_for_decl_type(i, declaration_type),
        keyword
    );
    
    // Parse the header with improved whitespace handling
    let (input, (modifiers, _)) = header_parser(input)?;
    
    // Parse the type name
    let (input, name) = bws(nom_to_bs(parse_identifier))(input)?;
    
    // Parse optional type parameters (generics like <T, U>)
    let (input, type_parameters) = opt(bws(nom_to_bs(opt_parse_type_parameter_list)))(input)?;
    
    // Flatten Option<Option<Vec<...>>> to Option<Vec<...>>
    let type_parameters = type_parameters.and_then(|x| x);
    
    // Parse optional base types (interfaces or base classes)
    let (input, base_types) = parse_base_type_list(input)?;
    
    Ok((input, BaseTypeDeclaration {
        attributes: attribute_lists,
        modifiers,
        name,
        type_parameters,
        base_types,
    }))
}

/// Parse the opening brace of a type declaration body
pub fn parse_open_brace<'a>(input: &'a str) -> BResult<&'a str, ()> {
    let (input, _) = bws(nom_to_bs(char::<_, nom::error::Error<_>>('{')))(input)?;
    Ok((input, ()))
}

/// Parse the closing brace of a type declaration body
pub fn parse_close_brace<'a>(input: &'a str) -> BResult<&'a str, ()> {
    let (input, _) = bws(nom_to_bs(char::<_, nom::error::Error<_>>('}')))(input)?;
    Ok((input, ()))
}

/// Skip whitespace and check if we've reached the end of a body (closing brace)
pub fn at_end_of_body<'a>(input: &'a str) -> bool {
    let (after_ws, _) = multispace0::<&str, nom::error::Error<&str>>(input).unwrap_or((input, ""));
    after_ws.starts_with('}')
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_base_type_declaration() {
        let input = "public class MyClass<T> : IComparable<T> {";
        let (input, result) = parse_type_declaration_header(input, "class", "class").unwrap();
        
        assert_eq!(result.modifiers, vec![Modifier::Public]);
        assert_eq!(result.name.name, "MyClass");
        assert!(result.type_parameters.is_some());
        assert_eq!(result.base_types.len(), 1);
        
        // Check that we're left with the opening brace
        assert_eq!(input.trim(), "{");
    }
    
    #[test]
    fn test_at_end_of_body() {
        assert!(at_end_of_body(" }"));
        assert!(at_end_of_body("\n\t}"));
        assert!(!at_end_of_body(" int x;"));
    }
}
