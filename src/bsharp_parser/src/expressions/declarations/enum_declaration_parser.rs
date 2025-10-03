use crate::parser::expressions::declarations::attribute_parser::parse_attribute_lists;
use crate::parser::expressions::declarations::modifier_parser::parse_modifiers;
use crate::parser::expressions::declarations::type_declaration_helpers::{
    parse_close_brace, parse_open_brace,
};
use crate::parser::expressions::primary_expression_parser::parse_expression;
use crate::parser::identifier_parser::parse_identifier;
use crate::parser::types::type_parser::parse_type_expression;
use crate::syntax::errors::BResult;
use crate::syntax::nodes::declarations::enum_declaration::{EnumDeclaration, EnumMember};
use crate::syntax::parser_helpers::{bchar, bws, context, keyword};
use log::trace;
use nom::{
    combinator::opt,
    multi::separated_list0,
    sequence::{preceded, tuple},
};

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
pub fn parse_enum_declaration(input: &str) -> BResult<&str, EnumDeclaration> {
    context(
        "enum declaration (expected optional attributes, modifiers, 'enum' keyword, name, optional underlying type, and body)",
        |input| {
            trace!("parse_enum_declaration: input = \"{}\"", input);
            // Parse attributes and convert to the expected format
            let (input, attribute_lists) = parse_attribute_lists(input)?;

            // Parse modifiers (public, internal, etc.)
            let (input, modifiers) = parse_modifiers(input)?;

            // Parse "enum" keyword
            let (input, _) =
                context("enum keyword (expected 'enum')", bws(keyword("enum")))(input)?;

            // Parse enum name
            let (input, name) = context(
                "enum name (expected valid identifier)",
                bws(parse_identifier),
            )(input)?;

            // Parse optional underlying type (: byte, : int, etc.)
            let (input, underlying_type) = opt(context(
                "enum underlying type (expected ':' followed by type)",
                tuple((bws(bchar(':')), bws(parse_type_expression))),
            ))(input)?;

            // Extract the Type from the tuple, if present
            let underlying_type = underlying_type.map(|(_, ty)| ty);

            // Parse the enum body
            let (input, _) = parse_open_brace(input)?;

            // Parse enum members
            let (input, members) = parse_enum_members(input)?;

            // Parse the closing brace
            let (input, _) = parse_close_brace(input)?;

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
        },
    )(input)
}

/// Parse a list of enum members
/// Example: "None = 0, Monday = 1, Tuesday = 2"
fn parse_enum_members<'a>(input: &'a str) -> BResult<&'a str, Vec<EnumMember>> {
    context(
        "enum members (expected comma-separated list of enum member declarations)",
        |input: &'a str| {
            trace!(
                "[DEBUG] parse_enum_members: input = {:?}",
                &input[..std::cmp::min(100, input.len())]
            );
            // Parse a comma-separated list of enum members
            // The list can be empty or have a trailing comma
            let result = separated_list0(bws(bchar(',')), parse_enum_member)(input);

            match &result {
                Ok((rest, members)) => {
                    trace!(
                        "[DEBUG] parse_enum_members: SUCCESS, {} members found. Remaining: {:?}",
                        members.len(),
                        &rest[..std::cmp::min(50, rest.len())]
                    );
                }
                Err(e) => {
                    trace!("[DEBUG] parse_enum_members: FAILED: {:?}", e);
                }
            }
            result
        },
    )(input)
}

/// Parse a single enum member
/// Example: "Monday = 1" or just "Monday"
fn parse_enum_member<'a>(input: &'a str) -> BResult<&'a str, EnumMember> {
    context(
        "enum member (expected optional attributes, identifier, and optional value assignment)",
        |input: &'a str| {
            trace!(
                "[DEBUG] parse_enum_member: input = {:?}",
                &input[..std::cmp::min(100, input.len())]
            );
            // Parse attributes for enum member
            let (input_after_attributes, attribute_lists) = parse_attribute_lists(input)?;
            trace!("[DEBUG] parse_enum_member: parsed attributes");

            // Parse the member name
            let (input_after_name_parse, name) = context(
                "enum member name (expected valid identifier)",
                bws(parse_identifier),
            )(input_after_attributes)?;
            trace!("[DEBUG] parse_enum_member: parsed name = {:?}", name.name);

            // Parse optional value assignment (e.g., "= 1" or "= Monday | Tuesday")
            // This should be applied to the input REMAINING AFTER THE NAME has been parsed.
            let (final_input, value) = opt(preceded(
                context(
                    "enum value assignment (expected '=' followed by expression)",
                    bws(bchar('=')),
                ),
                bws(parse_expression),
            ))(input_after_name_parse)?; // Apply to input_after_name_parse

            trace!(
                "[DEBUG] parse_enum_member: parsed value = {:?}, final_input for member: {:?}",
                value.is_some(),
                &final_input[..std::cmp::min(50, final_input.len())]
            );

            Ok((
                final_input,
                EnumMember {
                    // Use final_input as the remainder
                    attributes: attribute_lists,
                    name,
                    value,
                },
            ))
        },
    )(input)
}
