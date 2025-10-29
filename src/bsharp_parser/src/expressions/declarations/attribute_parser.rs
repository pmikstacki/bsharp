use crate::parser::expressions::primary_expression_parser::parse_expression_spanned;
use crate::parser::identifier_parser::parse_qualified_name;
use crate::parser::types::type_parser::parse_type_expression;
use crate::trivia::comment_parser::ws;
use crate::errors::BResult;
use crate::syntax::list_parser::parse_delimited_list0;
use syntax::span::Span;

use crate::tokens::delimiters::{tok_l_brack, tok_l_paren, tok_r_brack, tok_r_paren};
use crate::tokens::relational::{tok_gt, tok_lt};
use crate::tokens::separators::tok_comma;
use nom::Parser;
use nom::{
    combinator::{map, opt},
    multi::many0,
    sequence::{delimited, terminated},
};
use nom_supreme::ParserExt;
use syntax::declarations::{Attribute, AttributeList};
use syntax::expressions::Expression;
use syntax::types::Type;

// Helper: pretty-print a Type for attribute generic arguments (limited coverage for tests)
fn type_to_string(t: &syntax::types::Type) -> String {
    match t {
        syntax::types::Type::Primitive(p) => p.to_string(),
        syntax::types::Type::Reference(id) => id.to_string(),
        syntax::types::Type::Generic { base, args } => {
            let args_s = args
                .iter()
                .map(type_to_string)
                .collect::<Vec<_>>()
                .join(", ");
            format!("{}<{}>", base, args_s)
        }
        syntax::types::Type::Array { element_type, rank } => {
            let mut s = type_to_string(element_type);
            if *rank == 1 {
                s.push_str("[]");
            } else {
                s.push('[');
                s.push_str(&",".repeat(rank.saturating_sub(1)));
                s.push(']');
            }
            s
        }
        syntax::types::Type::Pointer(inner) => format!("{}*", type_to_string(inner)),
        syntax::types::Type::Nullable(inner) => format!("{}?", type_to_string(inner)),
        _ => format!("{:?}", t),
    }
}

/// Parses an attribute list enclosed in square brackets
/// Example: `[Serializable, DataContract]`
fn parse_attribute_group(input: Span) -> BResult<AttributeList> {
    map(
        parse_delimited_list0::<_, _, _, _, char, char, char, Attribute>(
            |i| delimited(ws, tok_l_brack(), ws).parse(i),
            parse_attribute,
            |i| delimited(ws, tok_comma(), ws).parse(i),
            |i| delimited(ws, tok_r_brack(), ws).parse(i),
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
    (|i| {
        let (rest, lists) = many0(terminated(parse_attribute_group, ws)).parse(i)?;
        // IMPORTANT: Do not rebase spans; keep original base to preserve correct offsets
        Ok((rest, lists))
    })
    .context("attribute  lists")
    .parse(input)
}

// Parse a single attribute: MyAttribute or MyAttribute(arg1, arg2)
pub fn parse_attribute(input: Span) -> BResult<Attribute> {
    (|i| {
        // Parse the qualified (dotted) identifier first
        let (rest0, name_parts) = parse_qualified_name(i)?;

        // Optional generic type argument list on the last segment
        let (rest, type_args_opt) =
            opt(parse_delimited_list0::<_, _, _, _, char, char, char, Type>(
                |i2| delimited(ws, tok_lt(), ws).parse(i2),
                parse_type_expression,
                |i2| delimited(ws, tok_comma(), ws).parse(i2),
                |i2| delimited(ws, tok_gt(), ws).parse(i2),
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
            char,
            char,
            Expression,
        >(
            |i2| delimited(ws, tok_l_paren(), ws).parse(i2),
            |i2| delimited(ws, parse_expression_spanned, ws).map(|s| s.node).parse(i2),
            |i2| delimited(ws, tok_comma(), ws).parse(i2),
            |i2| delimited(ws, tok_r_paren(), ws).parse(i2),
            false,
            true,
        ))
        .parse(rest)?;

        // Build qualified identifier segments for the attribute name, appending generic args to the last segment for display
        let mut name_segments: Vec<String> = name_parts
            .iter()
            .map(|id| match id {
                syntax::Identifier::Simple(s) => s.clone(),
                syntax::Identifier::QualifiedIdentifier(v) => v.join("."),
                syntax::Identifier::OperatorOverrideIdentifier(_) => "operator".to_string(),
            })
            .collect();
        if let Some(type_args) = &type_args_opt {
            if let Some(last) = name_segments.last_mut() {
                let mut appended = String::new();
                appended.push_str(last);
                appended.push('<');
                for (i, t) in type_args.iter().enumerate() {
                    if i > 0 {
                        appended.push_str(", ");
                    }
                    appended.push_str(&type_to_string(t));
                }
                appended.push('>');
                *last = appended;
            }
        }

        // Build final Identifier: Simple if single segment, otherwise QualifiedIdentifier
        let final_name = if name_segments.len() == 1 {
            syntax::identifier::Identifier::Simple(name_segments[0].clone())
        } else {
            syntax::identifier::Identifier::QualifiedIdentifier(name_segments.clone())
        };

        Ok((
            rest_after_args,
            Attribute {
                name: final_name,
                arguments: args_opt.unwrap_or_default(),
                structured: Some(syntax::declarations::attribute::AttributeName {
                    qualifier: name_parts[..name_parts.len().saturating_sub(1)].to_vec(),
                    name: name_parts
                        .last()
                        .cloned()
                        .unwrap_or_else(|| syntax::identifier::Identifier::Simple(String::new())),
                    type_arguments: type_args_opt.unwrap_or_default(),
                }),
            },
        ))
    })
    .context("attribute")
    .parse(input)
}

// Parse a single attribute list: [Attr1, Attr2]
pub fn parse_attribute_list(input: Span) -> BResult<AttributeList> {
    map(
        parse_delimited_list0::<_, _, _, _, char, char, char, Attribute>(
            |i| delimited(ws, tok_l_brack(), ws).parse(i),
            parse_attribute,
            |i| delimited(ws, tok_comma(), ws).parse(i),
            |i| delimited(ws, tok_r_brack(), ws).parse(i),
            false,
            false,
        ),
        |attributes| AttributeList { attributes },
    )
    .context("attribute list")
    .parse(input)
}

// Parse multiple attribute lists: [Attr1] [Attr2] [Attr3, Attr4]
pub fn parse_attribute_lists_new(input: Span) -> BResult<Vec<AttributeList>> {
    many0(|i| delimited(ws, parse_attribute_list, ws).parse(i))
        .context("attribute lists")
        .parse(input)
}
