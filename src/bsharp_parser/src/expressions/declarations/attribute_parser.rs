use crate::parser::expressions::primary_expression_parser::parse_expression;
use crate::parser::identifier_parser::parse_qualified_name;
use crate::syntax::comment_parser::ws;
use crate::syntax::errors::BResult;
use crate::syntax::list_parser;
use crate::parser::types::type_parser::parse_type_expression;
use nom::character::complete::{satisfy, char as nom_char};
use nom::{
    combinator::{map, opt},
    multi::many0,
    sequence::{delimited, terminated},
};
use nom::Parser;
use nom_supreme::ParserExt;
use syntax::declarations::{Attribute, AttributeList};
use syntax::expressions::Expression;
use syntax::types::Type;
use crate::syntax::list_parser::parse_delimited_list0;
use crate::syntax::span::Span;

/// Parses an attribute list enclosed in square brackets
/// Example: `[Serializable, DataContract]`
fn parse_attribute_group(input: Span) -> BResult<AttributeList> {
    map(
        parse_delimited_list0::<_, _, _, _, char, Attribute, char, char, Attribute>(
            |i| delimited(ws, satisfy(|c| c == '['), ws).parse(i),
            parse_attribute,
            |i| delimited(ws, satisfy(|c| c == ','), ws).parse(i),
            |i| delimited(ws, satisfy(|c| c == ']'), ws).parse(i),
            false,
            true,
        ),
        |attributes| AttributeList { attributes },
    )
    .context("attribute group")
    .parse(input)
}

/// Parses multiple attribute lists that might appear before a declaration
/// Example: `[Serializable] [DataContract]`
pub fn parse_attribute_lists(input: Span) -> BResult<Vec<AttributeList>> {
    many0(terminated(parse_attribute_group, ws))
        .context("attribute  lists")
        .parse(input)
}

// Parse a single attribute: MyAttribute or MyAttribute(arg1, arg2)
pub fn parse_attribute(input: Span) -> BResult<Attribute> {
    (|i| {
            // Parse the qualified (dotted) identifier first
            let (rest0, name_parts) = parse_qualified_name(i)?;

            // Optional generic type argument list on the last segment
            let (rest, type_args_opt) = opt(list_parser::parse_delimited_list0::<
                _,
                _,
                _,
                _,
                char,
                Type,
                char,
                char,
                Type,
            >(
                |i2| delimited(ws, satisfy(|c| c == '<'), ws).parse(i2),
                parse_type_expression,
                |i2| delimited(ws, satisfy(|c| c == ','), ws).parse(i2),
                |i2| delimited(ws, satisfy(|c| c == '>'), ws).parse(i2),
                false,
                true,
            ))
            .parse(rest0)?;

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
                |i2| delimited(ws, satisfy(|c| c == '('), ws).parse(i2),
                parse_expression,
                |i2| delimited(ws, satisfy(|c| c == ','), ws).parse(i2),
                |i2| delimited(ws, satisfy(|c| c == ')'), ws).parse(i2),
                false,
                true,
            ))
            .parse(rest)?;

            let name_str = name_parts
                .iter()
                .map(|id| id.name.clone())
                .collect::<Vec<_>>()
                .join(".");
            // Keep name string without generic arguments; use 'structured' to capture generics.

            Ok((
                rest_after_args,
                Attribute {
                    name: crate::syntax::identifier::Identifier { name: name_str },
                    arguments: args_opt.unwrap_or_default(),
                    structured: Some(crate::syntax::declarations::attribute::AttributeName {
                        qualifier: name_parts[..name_parts.len().saturating_sub(1)].to_vec(),
                        name: name_parts.last().cloned().unwrap_or_else(|| {
                            crate::syntax::identifier::Identifier {
                                name: String::new(),
                            }
                        }),
                        type_arguments: type_args_opt.unwrap_or_default(),
                    }),
                },
            ))
        })
        .context("attribute")
        .parse(input)
}

// Parse a single attribute list: [Attr1, Attr2]
pub fn parse_attribute_list<'a>(input: Span<'a>) -> BResult<'a, AttributeList> {
    map(
        parse_delimited_list0::<_, _, _, _, char, Attribute, char, char, Attribute>(
            |i| delimited(ws, nom_char('['), ws).parse(i),
            parse_attribute,
            |i| delimited(ws, nom_char(','), ws).parse(i),
            |i| delimited(ws, nom_char(']'), ws).parse(i),
            false,
            false,
        ),
        |attributes| AttributeList { attributes },
    )
    .context("attribute list")
    .parse(input)
}

// Parse multiple attribute lists: [Attr1] [Attr2] [Attr3, Attr4]
pub fn parse_attribute_lists_new<'a>(input: Span<'a>) -> BResult<'a, Vec<AttributeList>> {
    many0(|i| delimited(ws, parse_attribute_list, ws).parse(i))
        .context("attribute lists")
        .parse(input)
}
