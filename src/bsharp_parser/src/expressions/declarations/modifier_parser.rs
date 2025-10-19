use crate::parser::keywords::access_keywords::{
    kw_file, kw_internal, kw_private, kw_protected, kw_public,
};
use crate::parser::keywords::modifier_keywords::{
    kw_abstract, kw_async, kw_const, kw_extern, kw_new, kw_override, kw_partial, kw_readonly,
    kw_required, kw_sealed, kw_static, kw_unsafe, kw_virtual, kw_volatile,
};
use crate::parser::keywords::parameter_modifier_keywords::{kw_in, kw_out, kw_params, kw_ref};
use crate::syntax::errors::BResult;

use crate::syntax::comment_parser::ws;
use crate::syntax::span::Span;
use nom::Parser;
use nom::{branch::alt, combinator::value, multi::many0, sequence::delimited};
use nom_supreme::ParserExt;
use syntax::declarations::Modifier;

// Parse a single modifier keyword with word boundary check
fn parse_single_modifier(input: Span) -> BResult<Modifier> {
    alt((
        // First group
        alt((
            value(Modifier::Public, kw_public()),
            value(Modifier::Private, kw_private()),
            value(Modifier::Protected, kw_protected()),
            value(Modifier::Internal, kw_internal()),
            value(Modifier::File, kw_file()),
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
            value(Modifier::Const, kw_const()),
            // "fixed" can be used as a modifier in some contexts; reuse kw_fixed from exception_and_safety_keywords
            value(Modifier::Fixed, crate::parser::keywords::exception_and_safety_keywords::kw_fixed()),
        )),
    ))
        .context("modifier (expected access modifier, static, abstract, sealed, virtual, override, extern, unsafe, readonly, volatile, new, partial, ref, out, in, params, async, const, or fixed)")
        .parse(input.into())
}

/// Parse modifiers specifically for a given declaration type (e.g., "method", "class", etc.)
/// This allows for more specific error messages and validation
pub fn parse_modifiers_for_decl_type<'a>(
    input: Span<'a>,
    _declaration_type: &str,
) -> BResult<'a, Vec<Modifier>> {
    parse_modifiers
        .context("declaration modifiers (expected zero or more valid C# modifiers)")
        .parse(input.into())
}

// Parse zero or more modifiers (for backward compatibility or general use)
pub fn parse_modifiers(input: Span) -> BResult<Vec<Modifier>> {
    let (input, modifiers) = many0(delimited(ws, parse_single_modifier, ws)).parse(input.into())?;
    // Preserve input order to match expectations in tests and downstream consumers
    Ok((input, modifiers))
}
