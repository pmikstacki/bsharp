use crate::parser::expressions::primary_expression_parser::parse_expression;
use crate::parser::identifier_parser::{parse_identifier, parse_qualified_name};
use crate::syntax::comment_parser::ws;
use crate::syntax::errors::BResult;
use crate::syntax::nodes::declarations::attribute::{Attribute, AttributeList};
use crate::syntax::nodes::expressions::expression::Expression;
use crate::syntax::parser_helpers::{bchar, bws, context, parse_delimited_list0};
use nom::{
    combinator::{map, opt},
    multi::many0,
    sequence::{terminated, tuple},
};

/// Parses an attribute argument which can be any expression
fn parse_attribute_argument(input: &str) -> BResult<&str, Expression> {
    context(
        "attribute argument (expected valid C# expression)",
        parse_expression,
    )(input)
}

/// Parses a single attribute with optional arguments
/// Example: `[Serializable]` or `[DataMember(Name = "firstName", Order = 1)]`
fn parse_single_attribute(input: &str) -> BResult<&str, Attribute> {
    context(
        "single attribute (expected identifier optionally followed by arguments in parentheses)",
        map(
            tuple((
                bws(parse_identifier),
                opt(parse_delimited_list0::<_, _, _, _, char, Expression, char, char, Expression>(
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
                Attribute { name, arguments }
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
        map(
            tuple((
                // Attribute name (qualified name like System.Reflection.AssemblyVersion)
                parse_qualified_name,
                // Optional argument list
                opt(parse_delimited_list0::<_, _, _, _, char, Expression, char, char, Expression>(
                    bchar('('),
                    parse_expression,
                    bchar(','),
                    bchar(')'),
                    false,
                    true,
                )),
            )),
            |(name_parts, arguments)| {
                // Convert qualified name to single identifier by joining with dots
                let name_str = name_parts
                    .iter()
                    .map(|id| id.name.clone())
                    .collect::<Vec<_>>()
                    .join(".");
                Attribute {
                    name: crate::syntax::nodes::identifier::Identifier { name: name_str },
                    arguments: arguments.unwrap_or_default(),
                }
            },
        ),
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
