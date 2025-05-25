use nom::{
    bytes::complete::tag as nom_tag,
    character::complete::{char as nom_char, multispace0},
    combinator::map,
    branch::alt,
};

use crate::parser::errors::BResult;
use crate::parser::nodes::declarations::{IndexerDeclaration, IndexerAccessorList};
use crate::parser::nodes::types::Parameter;
use crate::parser::parser_helpers::{bws, nom_to_bs};
use crate::parsers::types::type_parser::parse_type_expression;
use crate::parsers::declarations::attribute_parser::parse_attribute_lists;
use crate::parsers::declarations::type_declaration_parser::convert_attributes;
use crate::parsers::declarations::modifier_parser::parse_modifiers;
use crate::parsers::statements::block_statement_parser::parse_block_statement;

/// Parse a C# indexer declaration
/// 
/// Examples:
/// ```csharp
/// public int this[int index] { get; set; }
/// public string this[int row, int col] { get { return _data[row][col]; } set { _data[row][col] = value; } }
/// ```
pub fn parse_indexer_declaration(input: &str) -> BResult<&str, IndexerDeclaration> {
    // Parse attributes
    let (input, attribute_lists) = parse_attribute_lists(input)?;
    let attributes = convert_attributes(attribute_lists);
    
    // Parse modifiers (public, private, etc.)
    let (input, modifiers) = parse_modifiers(input)?;
    
    // Parse the return type
    let (input, ty) = bws(nom_to_bs(parse_type_expression))(input)?;
    
    // Parse the "this" keyword
    let (input, _) = bws(nom_to_bs(nom_tag::<&str, &str, nom::error::Error<&str>>("this")))(input)?;
    
    // Parse the indexer parameters [type name, ...]
    let (input, _) = bws(nom_to_bs(nom_char::<&str, nom::error::Error<&str>>('[')))(input)?;
    let (input, parameters) = parse_indexer_parameters(input)?;
    let (input, _) = bws(nom_to_bs(nom_char::<&str, nom::error::Error<&str>>(']')))(input)?;
    
    // Parse the accessor list { get; set; } or { get { ... } set { ... } }
    let (input, accessor_list) = parse_indexer_accessor_list(input)?;
    
    let indexer_declaration = IndexerDeclaration {
        attributes,
        modifiers,
        indexer_type: ty,
        parameters,
        accessor_list,
    };
    
    Ok((input, indexer_declaration))
}

/// Parse indexer parameters (same as regular parameters but inside square brackets)
fn parse_indexer_parameters(input: &str) -> BResult<&str, Vec<Parameter>> {
    use nom::multi::separated_list0;
    use crate::parsers::declarations::parameter_parser::parse_parameter;
    
    let (input, parameters) = bws(nom_to_bs(separated_list0(
        bws(nom_to_bs(nom_char::<&str, nom::error::Error<&str>>(','))),
        bws(nom_to_bs(parse_parameter)),
    )))(input)?;
    
    Ok((input, parameters))
}

/// Parse the indexer accessor list
fn parse_indexer_accessor_list(input: &str) -> BResult<&str, IndexerAccessorList> {
    // Parse opening brace
    let (input, _) = bws(nom_to_bs(nom_char::<&str, nom::error::Error<&str>>('{')))(input)?;
    
    // Parse accessors (get and/or set)
    let (input, (get_accessor, set_accessor)) = parse_accessors(input)?;
    
    // Parse closing brace
    let (input, _) = bws(nom_to_bs(nom_char::<&str, nom::error::Error<&str>>('}')))(input)?;
    
    Ok((input, IndexerAccessorList {
        get_accessor,
        set_accessor,
    }))
}

/// Parse get and/or set accessors
fn parse_accessors(input: &str) -> BResult<&str, (Option<String>, Option<String>)> {
    let mut get_accessor = None;
    let mut set_accessor = None;
    let mut current = input;
    
    // Keep parsing accessors until we hit the closing brace
    while !current.trim_start().starts_with('}') {
        // Skip whitespace
        let (after_ws, _) = bws(nom_to_bs(multispace0::<&str, nom::error::Error<&str>>))(current)?;
        current = after_ws;
        
        // Check if we're at the end
        if current.trim_start().starts_with('}') {
            break;
        }
        
        // Try to parse an accessor
        if current.trim_start().starts_with("get") {
            let (rest, accessor_body) = parse_accessor_declaration(current, "get")?;
            get_accessor = Some(accessor_body);
            current = rest;
        } else if current.trim_start().starts_with("set") {
            let (rest, accessor_body) = parse_accessor_declaration(current, "set")?;
            set_accessor = Some(accessor_body);
            current = rest;
        } else {
            // Unknown accessor, skip it
            break;
        }
    }
    
    Ok((current, (get_accessor, set_accessor)))
}

/// Parse a single accessor declaration (get or set)
fn parse_accessor_declaration<'a>(input: &'a str, accessor_type: &str) -> BResult<&'a str, String> {
    // Parse the accessor keyword
    let (input, _) = bws(nom_to_bs(nom_tag::<&str, &str, nom::error::Error<&str>>(accessor_type)))(input)?;
    
    // Parse the body (either block or semicolon)
    alt((
        // Semicolon (auto-accessor)
        map(
            bws(nom_to_bs(nom_char::<&str, nom::error::Error<&str>>(';'))),
            |_| "".to_string()
        ),
        // Block body
        map(
            nom_to_bs(parse_block_statement),
            |_| format!("{{ /* {} body */ }}", accessor_type)
        ),
    ))(input)
} 