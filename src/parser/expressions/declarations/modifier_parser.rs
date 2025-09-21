use crate::syntax::errors::BResult;
use crate::syntax::nodes::declarations::Modifier;
use crate::syntax::parser_helpers::{bws, context, keyword};
use crate::parser::keywords::access_keywords::{kw_public, kw_private, kw_protected, kw_internal};
use crate::parser::keywords::modifier_keywords::{
    kw_static, kw_abstract, kw_sealed, kw_virtual, kw_override, kw_extern, kw_unsafe, kw_readonly,
    kw_volatile, kw_partial, kw_new, kw_async, kw_required,
};
use crate::parser::keywords::parameter_modifier_keywords::{kw_ref, kw_out, kw_in, kw_params};
use nom::{branch::alt, combinator::value, multi::many0};

// Parse a single modifier keyword with word boundary check
fn parse_single_modifier(input: &str) -> BResult<&str, Modifier> {
    context(
        "modifier (expected access modifier, static, abstract, sealed, virtual, override, extern, unsafe, readonly, volatile, new, partial, ref, out, in, params, async, const, or fixed)",
        alt((
            // First group
            alt((
                value(Modifier::Public, kw_public()),
                value(Modifier::Private, kw_private()),
                value(Modifier::Protected, kw_protected()),
                value(Modifier::Internal, kw_internal()),
                value(Modifier::Static, kw_static()),
                value(Modifier::Abstract, kw_abstract()),
            )),
            // Second group
            alt((
                value(Modifier::Sealed, kw_sealed()),
                value(Modifier::Virtual, kw_virtual()),
                value(Modifier::Override, kw_override()),
                value(Modifier::Extern, kw_extern()),
                value(Modifier::Unsafe, kw_unsafe()),
                value(Modifier::Readonly, kw_readonly()),
                value(Modifier::Volatile, kw_volatile()),
            )),
            // Third group
            alt((
                value(Modifier::New, kw_new()),
                value(Modifier::Partial, kw_partial()),
                value(Modifier::Ref, kw_ref()),
                value(Modifier::Out, kw_out()),
                value(Modifier::In, kw_in()),
                value(Modifier::Params, kw_params()),
                value(Modifier::Required, kw_required()),
            )),
            // Fourth group
            alt((
                value(Modifier::Async, kw_async()),
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
