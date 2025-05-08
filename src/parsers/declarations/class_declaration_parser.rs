use nom::{
    character::complete::{char as nom_char, multispace0},
    bytes::complete::tag,
    combinator::map,
};
use crate::parser::errors::BResult;
use crate::parser::nodes::declarations::{ClassDeclaration, ClassMember};
use crate::parser::parser_helpers::{bws, nom_to_bs};
use crate::parsers::identifier_parser::parse_identifier;
use crate::parsers::declarations::field_declaration_parser::parse_field_declaration;
use crate::parsers::declarations::method_declaration_parser::parse_method_declaration;
use crate::parsers::declarations::type_parameter_parser::opt_parse_type_parameter_list;
use crate::parsers::declarations::modifier_parser::parse_modifiers_for_decl_type;

// Using bws from parser_helpers instead of local ws function

// Parse a class member (field or method)
fn parse_class_member(input: &str) -> BResult<&str, ClassMember> {
    // Try parsing a field first, then a method
    // TODO: Add other members like constructors, properties, nested types, etc.
    nom::branch::alt((
        map(parse_field_declaration, ClassMember::Field),
        map(nom_to_bs(parse_method_declaration), ClassMember::Method),
    ))(input)
}

// Parse a class declaration
pub fn parse_class_declaration(input: &str) -> BResult<&str, ClassDeclaration> {
    // Parse modifiers with validation for class declarations
    let (input, modifiers) = bws(nom_to_bs(|i| parse_modifiers_for_decl_type(i, "class")))(input)?;

    // Parse "class" keyword
    let (input, _) = bws(nom_to_bs(tag::<_, _, nom::error::Error<_>>("class")))(input)?;

    // Parse class name
    let (input, name) = bws(nom_to_bs(parse_identifier))(input)?;
    
    // Parse optional type parameters (generics)
    let (input, type_parameters) = bws(nom_to_bs(opt_parse_type_parameter_list))(input)?;

    // Parse class body enclosed in {}
    let (input, _) = bws(nom_to_bs(nom_char::<&str, nom::error::Error<&str>>('{')))(input)?;
    
    // Parse zero or more class members with whitespace between
    let mut members = Vec::new();
    let mut current_input = input;
    
    loop {
        // Skip whitespace
        let after_ws = match multispace0::<_, nom::error::Error<_>>(current_input) {
            Ok((input_after_ws, _)) => input_after_ws,
            Err(_) => current_input, // If there's an error parsing whitespace, just use current input
        };
        
        // Try to parse a class member
        match parse_class_member(after_ws) {
            Ok((new_input, member)) => {
                members.push(member);
                current_input = new_input;
            },
            Err(_) => {
                // No more members or syntax error - break the loop
                break;
            }
        }
    }
    
    // Parse closing brace
    let (input, _) = bws(nom_to_bs(nom_char::<&str, nom::error::Error<&str>>('}')))(current_input)?;

    Ok((input, ClassDeclaration {
        modifiers,
        name,
        type_parameters,
        members,
    }))
}
