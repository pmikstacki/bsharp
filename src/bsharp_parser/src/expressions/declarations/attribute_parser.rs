use crate::expressions::primary_expression_parser::parse_expression;
use crate::identifier_parser::{parse_identifier, parse_qualified_name};
use crate::syntax::comment_parser::ws;
use crate::syntax::errors::BResult;
use crate::syntax::parser_helpers;
use crate::syntax::parser_helpers::{bchar, bws, context, parse_delimited_list0};
use crate::types::type_parser::parse_type_expression;
use nom::character::complete::char as nom_char;
use nom::{
    combinator::{map, opt},
    multi::many0,
    sequence::{terminated, tuple},
};
use syntax::declarations::{Attribute, AttributeList};
use syntax::expressions::Expression;

/// Parses an attribute argument which can be any expression
#[allow(dead_code)]
fn parse_attribute_argument(input: &str) -> BResult<&str, Expression> {
    context(
        "attribute argument (expected valid C# expression)",
        parse_expression,
    )(input)
}

/// Parses a single attribute with optional arguments
/// Example: `[Serializable]` or `[DataMember(Name = "firstName", Order = 1)]`
#[allow(dead_code)]
fn parse_single_attribute(input: &str) -> BResult<&str, Attribute> {
    context(
        "single attribute (expected identifier optionally followed by arguments in parentheses)",
        map(
            tuple((
                bws(parse_identifier),
                opt(parse_delimited_list0::<
                    _,
                    _,
                    _,
                    _,
                    char,
                    Expression,
                    char,
                    char,
                    Expression,
                >(
                    bchar('('),
                    parse_attribute_argument,
                    bchar(','),
                    bchar(')'),
                    false, // trailing commas not allowed in attribute args
                    true,  // cut on close
                )),
            )),
            |(name, opt_args)| {
                let arguments = opt_args.unwrap_or_default();
                Attribute {
                    name,
                    arguments,
                    structured: None,
                }
            },
        ),
    )(input)
}

/// Parses an attribute list enclosed in square brackets
/// Example: `[Serializable, DataContract]`
fn parse_attribute_group(input: &str) -> BResult<&str, AttributeList> {
    context(
        "attribute group (expected '[' followed by comma-separated attributes and ']')",
        map(
            parse_delimited_list0::<_, _, _, _, char, Attribute, char, char, Attribute>(
                bchar('['),
                parse_attribute,
                bchar(','),
                bchar(']'),
                false, // trailing commas not allowed in attribute list
                true,  // cut on close
            ),
            |attributes| AttributeList { attributes },
        ),
    )(input)
}

/// Parses multiple attribute lists that might appear before a declaration
/// Example: `[Serializable] [DataContract]`
pub fn parse_attribute_lists(input: &str) -> BResult<&str, Vec<AttributeList>> {
    context(
        "attribute lists (expected zero or more attribute groups in square brackets)",
        many0(terminated(parse_attribute_group, ws)),
    )(input)
}

// Parse a single attribute: MyAttribute or MyAttribute(arg1, arg2)
pub fn parse_attribute(input: &str) -> BResult<&str, Attribute> {
    context(
        "attribute (expected qualified name optionally followed by arguments in parentheses)",
        |i| {
            // Parse the qualified (dotted) identifier first
            let (rest0, name_parts) = parse_qualified_name(i)?;

            // Optional generic type argument list on the last segment
            let (rest, type_args_opt) = opt(parser_helpers::parse_delimited_list0::<
                _,
                _,
                _,
                _,
                char,
                crate::syntax::nodes::types::Type,
                char,
                char,
                crate::syntax::nodes::types::Type,
            >(
                nom_char('<'),
                parse_type_expression,
                nom_char(','),
                nom_char('>'),
                false,
                true,
            ))(rest0)?;

            // Optional argument list ( ... )
            let (rest_after_args, args_opt) = opt(parse_delimited_list0::<
                _,
                _,
                _,
                _,
                char,
                Expression,
                char,
                char,
                Expression,
            >(
                bchar('('),
                parse_expression,
                bchar(','),
                bchar(')'),
                false,
                true,
            ))(rest)?;

            let name_str = name_parts
                .iter()
                .map(|id| id.name.clone())
                .collect::<Vec<_>>()
                .join(".");
            // Keep name string without generic arguments; use 'structured' to capture generics.

            Ok((
                rest_after_args,
                Attribute {
                    name: crate::syntax::nodes::identifier::Identifier { name: name_str },
                    arguments: args_opt.unwrap_or_default(),
                    structured: Some(
                        crate::syntax::nodes::declarations::attribute::AttributeName {
                            qualifier: name_parts[..name_parts.len().saturating_sub(1)].to_vec(),
                            name: name_parts.last().cloned().unwrap_or_else(|| {
                                crate::syntax::nodes::identifier::Identifier {
                                    name: String::new(),
                                }
                            }),
                            type_arguments: type_args_opt.unwrap_or_default(),
                        },
                    ),
                },
            ))
        },
    )(input)
}

// Parse a single attribute list: [Attr1, Attr2]
pub fn parse_attribute_list(input: &str) -> BResult<&str, AttributeList> {
    context(
        "attribute list (expected '[' followed by comma-separated attributes and ']')",
        map(
            parse_delimited_list0::<_, _, _, _, char, Attribute, char, char, Attribute>(
                bchar('['),
                parse_attribute,
                bchar(','),
                bchar(']'),
                false,
                false, // not committed as hard as others; keep without cut here
            ),
            |attributes| AttributeList { attributes },
        ),
    )(input)
}

// Parse multiple attribute lists: [Attr1] [Attr2] [Attr3, Attr4]
pub fn parse_attribute_lists_new(input: &str) -> BResult<&str, Vec<AttributeList>> {
    context(
        "attribute lists (expected zero or more attribute groups in square brackets)",
        many0(bws(parse_attribute_list)),
    )(input)
}
