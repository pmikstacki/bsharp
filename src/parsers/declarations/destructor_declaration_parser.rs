use nom::{
    branch::alt,
    character::complete::char as nom_char,
    combinator::map,
};

use crate::parser::errors::BResult;
use crate::parser::nodes::declarations::DestructorDeclaration;
use crate::parser::parser_helpers::{bws, nom_to_bs};
use crate::parsers::declarations::attribute_parser::parse_attribute_lists;
use crate::parsers::declarations::modifier_parser::parse_modifiers;
use crate::parsers::declarations::type_declaration_parser::convert_attributes;
use crate::parsers::identifier_parser::parse_identifier;
use crate::parsers::statements::block_statement_parser::parse_block_statement;

/// Parse a C# destructor declaration
/// 
/// Examples:
/// ```csharp
/// ~MyClass() { ... }
/// ~MyClass() { /* cleanup code */ }
/// ```
pub fn parse_destructor_declaration(input: &str) -> BResult<&str, DestructorDeclaration> {
    // Parse attributes
    let (input, attribute_lists) = parse_attribute_lists(input)?;
    let attributes = convert_attributes(attribute_lists);
    
    // Parse modifiers (destructors typically don't have explicit modifiers)
    let (input, modifiers) = parse_modifiers(input)?;
    
    // Parse the tilde (~) symbol
    let (input, _) = bws(nom_to_bs(nom_char::<&str, nom::error::Error<&str>>('~')))(input)?;
    
    // Parse the class name (destructor name must match class name)
    let (input, name) = bws(nom_to_bs(parse_identifier))(input)?;
    
    // Parse the parameter list (must be empty for destructors)
    let (input, _) = bws(nom_to_bs(nom_char::<&str, nom::error::Error<&str>>('(')))(input)?;
    let (input, _) = bws(nom_to_bs(nom_char::<&str, nom::error::Error<&str>>(')')))(input)?;
    
    // Parse the body (either block statement or semicolon for extern)
    let (input, body) = parse_destructor_body(input)?;
    
    let destructor_declaration = DestructorDeclaration {
        attributes,
        modifiers,
        name,
        body,
    };
    
    Ok((input, destructor_declaration))
}

/// Parse the destructor body (either a block statement or semicolon)
fn parse_destructor_body(input: &str) -> BResult<&str, String> {
    alt((
        // Block body
        map(
            nom_to_bs(parse_block_statement),
            |_| "{ /* destructor body */ }".to_string() // Simplified for now
        ),
        // Semicolon (extern)
        map(
            bws(nom_to_bs(nom_char::<&str, nom::error::Error<&str>>(';'))),
            |_| "".to_string()
        ),
    ))(input)
} 