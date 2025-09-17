// Common helpers for type declarations (struct, class, interface, record, enum)
// This module provides shared functionality for parsing C# type declarations

use log::trace;

use crate::parser::expressions::declarations::attribute_parser::parse_attribute_lists;
use crate::parser::expressions::declarations::base_types_parser::parse_base_type_list;
use crate::parser::expressions::declarations::modifier_parser::parse_modifiers_for_decl_type;
use crate::parser::expressions::declarations::type_parameter_parser::opt_parse_type_parameter_list;
use crate::parser::identifier_parser::parse_identifier;
use crate::syntax::comment_parser::ws;
use crate::syntax::errors::BResult;
use crate::syntax::nodes::declarations::{attribute::AttributeList, modifier::Modifier};
use crate::syntax::nodes::identifier::Identifier;
use crate::syntax::nodes::types::{Type, TypeParameter};
use crate::syntax::parser_helpers::{bchar, bws, context};
use nom::combinator::cut;

/// Core structure for type declarations (class, struct, interface, record)
/// Contains the common elements shared by all these declaration types
pub struct BaseTypeDeclaration {
    pub attributes: Vec<AttributeList>,
    pub modifiers: Vec<Modifier>,
    pub name: Identifier,
    pub type_parameters: Option<Vec<TypeParameter>>,
    pub base_types: Vec<Type>,
}

/// Parse a type declaration header, handling attributes, modifiers, keyword, name and type parameters
/// Returns the parsed BaseTypeDeclaration and the remaining input
pub fn parse_type_declaration_header<'a>(
    input: &'a str,
    declaration_type: &'static str,
    keyword: &'static str,
) -> BResult<&'a str, BaseTypeDeclaration> {
    // Parse attributes first
    let (input, attribute_lists) = parse_attribute_lists(input)?;

    trace!("parse_type_declaration_header: input = {:?}", input);

    // Try to parse using the declaration helper which handles the keyword and modifiers
    let mut header_parser = crate::parser::declaration_helpers::parse_declaration_header(
        |i| parse_modifiers_for_decl_type(i, declaration_type),
        keyword,
    );

    // Parse the header with improved whitespace handling
    let (remaining, (modifiers, _)) = match header_parser(input) {
        Ok(result) => result,
        Err(err) => {
            trace!(
                "Error parsing declaration header for {}: {:?}",
                declaration_type, err
            );
            return Err(err);
        }
    };

    // Parse the type name
    let (remaining, name) = match context(
        "type name (expected valid identifier)",
        bws(parse_identifier),
    )(remaining)
    {
        Ok(result) => result,
        Err(err) => {
            trace!(
                "Error parsing type name for {}: {:?}",
                declaration_type, err
            );
            return Err(err);
        }
    };

    // Parse type parameters directly - avoid nested Option
    let (remaining, type_parameters) = bws(opt_parse_type_parameter_list)(remaining)?;

    // Parse optional base types (interfaces or base classes)
    let (remaining, base_types) = parse_base_type_list(remaining)?;

    Ok((
        remaining,
        BaseTypeDeclaration {
            attributes: attribute_lists,
            modifiers,
            name,
            type_parameters,
            base_types,
        },
    ))
}

/// Parse the opening brace of a type declaration body
pub fn parse_open_brace(input: &str) -> BResult<&str, ()> {
    let (input, _) = context(
        "opening brace (expected '{' to start type body)",
        bws(bchar('{')),
    )(input)?;
    Ok((input, ()))
}

/// Parse the closing brace of a type declaration body
pub fn parse_close_brace(input: &str) -> BResult<&str, ()> {
    let (input, _) = context(
        "closing brace (expected '}' to end type body)",
        cut(bws(bchar('}'))),
    )(input)?;
    Ok((input, ()))
}

/// Skip whitespace and check if we've reached the end of a body (closing brace)
pub fn at_end_of_body(input: &str) -> bool {
    let (after_ws, _) = ws(input).unwrap_or((input, ""));
    after_ws.trim_start().starts_with('}')
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
pub fn skip_to_member_boundary_top_level(input: &str) -> &str {
    use std::cmp::min;

    // Guardrails: discourage calling when already at a closing brace for the current body.
    // This is not a hard error in release builds, but it helps surface misuse during development.
    let trimmed = input.trim_start();
    debug_assert!(
        !trimmed.starts_with('}'),
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
            if ch == '\n' { in_line_comment = false; }
            continue;
        }
        if in_block_comment {
            if ch == '*' { if let Some((_, '/')) = iter.peek() { in_block_comment = false; iter.next(); } }
            continue;
        }
        if in_string {
            if ch == '\\' { iter.next(); continue; } // skip escaped char
            if ch == string_delim { in_string = false; }
            continue;
        }
        if in_char {
            if ch == '\\' { iter.next(); continue; }
            if ch == '\'' { in_char = false; }
            continue;
        }

        match ch {
            '/' => {
                if let Some((_, next)) = iter.peek() {
                    if *next == '/' { in_line_comment = true; iter.next(); continue; }
                    if *next == '*' { in_block_comment = true; iter.next(); continue; }
                }
            }
            '"' => { in_string = true; string_delim = '"'; continue; }
            '\'' => { in_char = true; continue; }
            '{' => { brace_depth = brace_depth.saturating_add(1); }
            '}' => {
                if brace_depth > 0 { brace_depth -= 1; }
                else {
                    // Top-level closing brace: do not consume it
                    #[cfg(feature = "parser_recovery_trace")]
                    trace!("recovery stop at top-level '}}' after {} chars", i);
                    return &input[i..];
                }
            }
            '(' => { paren_depth = paren_depth.saturating_add(1); }
            ')' => { if paren_depth > 0 { paren_depth -= 1; } }
            '[' => { bracket_depth = bracket_depth.saturating_add(1); }
            ']' => { if bracket_depth > 0 { bracket_depth -= 1; } }
            '<' => { angle_depth = angle_depth.saturating_add(1); }
            '>' => { if angle_depth > 0 { angle_depth -= 1; } }
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

#[cfg(test)]
mod tests {
    use super::skip_to_member_boundary_top_level as recover;

    #[test]
    fn stops_at_top_level_semicolon() {
        let inp = "bad bad bad; int y;";
        let out = recover(inp);
        assert_eq!(out.trim_start(), "int y;");
    }

    #[test]
    fn stops_before_top_level_close_brace() {
        let inp = "bad() // oops\n}";
        let out = recover(inp);
        assert!(out.starts_with('}'));
    }

    #[test]
    fn ignores_semicolons_in_strings_and_comments() {
        let inp = "var s = \"; not boundary }\"; /* } ; */ int z;";
        let out = recover(inp);
        assert!(out.contains("int z;"));
    }

    #[test]
    fn handles_nested_brackets_and_generics() {
        let inp = "var x = new List<Dictionary<string, List<int>>>(42); int y;";
        let out = recover(inp);
        assert_eq!(out.trim_start(), "int y;");
    }
}
