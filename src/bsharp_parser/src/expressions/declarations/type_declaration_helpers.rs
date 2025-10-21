// Common helpers for type declarations (struct, class, interface, record, enum)
// This module provides shared functionality for parsing C# type declarations

use crate::parser::expressions::declarations::attribute_parser::parse_attribute_lists;
use crate::parser::expressions::declarations::base_types_parser::parse_base_types;
use crate::parser::expressions::declarations::modifier_parser::parse_modifiers_for_decl_type;
use crate::parser::expressions::declarations::type_parameter_parser::opt_parse_type_parameter_list;
use crate::parser::identifier_parser::parse_identifier;
use crate::syntax::errors::BResult;
#[cfg(feature = "parser_recovery_trace")]
use log::trace;

use crate::syntax::comment_parser::ws;
use nom::bytes::complete::tag;
use nom::character::complete::satisfy;
use nom::combinator::{cut, peek};
use nom::sequence::delimited;
use nom::Parser;
use nom_supreme::ParserExt;
use syntax::declarations::{AttributeList, Modifier};
use syntax::types::{Type, TypeParameter};
use syntax::Identifier;

/// Core structure for type declarations (class, struct, interface, record)
/// Contains the common elements shared by all these declaration types
pub struct BaseTypeDeclaration {
    pub attributes: Vec<AttributeList>,
    pub modifiers: Vec<Modifier>,
    pub name: Identifier,
    pub type_parameters: Option<Vec<TypeParameter>>,
    pub base_types: Vec<Type>,
}

/// Span-native: Parse a type declaration header, handling attributes, modifiers, keyword, name and type parameters
pub fn parse_type_declaration_header_span<'a>(
    input: Span<'a>,
    declaration_type: &'static str,
    keyword: &'static str,
) -> BResult<'a, BaseTypeDeclaration> {
    let (input, attributes) = parse_attribute_lists(input)?;
    let (input, modifiers) = parse_modifiers_for_decl_type(input, declaration_type)?;
    let (input, _) = delimited(ws, tag(keyword), ws)
        .context("declaration keyword")
        .parse(input)?;
    let (input, name) = delimited(ws, parse_identifier, ws)
        .context("type name")
        .parse(input)?;
    let (input, type_parameters) = opt_parse_type_parameter_list.parse(input)?;
    let (input, base_types) = parse_base_types.parse(input)?;

    Ok((
        input,
        BaseTypeDeclaration {
            attributes,
            modifiers,
            name,
            type_parameters,
            base_types,
        },
    ))
}

// Removed legacy &str-based wrapper. Use `parse_type_declaration_header_span`.

/// Parse the opening brace of a type declaration body
pub fn parse_open_brace(input: Span) -> BResult<()> {
    let (input, _) = delimited(ws, satisfy(|c| c == '{'), ws)
        .context("opening brace")
        .parse(input)?;
    Ok((input, ()))
}

/// Parse the closing brace of a type declaration body
pub fn parse_close_brace(input: Span) -> BResult<()> {
    let (input, _) = cut(delimited(ws, satisfy(|c| c == '}'), ws))
        .context("closing brace")
        .parse(input)?;
    Ok((input, ()))
}

/// Skip whitespace and check if we've reached the end of a body (closing brace)
pub fn at_end_of_body(input: Span) -> bool {
    // Non-consuming lookahead for '}' after whitespace/comments
    peek(delimited(ws, satisfy(|c| c == '}'), ws)).parse(input).is_ok()
}

/// Skip malformed input within a type body until a safe, top-level recovery boundary.
///
/// Contract (Type Member Top-Level Only):
/// - Pre-conditions:
///   - `input` points somewhere inside a type body, typically at the start of a malformed member.
///   - The first non-whitespace character SHOULD NOT be the closing brace `}` of the enclosing type.
///     If it is, recovery should be handled by the caller by consuming `}` (or returning) rather than
///     invoking this function. We add a debug assertion to surface misuse.
/// - Behavior:
///   - Scans forward while tracking depth of `()`, `[]`, `{}`, and a heuristic generic angle depth `<>`.
///   - Ignores (does not treat as boundaries) any control characters that appear inside strings, chars, or comments.
///   - Stops at the first top-level `;` (when all depths are zero) and returns the slice AFTER that semicolon.
///   - Or stops at the first top-level `}` (when all depths are zero) and returns the slice STARTING at that brace
///     (i.e., the brace is NOT consumed). This allows the outer body parser to close cleanly.
///   - If no boundary is found before EOF, returns an empty string slice `""`.
/// - Post-conditions:
///   - Return always points at a stable recovery boundary suitable for resuming member parsing or closing the body.
///
/// Limitations and non-goals:
/// - Angle-bracket tracking is a heuristic for generic type arguments and will not fully disambiguate all cases
///   (e.g., shift operators vs generics). This is acceptable for recovery in the declarations context.
/// - Verbatim strings (@"...") and interpolated strings ($"..." or $@"..."), as well as escaped newlines within
///   strings, are not handled with full lexical fidelity. They are out of scope for this lightweight recovery pass.
///
/// This helper is designed to be called from type declaration bodies (classes, structs, records, interfaces)
/// at the point where a member failed to parse, to avoid cascading errors and continue with subsequent members.
pub fn skip_to_member_boundary_top_level(input: Span) -> &str {
    use std::cmp::min;

    // Guardrails: discourage calling when already at a closing brace for the current body.
    // This is not a hard error in release builds, but it helps surface misuse during development.
    debug_assert!(
        peek(delimited(ws, satisfy(|c| c == '}'), ws)).parse(input).is_err(),
        "skip_to_member_boundary_top_level called at a top-level closing brace; caller should handle '}}'"
    );

    let mut iter = input.char_indices().peekable();
    let mut brace_depth = 0usize;
    let mut paren_depth = 0usize;
    let mut bracket_depth = 0usize;
    let mut angle_depth = 0usize; // heuristic for generics in declarations
    let mut in_string = false;
    let mut string_delim: char = '\0';
    let mut in_char = false;
    let mut in_line_comment = false;
    let mut in_block_comment = false;

    while let Some((i, ch)) = iter.next() {
        if in_line_comment {
            if ch == '\n' {
                in_line_comment = false;
            }
            continue;
        }
        if in_block_comment {
            if ch == '*' {
                if let Some((_, '/')) = iter.peek() {
                    in_block_comment = false;
                    iter.next();
                }
            }
            continue;
        }
        if in_string {
            if ch == '\\' {
                iter.next();
                continue;
            } // skip escaped char
            if ch == string_delim {
                in_string = false;
            }
            continue;
        }
        if in_char {
            if ch == '\\' {
                iter.next();
                continue;
            }
            if ch == '\'' {
                in_char = false;
            }
            continue;
        }

        match ch {
            '/' => {
                if let Some((_, next)) = iter.peek() {
                    if *next == '/' {
                        in_line_comment = true;
                        iter.next();
                        continue;
                    }
                    if *next == '*' {
                        in_block_comment = true;
                        iter.next();
                        continue;
                    }
                }
            }
            '"' => {
                in_string = true;
                string_delim = '"';
                continue;
            }
            '\'' => {
                in_char = true;
                continue;
            }
            '{' => {
                brace_depth = brace_depth.saturating_add(1);
            }
            '}' => {
                if brace_depth > 0 {
                    brace_depth -= 1;
                } else {
                    // Top-level closing brace: do not consume it
                    #[cfg(feature = "parser_recovery_trace")]
                    trace!("recovery stop at top-level '}}' after {} chars", i);
                    return &input[i..];
                }
            }
            '(' => {
                paren_depth = paren_depth.saturating_add(1);
            }
            ')' => {
                paren_depth = paren_depth.saturating_sub(1);
            }
            '[' => {
                bracket_depth = bracket_depth.saturating_add(1);
            }
            ']' => {
                bracket_depth = bracket_depth.saturating_sub(1);
            }
            '<' => {
                angle_depth = angle_depth.saturating_add(1);
            }
            '>' => {
                angle_depth = angle_depth.saturating_sub(1);
            }
            ';' => {
                if brace_depth == 0 && paren_depth == 0 && bracket_depth == 0 && angle_depth == 0 {
                    // Consume the semicolon and stop
                    let next = min(i + 1, input.len());
                    #[cfg(feature = "parser_recovery_trace")]
                    trace!("recovery stop at top-level ';' after {} chars", i);
                    return &input[next..];
                }
            }
            _ => {}
        }
    }

    // End of input
    #[cfg(feature = "parser_recovery_trace")]
    trace!("recovery reached EOF without boundary");
    ""
}
use crate::syntax::span::Span;
