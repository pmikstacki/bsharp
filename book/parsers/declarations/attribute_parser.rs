use crate::parser::errors::BResult;
use crate::parser::nodes::declarations::attribute::{Attribute, AttributeList};
use crate::parser::nodes::expressions::expression::Expression;
use crate::parser::parser_helpers::{bchar, bs_context, bws, nom_to_bs};
use crate::parsers::expressions::expression_parser::parse_expression;
use crate::parsers::identifier_parser::{parse_identifier, parse_qualified_name};
use nom::{
    character::complete::{char as nom_char, multispace0},
    combinator::{map, opt},
    multi::{many0, separated_list0},
    sequence::{delimited, terminated, tuple},
};

/// Parses an attribute argument which can be any expression
fn parse_attribute_argument(input: &str) -> BResult<&str, Expression> {
    parse_expression(input)
}

/// Parses a single attribute with optional arguments
/// Example: `[Serializable]` or `[DataMember(Name = "firstName", Order = 1)]`
fn parse_single_attribute(input: &str) -> BResult<&str, Attribute> {
    // Parse the attribute name
    let (input, name) = bws(nom_to_bs(parse_identifier))(input)?;
    
    // Parse optional arguments in parentheses
    let (input, opt_args) = opt(delimited(
        bws(nom_to_bs(nom_char::<&str, nom::error::Error<&str>>('('))),
        separated_list0(
            bws(nom_to_bs(nom_char::<&str, nom::error::Error<&str>>(','))),
            bws(nom_to_bs(parse_attribute_argument))
        ),
        bws(nom_to_bs(nom_char::<&str, nom::error::Error<&str>>(')')))
    ))(input)?;
    
    let arguments = opt_args.unwrap_or_default();
    
    Ok((input, Attribute { name, arguments }))
}

/// Parses an attribute list enclosed in square brackets
/// Example: `[Serializable, DataContract]`
fn parse_attribute_group(input: &str) -> BResult<&str, AttributeList> {
    let (input, attributes) = delimited(
        bws(nom_to_bs(nom_char::<&str, nom::error::Error<&str>>('['))),
        separated_list0(
            bws(nom_to_bs(nom_char::<&str, nom::error::Error<&str>>(','))),
            bws(nom_to_bs(parse_single_attribute))
        ),
        bws(nom_to_bs(nom_char::<&str, nom::error::Error<&str>>(']')))
    )(input)?;
    
    Ok((input, AttributeList { attributes }))
}

/// Parses multiple attribute lists that might appear before a declaration
/// Example: `[Serializable] [DataContract]`
pub fn parse_attribute_lists(input: &str) -> BResult<&str, Vec<AttributeList>> {
    // Use terminated to ensure we don't consume trailing whitespace after the attributes
    // This ensures the rest of the parsers (class, interface, etc.) get any whitespace before them
    many0(terminated(nom_to_bs(parse_attribute_group), multispace0))(input)
}

// Parse a single attribute: MyAttribute or MyAttribute(arg1, arg2)
pub fn parse_attribute(input: &str) -> BResult<&str, Attribute> {
    bs_context(
        "attribute",
        map(
            tuple((
                // Attribute name (qualified name like System.Reflection.AssemblyVersion)
                parse_qualified_name,
                // Optional argument list
                opt(delimited(
                    bws(bchar('(')),
                    separated_list0(bws(bchar(',')), bws(parse_expression)),
                    bws(bchar(')')),
                )),
            )),
            |(name_parts, arguments)| {
                // Convert qualified name to single identifier by joining with dots
                let name_str = name_parts.iter().map(|id| id.name.clone()).collect::<Vec<_>>().join(".");
                Attribute {
                    name: crate::parser::nodes::identifier::Identifier { name: name_str },
                    arguments: arguments.unwrap_or_default(),
                }
            },
        ),
    )(input)
}

// Parse a single attribute list: [Attr1, Attr2]
pub fn parse_attribute_list(input: &str) -> BResult<&str, AttributeList> {
    bs_context(
        "attribute list",
        map(
            delimited(
                bws(bchar('[')),
                separated_list0(bws(bchar(',')), bws(parse_attribute)),
                bws(bchar(']')),
            ),
            |attributes| AttributeList { attributes },
        ),
    )(input)
}

// Parse multiple attribute lists: [Attr1] [Attr2] [Attr3, Attr4]
pub fn parse_attribute_lists_new(input: &str) -> BResult<&str, Vec<AttributeList>> {
    bs_context(
        "attribute lists",
        many0(bws(parse_attribute_list)),
    )(input)
}
