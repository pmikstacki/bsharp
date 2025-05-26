use crate::parser::errors::{BResult, BSharpParseError};
use crate::parser::nodes::declarations::Modifier;
use crate::parser::parser_helpers::{bws, nom_to_bs};
use nom::error::ParseError;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::alpha1,
    combinator::{not, peek, value},
    multi::many0,
    sequence::terminated,
};

// Helper to ensure we match complete words, not prefixes
fn word_boundary(input: &str) -> nom::IResult<&str, (), nom::error::Error<&str>> {
    // Check that the next character is not alphanumeric or underscore, without consuming it
    peek(not(alpha1))(input)
}

// Parse a single modifier keyword with word boundary check
fn parse_single_modifier(input: &str) -> BResult<&str, Modifier> {
    nom_to_bs(alt((
        // First group
        alt((
            value(Modifier::Public, terminated(tag("public"), word_boundary)),
            value(Modifier::Private, terminated(tag("private"), word_boundary)),
            value(Modifier::Protected, terminated(tag("protected"), word_boundary)),
            value(Modifier::Internal, terminated(tag("internal"), word_boundary)),
            value(Modifier::Static, terminated(tag("static"), word_boundary)),
            value(Modifier::Abstract, terminated(tag("abstract"), word_boundary)),
        )),
        // Second group
        alt((
            value(Modifier::Sealed, terminated(tag("sealed"), word_boundary)),
            value(Modifier::Virtual, terminated(tag("virtual"), word_boundary)),
            value(Modifier::Override, terminated(tag("override"), word_boundary)),
            value(Modifier::Extern, terminated(tag("extern"), word_boundary)),
            value(Modifier::Unsafe, terminated(tag("unsafe"), word_boundary)),
            value(Modifier::Readonly, terminated(tag("readonly"), word_boundary)),
            value(Modifier::Volatile, terminated(tag("volatile"), word_boundary)),
        )),
        // Third group
        alt((
            value(Modifier::New, terminated(tag("new"), word_boundary)),
            value(Modifier::Partial, terminated(tag("partial"), word_boundary)),
            value(Modifier::Ref, terminated(tag("ref"), word_boundary)),
            value(Modifier::Out, terminated(tag("out"), word_boundary)),
            value(Modifier::In, terminated(tag("in"), word_boundary)),
            value(Modifier::Params, terminated(tag("params"), word_boundary)),
        )),
        // Fourth group
        alt((
            value(Modifier::Async, terminated(tag("async"), word_boundary)),
            value(Modifier::Const, terminated(tag("const"), word_boundary)),
            value(Modifier::Fixed, terminated(tag("fixed"), word_boundary)),
        )),
    )))(input)
}

/// Parse and validate modifiers for a specific declaration type
pub fn parse_modifiers_for_decl_type<'a>(input: &'a str, decl_type: &str) -> BResult<&'a str, Vec<Modifier>> {
    let mut modifiers = Vec::new();
    let mut current_input = input;
    
    // Get compatible modifiers for this declaration type
    let compatible_modifiers = Modifier::get_compatible_modifiers_for(decl_type);
    
    // Parse modifiers one by one and validate compatibility immediately
    loop {
        // Try to parse a modifier with whitespace
        match bws(parse_single_modifier)(current_input) {
            Ok((remaining_input, modifier)) => {
                // Check if this modifier is compatible with the declaration type
                if !compatible_modifiers.contains(&modifier) {
                    // Incompatible modifier found - stop parsing and don't consume it
                    break;
                }
                
                // Check for incompatible modifier combinations with already parsed modifiers
                for existing_modifier in &modifiers {
                    if modifier.is_incompatible_with(existing_modifier) {
                        return Err(nom::Err::Error(BSharpParseError::from_error_kind(current_input, nom::error::ErrorKind::Tag)));
                    }
                }
                
                modifiers.push(modifier);
                current_input = remaining_input;
            }
            Err(_) => {
                // No more modifiers to parse
                break;
            }
        }
    }
    
    // Order modifiers according to C# conventions
    Modifier::order_modifiers(&mut modifiers);
    
    Ok((current_input, modifiers))
}

// Parse zero or more modifiers (for backward compatibility or general use)
pub fn parse_modifiers(input: &str) -> BResult<&str, Vec<Modifier>> {
    // This version uses many0(bws(parse_single_modifier)) for consistency.
    let (input, mut modifiers) = many0(bws(parse_single_modifier))(input)?;
    
    Modifier::order_modifiers(&mut modifiers);
    
    Ok((input, modifiers))
}
