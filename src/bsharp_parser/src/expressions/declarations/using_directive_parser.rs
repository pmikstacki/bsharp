use crate::parser::identifier_parser::parse_qualified_name;
use crate::parser::keywords::declaration_keywords::kw_using;
use crate::parser::keywords::modifier_keywords::kw_static;
use crate::syntax::comment_parser::ws;
use crate::syntax::errors::BResult;
use nom::Parser;
use nom::character::complete::char as nom_char;
use nom::combinator::peek;
use nom::sequence::delimited;
use nom_supreme::ParserExt;
use syntax::Identifier;
use syntax::declarations::UsingDirective;

/// Parse a using directive (namespace, alias, or static)
pub fn parse_using_directive(input: Span) -> BResult<UsingDirective> {
    (|input| {
        // 'using' keyword
        let (input, _) = kw_using().context("using keyword").parse(input)?;

        // Optional 'static' keyword branch
        let (input_after_static, opt_static) =
            nom::combinator::opt(delimited(ws, kw_static(), ws)).parse(input)?;
        if opt_static.is_some() {
            let (input_after_name, type_name_parts) =
                delimited(ws, parse_qualified_name, ws).parse(input_after_static)?;
            let type_name_segments: Vec<String> = type_name_parts
                .into_iter()
                .map(|id| match id {
                    Identifier::Simple(s) => s,
                    Identifier::QualifiedIdentifier(v) => v.join("."),
                    Identifier::OperatorOverrideIdentifier(_) => "operator".to_string(),
                })
                .collect();
            let (input_final, _) = delimited(ws, nom_char(';'), ws).parse(input_after_name)?;
            let using_directive = UsingDirective::Static {
                type_name: Identifier::QualifiedIdentifier(type_name_segments),
            };
            return Ok((input_final, using_directive));
        }

        // Otherwise parse a qualified name and decide alias vs namespace using lookahead for '='
        let (input, left_parts) = delimited(ws, parse_qualified_name, ws).parse(input)?;
        let left_segments: Vec<String> = left_parts
            .into_iter()
            .map(|id| match id {
                Identifier::Simple(s) => s,
                Identifier::QualifiedIdentifier(v) => v.join("."),
                Identifier::OperatorOverrideIdentifier(_) => "operator".to_string(),
            })
            .collect();

        if peek(delimited(ws, tok_assign(), ws)).parse(input).is_ok() {
            let (input, _) = delimited(ws, tok_assign(), ws).parse(input)?;
            let (input, right_parts) = delimited(ws, parse_qualified_name, ws).parse(input)?;
            let right_segments: Vec<String> = right_parts
                .into_iter()
                .map(|id| match id {
                    Identifier::Simple(s) => s,
                    Identifier::QualifiedIdentifier(v) => v.join("."),
                    Identifier::OperatorOverrideIdentifier(_) => "operator".to_string(),
                })
                .collect();
            let (input, _) = delimited(ws, nom_char(';'), ws).parse(input)?;
            let using_directive = UsingDirective::Alias {
                alias: Identifier::QualifiedIdentifier(left_segments),
                namespace_or_type: Identifier::QualifiedIdentifier(right_segments),
            };
            Ok((input, using_directive))
        } else {
            let (input, _) = delimited(ws, nom_char(';'), ws).parse(input)?;
            let using_directive = UsingDirective::Namespace {
                namespace: Identifier::QualifiedIdentifier(left_segments),
            };
            Ok((input, using_directive))
        }
    })
    .context("using directive")
    .parse(input)
}
use crate::syntax::span::Span;
use crate::tokens::assignment::tok_assign;
