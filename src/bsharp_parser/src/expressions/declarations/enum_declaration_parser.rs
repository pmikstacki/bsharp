use crate::parser::expressions::declarations::attribute_parser::parse_attribute_lists;
use crate::parser::expressions::declarations::modifier_parser::parse_modifiers;
use crate::parser::expressions::declarations::type_declaration_helpers::{
    parse_close_brace, parse_open_brace,
};
use crate::parser::expressions::primary_expression_parser::parse_expression;
use crate::parser::identifier_parser::parse_identifier;
use crate::parser::types::type_parser::parse_type_expression;
use crate::syntax::errors::BResult;
use crate::syntax::span::Span;

use crate::syntax::comment_parser::ws;
use log::trace;
use nom::sequence::delimited;
use nom::Parser;
use nom::{
    combinator::opt,
    multi::separated_list0,
    sequence::preceded,
};
use nom_supreme::ParserExt;
use syntax::declarations::enum_declaration::EnumMember;
pub use syntax::declarations::*;
pub use syntax::expressions::expression::*;
pub use syntax::statements::statement::*;
pub use syntax::trivia::*;
use crate::keywords::declaration_keywords::kw_enum;
use crate::tokens::assignment::tok_assign;
use crate::tokens::separators::{tok_colon, tok_comma};

/// Parse a C# enum declaration.
///
/// Supports enum declarations in several formats:
///
/// ```csharp
/// enum Days { Monday, Tuesday, Wednesday }
/// enum ErrorCode { Success = 0, NotFound = 404 }
/// enum FilePermissions : int { Read = 1, Write = 2, Execute = 4 }
/// [Flags]
/// enum Weekend : byte {
///     Saturday = 64,
///     Sunday = 128,
///     Weekend = Saturday | Sunday
/// }
/// ```
pub fn parse_enum_declaration(input: Span) -> BResult<EnumDeclaration> {
    trace!("parse_enum_declaration: input = \"{}\"", input);
    // Parse attributes and convert to the expected format
    let (input, attribute_lists) = parse_attribute_lists(input.into())?;

    // Parse modifiers (public, internal, etc.)
    let (input, modifiers) = parse_modifiers(input.into())?;

    // Parse "enum" keyword
    let (input, _) = delimited(ws, kw_enum(), ws)
        .context("enum keyword")
        .parse(input.into())?;

    // Parse enum name
    let (input, name) = delimited(ws, parse_identifier, ws)
        .context("enum name")
        .parse(input.into())?;

    // Parse optional underlying type (: byte, : int, etc.)
    let (input, underlying_type) = opt(
        (
            delimited(ws, tok_colon(), ws),
            delimited(ws, parse_type_expression, ws),
        )
            .map(|(_, ty)| ty)
            .context("enum underlying type"),
    )
        .parse(input.into())?;

    // Parse the enum body
    let (input, _) = parse_open_brace(input.into())?;

    // Parse enum members
    let (input, members) = parse_enum_members(input.into())?;

    // Parse the closing brace
    let (input, _) = parse_close_brace(input.into())?;

    Ok((
        input,
        EnumDeclaration {
            attributes: attribute_lists,
            modifiers,
            name,
            underlying_type,
            enum_members: members,
        },
    ))
}

/// Parse a list of enum members
/// Example: "None = 0, Monday = 1, Tuesday = 2"
fn parse_enum_members(input: Span) -> BResult<Vec<EnumMember>> {
    trace!("[DEBUG] parse_enum_members: input = {:?}", input);
    separated_list0(
        |i| delimited(ws, tok_comma(), ws).parse(i),
        |i| delimited(ws, parse_enum_member, ws).parse(i),
    )
        .context("enum members")
        .parse(input.into())
}

/// Parse a single enum member
/// Example: "Monday = 1" or just "Monday"
fn parse_enum_member(input: Span) -> BResult<EnumMember> {
    trace!("[DEBUG] parse_enum_member: input = {:?}", input);
    // Parse attributes for enum member
    let (input, attribute_lists) = parse_attribute_lists(input.into())?;
    trace!("[DEBUG] parse_enum_member: parsed attributes");

    // Parse the member name
    let (input, name) = delimited(ws, parse_identifier, ws)
        .context("enum member name")
        .parse(input.into())?;
    trace!("[DEBUG] parse_enum_member: parsed name = {:?}", name);

    // Parse optional value assignment
    let (input, value) = opt(preceded(
        delimited(ws, tok_assign(), ws)
            .context("enum value assignment"),
        delimited(ws, parse_expression, ws),
    ))
        .parse(input.into())?;

    Ok((
        input,
        EnumMember {
            attributes: attribute_lists,
            name,
            value,
        },
    ))
}

