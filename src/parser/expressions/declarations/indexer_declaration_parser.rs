use nom::{branch::alt, combinator::map};
use nom::multi::many0;

use crate::parser::expressions::declarations::attribute_parser::parse_attribute_lists;
use crate::parser::expressions::declarations::modifier_parser::parse_modifiers;
use crate::parser::expressions::declarations::type_declaration_parser::convert_attributes;
use crate::parser::expressions::statements::block_statement_parser::parse_block_statement;
use crate::parser::types::type_parser::parse_type_expression;
use crate::syntax::errors::BResult;
use crate::syntax::nodes::declarations::{IndexerAccessorList, IndexerAccessor, IndexerDeclaration};
use crate::syntax::nodes::types::Parameter;
use crate::syntax::parser_helpers::{bchar, bws, context, parse_delimited_list0};
use nom::combinator::{cut, peek};
use crate::parser::keywords::accessor_keywords::{kw_get, kw_set};
use crate::parser::keywords::contextual_misc_keywords::kw_this;

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
                bws(kw_this()),
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
                IndexerAccessorList { get_accessor, set_accessor },
            ))
        },
    )(input)
}

/// Parse get and/or set accessors
fn parse_accessors(input: &str) -> BResult<&str, (Option<IndexerAccessor>, Option<IndexerAccessor>)> {
    context(
        "indexer accessors (expected 'get' and/or 'set' accessor declarations)",
        |input| {
            // Two accessor parsers with lookahead over optional attrs/modifiers
            let get_branch = |i| {
                let (i, _) = peek(|ii| {
                    let (ii, _) = bws(parse_attribute_lists)(ii)?;
                    let (ii, _) = bws(parse_modifiers)(ii)?;
                    let (ii, _) = bws(kw_get())(ii)?;
                    Ok((ii, ()))
                })(i)?;
                parse_get_accessor_declaration(i)
            };
            let set_branch = |i| {
                let (i, _) = peek(|ii| {
                    let (ii, _) = bws(parse_attribute_lists)(ii)?;
                    let (ii, _) = bws(parse_modifiers)(ii)?;
                    let (ii, _) = bws(kw_set())(ii)?;
                    Ok((ii, ()))
                })(i)?;
                parse_set_accessor_declaration(i)
            };

            let one_accessor = |i| alt((map(get_branch, |a| (true, a)), map(set_branch, |a| (false, a))))(i);

            let (cur, pairs) = many0(|i| one_accessor(i))(input)?;
            let mut get_accessor: Option<IndexerAccessor> = None;
            let mut set_accessor: Option<IndexerAccessor> = None;
            for (is_get, accessor) in pairs {
                if is_get { get_accessor = Some(accessor); } else { set_accessor = Some(accessor); }
            }

            Ok((cur, (get_accessor, set_accessor)))
        },
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::syntax::nodes::statements::statement::Statement;

    #[test]
    fn parses_get_set_accessors_in_any_order() {
        let src = " {  get; /*c*/ set { } }";
        let (rest, list) = parse_indexer_accessor_list(src).expect("parse");
        assert!(rest.is_empty() || rest.trim().is_empty());
        // get; is present with no body
        assert!(list.get_accessor.is_some());
        assert!(list.get_accessor.as_ref().unwrap().body.is_none());
        // set { } has a block body -> Some(Block)
        assert!(matches!(list.set_accessor.as_ref().unwrap().body, Some(Statement::Block(_))));
    }

    #[test]
    fn stops_before_close_brace_without_consuming() {
        let src = " get; } tail";
        let (rest, (g, s)) = parse_accessors(src).expect("parse");
        // get; present with no body
        assert!(g.is_some());
        assert!(g.as_ref().unwrap().body.is_none());
        assert!(s.is_none());
        assert!(rest.trim_start().starts_with('}'));
    }
}

/// Parse a get accessor declaration
fn parse_get_accessor_declaration(input: &str) -> BResult<&str, IndexerAccessor> {
    context(
        "get accessor declaration (expected 'get' followed by body or semicolon)",
        |input| {
            // Optional attribute lists and modifiers
            let (input, attribute_lists) = bws(parse_attribute_lists)(input)?;
            let attributes = convert_attributes(attribute_lists);
            let (input, modifiers) = bws(parse_modifiers)(input)?;

            // Parse the get keyword
            let (input, _) = context("get keyword (expected 'get')", bws(kw_get()))(input)?;

            // Parse the body (either block or semicolon)
            let (input, body) = alt((
                // Semicolon (auto-accessor)
                map(
                    context(
                        "get accessor semicolon (expected ';' for auto-accessor)",
                        bws(bchar(';')),
                    ),
                    |_| None,
                ),
                // Block body
                map(
                    context(
                        "get accessor body (expected block statement)",
                        parse_block_statement,
                    ),
                    |blk| Some(blk),
                ),
            ))(input)?;

            Ok((input, IndexerAccessor { modifiers, attributes, body }))
        },
    )(input)
}

/// Parse a set accessor declaration  
fn parse_set_accessor_declaration(input: &str) -> BResult<&str, IndexerAccessor> {
    context(
        "set accessor declaration (expected 'set' followed by body or semicolon)",
        |input| {
            // Optional attribute lists and modifiers
            let (input, attribute_lists) = bws(parse_attribute_lists)(input)?;
            let attributes = convert_attributes(attribute_lists);
            let (input, modifiers) = bws(parse_modifiers)(input)?;

            // Parse the set keyword
            let (input, _) = context("set keyword (expected 'set')", bws(kw_set()))(input)?;

            // Parse the body (either block or semicolon)
            let (input, body) = alt((
                // Semicolon (auto-accessor)
                map(
                    context(
                        "set accessor semicolon (expected ';' for auto-accessor)",
                        bws(bchar(';')),
                    ),
                    |_| None,
                ),
                // Block body
                map(
                    context(
                        "set accessor body (expected block statement)",
                        parse_block_statement,
                    ),
                    |blk| Some(blk),
                ),
            ))(input)?;

            Ok((input, IndexerAccessor { modifiers, attributes, body }))
        },
    )(input)
}
