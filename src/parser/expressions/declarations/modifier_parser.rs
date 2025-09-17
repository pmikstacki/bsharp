use crate::syntax::errors::BResult;
use crate::syntax::nodes::declarations::Modifier;
use crate::syntax::parser_helpers::{bws, context, keyword};
use nom::{branch::alt, combinator::value, multi::many0};

// Parse a single modifier keyword with word boundary check
fn parse_single_modifier(input: &str) -> BResult<&str, Modifier> {
    context(
        "modifier (expected access modifier, static, abstract, sealed, virtual, override, extern, unsafe, readonly, volatile, new, partial, ref, out, in, params, async, const, or fixed)",
        alt((
            // First group
            alt((
                value(Modifier::Public, keyword("public")),
                value(Modifier::Private, keyword("private")),
                value(Modifier::Protected, keyword("protected")),
                value(Modifier::Internal, keyword("internal")),
                value(Modifier::Static, keyword("static")),
                value(Modifier::Abstract, keyword("abstract")),
            )),
            // Second group
            alt((
                value(Modifier::Sealed, keyword("sealed")),
                value(Modifier::Virtual, keyword("virtual")),
                value(Modifier::Override, keyword("override")),
                value(Modifier::Extern, keyword("extern")),
                value(Modifier::Unsafe, keyword("unsafe")),
                value(Modifier::Readonly, keyword("readonly")),
                value(Modifier::Volatile, keyword("volatile")),
            )),
            // Third group
            alt((
                value(Modifier::New, keyword("new")),
                value(Modifier::Partial, keyword("partial")),
                value(Modifier::Ref, keyword("ref")),
                value(Modifier::Out, keyword("out")),
                value(Modifier::In, keyword("in")),
                value(Modifier::Params, keyword("params")),
            )),
            // Fourth group
            alt((
                value(Modifier::Async, keyword("async")),
                value(Modifier::Const, keyword("const")),
                value(Modifier::Fixed, keyword("fixed")),
            )),
        )),
    )(input)
}

/// Parse modifiers specifically for a given declaration type (e.g., "method", "class", etc.)
/// This allows for more specific error messages and validation
pub fn parse_modifiers_for_decl_type<'a>(
    input: &'a str,
    _declaration_type: &str,
) -> BResult<&'a str, Vec<Modifier>> {
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
