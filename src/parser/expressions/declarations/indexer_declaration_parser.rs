use crate::syntax::comment_parser::ws;
use nom::{branch::alt, combinator::map};

use crate::parser::expressions::declarations::attribute_parser::parse_attribute_lists;
use crate::parser::expressions::declarations::modifier_parser::parse_modifiers;
use crate::parser::expressions::declarations::type_declaration_parser::convert_attributes;
use crate::parser::expressions::statements::block_statement_parser::parse_block_statement;
use crate::parser::types::type_parser::parse_type_expression;
use crate::syntax::errors::BResult;
use crate::syntax::nodes::declarations::{IndexerAccessorList, IndexerDeclaration};
use crate::syntax::nodes::types::Parameter;
use crate::syntax::parser_helpers::{bchar, bws, context, keyword, parse_delimited_list0};
use nom::combinator::cut;

/// Parse a C# indexer declaration
///
/// Examples:
/// ```csharp
/// public int this[int index] { get; set; }
/// public string this[int row, int col] { get { return _data[row][col]; } set { _data[row][col] = value; } }
/// ```
pub fn parse_indexer_declaration(input: &str) -> BResult<&str, IndexerDeclaration> {
    context(
        "indexer declaration (expected optional attributes, modifiers, type, 'this' keyword, parameters in brackets, and accessor list)",
        |input| {
            // Parse attributes
            let (input, attribute_lists) = parse_attribute_lists(input)?;
            let attributes = convert_attributes(attribute_lists);

            // Parse modifiers (public, private, etc.)
            let (input, modifiers) = parse_modifiers(input)?;

            // Parse the return type
            let (input, ty) = context(
                "indexer return type (expected valid C# type)",
                bws(parse_type_expression),
            )(input)?;

            // Parse the "this" keyword
            let (input, _) = context(
                "this keyword (expected 'this' for indexer declaration)",
                bws(keyword("this")),
            )(input)?;

            // Parse the indexer parameters [type name, ...]
            let (input, parameters) = parse_indexer_parameters(input)?;

            // Parse the accessor list { get; set; } or { get { ... } set { ... } }
            let (input, accessor_list) = parse_indexer_accessor_list(input)?;

            let indexer_declaration = IndexerDeclaration {
                attributes,
                modifiers,
                indexer_type: ty,
                parameters,
                accessor_list,
            };

            Ok((input, indexer_declaration))
        },
    )(input)
}

/// Parse indexer parameters (same as regular parameters but inside square brackets)
fn parse_indexer_parameters(input: &str) -> BResult<&str, Vec<Parameter>> {
    use crate::parser::expressions::declarations::parameter_parser::parse_parameter;

    context(
        "indexer parameters (expected comma-separated list of parameters)",
        parse_delimited_list0::<_, _, _, _, char, Parameter, char, char, Parameter>(
            bchar('['),
            |i| bws(parse_parameter)(i),
            bchar(','),
            bchar(']'),
            false,
            true,
        ),
    )(input)
}

/// Parse the indexer accessor list
fn parse_indexer_accessor_list(input: &str) -> BResult<&str, IndexerAccessorList> {
    context(
        "indexer accessor list (expected '{' followed by get/set accessors and '}')",
        |input| {
            // Parse opening brace
            let (input, _) = context(
                "accessor list opening brace (expected '{' to start accessor list)",
                bws(bchar('{')),
            )(input)?;

            // Parse accessors (get and/or set)
            let (input, (get_accessor, set_accessor)) = parse_accessors(input)?;

            // Parse closing brace
            let (input, _) = context(
                "accessor list closing brace (expected '}' to end accessor list)",
                cut(bws(bchar('}'))),
            )(input)?;

            Ok((
                input,
                IndexerAccessorList {
                    get_accessor,
                    set_accessor,
                },
            ))
        },
    )(input)
}

/// Parse get and/or set accessors
fn parse_accessors(input: &str) -> BResult<&str, (Option<String>, Option<String>)> {
    context(
        "indexer accessors (expected 'get' and/or 'set' accessor declarations)",
        |input| {
            let mut get_accessor = None;
            let mut set_accessor = None;
            let mut current = input;

            // Keep parsing accessors until we hit the closing brace
            while !current.trim_start().starts_with('}') {
                // Skip whitespace and comments
                let (after_ws, _) = ws(current)?;
                current = after_ws;

                // Check if we're at the end
                if current.trim_start().starts_with('}') {
                    break;
                }

                // Try to parse an accessor
                if current.trim_start().starts_with("get") {
                    let (rest, accessor_body) = parse_get_accessor_declaration(current)?;
                    get_accessor = Some(accessor_body);
                    current = rest;
                } else if current.trim_start().starts_with("set") {
                    let (rest, accessor_body) = parse_set_accessor_declaration(current)?;
                    set_accessor = Some(accessor_body);
                    current = rest;
                } else {
                    // Unknown accessor, skip it
                    break;
                }
            }

            Ok((current, (get_accessor, set_accessor)))
        },
    )(input)
}

/// Parse a get accessor declaration
fn parse_get_accessor_declaration(input: &str) -> BResult<&str, String> {
    context(
        "get accessor declaration (expected 'get' followed by body or semicolon)",
        |input| {
            // Parse the get keyword
            let (input, _) = context("get keyword (expected 'get')", bws(keyword("get")))(input)?;

            // Parse the body (either block or semicolon)
            alt((
                // Semicolon (auto-accessor)
                map(
                    context(
                        "get accessor semicolon (expected ';' for auto-accessor)",
                        bws(bchar(';')),
                    ),
                    |_| "".to_string(),
                ),
                // Block body
                map(
                    context(
                        "get accessor body (expected block statement)",
                        parse_block_statement,
                    ),
                    |_| "{ /* get body */ }".to_string(),
                ),
            ))(input)
        },
    )(input)
}

/// Parse a set accessor declaration  
fn parse_set_accessor_declaration(input: &str) -> BResult<&str, String> {
    context(
        "set accessor declaration (expected 'set' followed by body or semicolon)",
        |input| {
            // Parse the set keyword
            let (input, _) = context("set keyword (expected 'set')", bws(keyword("set")))(input)?;

            // Parse the body (either block or semicolon)
            alt((
                // Semicolon (auto-accessor)
                map(
                    context(
                        "set accessor semicolon (expected ';' for auto-accessor)",
                        bws(bchar(';')),
                    ),
                    |_| "".to_string(),
                ),
                // Block body
                map(
                    context(
                        "set accessor body (expected block statement)",
                        parse_block_statement,
                    ),
                    |_| "{ /* set body */ }".to_string(),
                ),
            ))(input)
        },
    )(input)
}
