// Improved whitespace handling for declaration parser
// This module provides common helpers for parsing C# declarations with robust whitespace handling

use crate::trivia::comment_parser::ws;
use crate::errors::BResult;
use nom::Parser;
use nom::character::complete::satisfy;
use nom::combinator::{map, not, peek};
use nom::sequence::{delimited, terminated};
use nom_supreme::tag::complete::tag;
use syntax::declarations::Modifier;
use syntax::span::Span;

/// Helper for consuming optional whitespace
pub fn optional_whitespace(input: Span<'_>) -> BResult<'_, &str> {
    // Consume whitespace and comments without returning the slice (consistent with ws signature)
    let (input, consumed) = ws(input)?;
    Ok((input, consumed))
}

/// Helper for robustly handling modifiers followed by a keyword
pub fn parse_declaration_header<'a, F>(
    mut modifiers_parser: F,
    kw: &'static str,
) -> impl FnMut(Span<'a>) -> BResult<'a, (Vec<Modifier>, &'a str)>
where
    F: FnMut(Span<'a>) -> BResult<'a, Vec<Modifier>>,
{
    // Dynamic keyword parser with word-boundary enforcement
    let mut kw_parser = move |i: Span<'a>| {
        map(
            terminated(
                tag(kw),
                peek(not(satisfy(|c: char| c.is_alphanumeric() || c == '_'))),
            ),
            |s: Span<'a>| *s.fragment(),
        )
        .parse(i)
    };

    move |input: Span<'a>| {
        // Parse modifiers (which might be empty)
        let (input, modifiers) = modifiers_parser(input)?;

        // Parse the keyword (struct, interface, etc.)
        let (input, keyword_result) = delimited(ws, &mut kw_parser, ws).parse(input)?;

        Ok((input, (modifiers, keyword_result)))
    }
}
