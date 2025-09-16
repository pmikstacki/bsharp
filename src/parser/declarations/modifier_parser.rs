use crate::syntax::errors::BResult;
use crate::syntax::nodes::declarations::Modifier;
use crate::syntax::parser_helpers::{context, bws};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::alpha1,
    combinator::{not, peek, value},
    multi::many0,
    sequence::terminated,
};

// Helper to ensure we match complete words, not prefixes
fn word_boundary(input: &str) -> BResult<&str, ()> {
    // Check that the next character is not alphanumeric or underscore, without consuming it
    peek(not(alpha1))(input)
}

// Parse a single modifier keyword with word boundary check
fn parse_single_modifier(input: &str) -> BResult<&str, Modifier> {
    context("modifier (expected access modifier, static, abstract, sealed, virtual, override, extern, unsafe, readonly, volatile, new, partial, ref, out, in, params, async, const, or fixed)", alt((
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

/// Parse modifiers specifically for a given declaration type (e.g., "method", "class", etc.)
/// This allows for more specific error messages and validation
pub fn parse_modifiers_for_decl_type<'a>(input: &'a str, _declaration_type: &str) -> BResult<&'a str, Vec<Modifier>> {
    context(
        "declaration modifiers (expected zero or more valid C# modifiers)",
        parse_modifiers,
    )(input)
}

// Parse zero or more modifiers (for backward compatibility or general use)
pub fn parse_modifiers(input: &str) -> BResult<&str, Vec<Modifier>> {
    // This version uses many0(bws(parse_single_modifier)) for consistency.
    let (input, mut modifiers) = many0(bws(parse_single_modifier))(input)?;
    
    Modifier::order_modifiers(&mut modifiers);
    
    Ok((input, modifiers))
}
