// Improved whitespace handling for declaration parser
// This module provides common helpers for parsing C# declarations with robust whitespace handling

use crate::syntax::comment_parser::ws;
use crate::syntax::errors::BResult;
use crate::syntax::nodes::declarations::Modifier;
use crate::syntax::parser_helpers::{bws, keyword};

/// Helper for consuming optional whitespace
pub fn optional_whitespace(input: &str) -> BResult<&str, &str> {
    // Consume whitespace and comments without returning the slice (consistent with ws signature)
    let (input, consumed) = ws(input)?;
    Ok((input, consumed))
}

/// Helper for robustly handling modifiers followed by a keyword
pub fn parse_declaration_header<'a, F>(
    mut modifiers_parser: F,
    kw: &'static str,
) -> impl FnMut(&'a str) -> BResult<&'a str, (Vec<Modifier>, &'a str)>
where
    F: FnMut(&'a str) -> BResult<&'a str, Vec<Modifier>>,
{
    move |input: &'a str| {
        // Parse modifiers (which might be empty)
        let (input, modifiers) = modifiers_parser(input)?;

        // Parse the keyword (struct, interface, etc.)
        let (input, keyword_result) = bws(keyword(kw))(input)?;

        Ok((input, (modifiers, keyword_result)))
    }
}
