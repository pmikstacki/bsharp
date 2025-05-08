use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{multispace1},
    combinator::{value},
    multi::many0,
    sequence::{terminated},
    IResult,
    error::{Error, ErrorKind},
};
use crate::parser::nodes::declarations::Modifier;

// Parse a single modifier keyword
fn parse_single_modifier(input: &str) -> IResult<&str, Modifier> {
    alt((
        // First group
        alt((
            value(Modifier::Public, tag("public")),
            value(Modifier::Private, tag("private")),
            value(Modifier::Protected, tag("protected")),
            value(Modifier::Internal, tag("internal")),
            value(Modifier::Static, tag("static")),
            value(Modifier::Abstract, tag("abstract")),
        )),
        // Second group
        alt((
            value(Modifier::Sealed, tag("sealed")),
            value(Modifier::Virtual, tag("virtual")),
            value(Modifier::Override, tag("override")),
            value(Modifier::Extern, tag("extern")),
            value(Modifier::Unsafe, tag("unsafe")),
            value(Modifier::Readonly, tag("readonly")),
            value(Modifier::Volatile, tag("volatile")),
        )),
        // Third group
        alt((
            value(Modifier::New, tag("new")),
            value(Modifier::Partial, tag("partial")),
            value(Modifier::Ref, tag("ref")),
            value(Modifier::Out, tag("out")),
            value(Modifier::In, tag("in")),
            value(Modifier::Params, tag("params")),
        )),
        // Fourth group
        alt((
            value(Modifier::Async, tag("async")),
            value(Modifier::Const, tag("const")),
            value(Modifier::Fixed, tag("fixed")),
        )),
    ))(input)
}

/// Parse and validate modifiers for a specific declaration type
pub fn parse_modifiers_for_decl_type<'a>(input: &'a str, decl_type: &str) -> IResult<&'a str, Vec<Modifier>> {
    // Consume modifier + mandatory space
    let (input, mut modifiers) = many0(terminated(parse_single_modifier, multispace1))(input)?;
    
    // Get compatible modifiers for this declaration type
    let compatible_modifiers = Modifier::get_compatible_modifiers_for(decl_type);
    
    // Check if all parsed modifiers are compatible with the declaration type
    for modifier in &modifiers {
        if !compatible_modifiers.contains(modifier) {
            return Err(nom::Err::Error(Error::new(input, ErrorKind::Tag)));
        }
    }
    
    // Check for incompatible modifier combinations
    for (i, mod1) in modifiers.iter().enumerate() {
        for (j, mod2) in modifiers.iter().enumerate() {
            if i != j && mod1.is_incompatible_with(mod2) {
                return Err(nom::Err::Error(Error::new(input, ErrorKind::Tag)));
            }
        }
    }
    
    // Order modifiers according to C# conventions
    Modifier::order_modifiers(&mut modifiers);
    
    Ok((input, modifiers))
}

// Parse zero or more modifiers (for backward compatibility)
pub fn parse_modifiers(input: &str) -> IResult<&str, Vec<Modifier>> {
    // Consume modifier + mandatory space
    let (input, mut modifiers) = many0(terminated(parse_single_modifier, multispace1))(input)?;
    
    // Just order them but don't validate for specific declaration types
    Modifier::order_modifiers(&mut modifiers);
    
    Ok((input, modifiers))
}
