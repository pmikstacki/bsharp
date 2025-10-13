use crate::parser::identifier_parser::parse_qualified_name;
use crate::parser::keywords::declaration_keywords::kw_using;
use crate::parser::keywords::modifier_keywords::{kw_static, peek_static};
use crate::syntax::errors::BResult;
use syntax::declarations::UsingDirective;
use syntax::Identifier;
use crate::syntax::comment_parser::ws;
use nom::character::complete::char as nom_char;
use nom::combinator::peek;
use nom::sequence::delimited;
use nom::Parser;
use nom_supreme::ParserExt;

/// Parse a using directive (namespace, alias, or static)
pub fn parse_using_directive<'a>(input: Span<'a>) -> BResult<'a, UsingDirective> {
    (|input| {
        // 'using' keyword
        let (input, _) = kw_using().context("using keyword").parse(input)?;

        // Try static using first: 'using static TypeName;'
        if peek_static().parse(input).is_ok() {
            let (input, _) = delimited(ws, kw_static(), ws).parse(input)?;
            let (input, type_name_parts) = delimited(ws, parse_qualified_name, ws).parse(input)?;
            let type_name_str = type_name_parts
                .iter()
                .map(|id| id.name.clone())
                .collect::<Vec<_>>()
                .join(".");
            let (input, _) = delimited(ws, nom_char(';'), ws).parse(input)?;
            let using_directive = UsingDirective::Static {
                type_name: Identifier { name: type_name_str },
            };
            return Ok((input, using_directive));
        }

        // Otherwise parse a qualified name and decide alias vs namespace using lookahead for '='
        let (input, left_parts) = delimited(ws, parse_qualified_name, ws).parse(input)?;
        let left_str = left_parts
            .iter()
            .map(|id| id.name.clone())
            .collect::<Vec<_>>()
            .join(".");

        if peek(delimited(ws, nom_char('='), ws)).parse(input).is_ok() {
            let (input, _) = delimited(ws, nom_char('='), ws).parse(input)?;
            let (input, right_parts) = delimited(ws, parse_qualified_name, ws).parse(input)?;
            let right_str = right_parts
                .iter()
                .map(|id| id.name.clone())
                .collect::<Vec<_>>()
                .join(".");
            let (input, _) = delimited(ws, nom_char(';'), ws).parse(input)?;
            let using_directive = UsingDirective::Alias {
                alias: Identifier { name: left_str },
                namespace_or_type: Identifier { name: right_str },
            };
            Ok((input, using_directive))
        } else {
            let (input, _) = delimited(ws, nom_char(';'), ws).parse(input)?;
            let using_directive = UsingDirective::Namespace {
                namespace: Identifier { name: left_str },
            };
            Ok((input, using_directive))
        }
    })
    .context("using directive")
    .parse(input)
}
use crate::syntax::span::Span;
